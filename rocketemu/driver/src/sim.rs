use crate::dpi::get_t;

use clap::{arg, Parser};
use std::collections::HashMap;
use std::os::unix::fs::FileExt;
use std::{
  fs,
  path::{Path, PathBuf},
};
use tracing::{debug, error, info, trace};

use anyhow::Context;
use elf::abi::STT_FUNC;
use elf::{
  abi::{EM_RISCV, ET_EXEC, PT_LOAD},
  endian::LittleEndian,
  ElfStream,
};

pub(crate) struct AxiReadPayload {
  pub(crate) data: Vec<u8>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SimulationArgs {
  /// Path to the ELF file
  #[arg(long)]
  pub elf_file: PathBuf,

  /// Path to the log file
  #[arg(long)]
  pub log_file: Option<PathBuf>,

  /// Log level: trace, debug, info, warn, error
  #[arg(long, default_value = "info")]
  pub log_level: String,

  /// The timeout value
  #[arg(long, default_value_t = 1_0000)]
  pub timeout: u64,
}

// FIXME: fix FunctionSym
#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionSym {
  #[allow(dead_code)]
  pub(crate) name: String,
  #[allow(dead_code)]
  pub(crate) info: u8,
}
pub type FunctionSymTab = HashMap<u64, FunctionSym>;

// NOTE: make it configurable from cmd line?
const SIM_MEM_SIZE: usize = 1usize << 32;

#[derive(Debug)]
pub struct Simulator {
  pub(crate) mem: Vec<u8>,
  #[allow(dead_code)]
  pub(crate) fn_sym_tab: FunctionSymTab,
  pub(crate) dlen: u32,
  pub(crate) timeout: u64,
}

pub static WATCHDOG_CONTINUE: u8 = 0;
pub static WATCHDOG_TIMEOUT: u8 = 1;

impl Simulator {
  pub fn new(args: SimulationArgs) -> Self {
    let log_level: tracing::Level = args.log_level.parse().expect("fail to parse LOG level");
    let global_logger = tracing_subscriber::FmtSubscriber::builder()
      .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
      .with_max_level(log_level)
      .without_time()
      .with_target(false)
      .with_ansi(true)
      .compact()
      .finish();
    tracing::subscriber::set_global_default(global_logger)
      .expect("internal error: fail to setup log subscriber");

    // FIXME: pass e_entry to rocket
    let (_FIXME_e_entry, mem, fn_sym_tab) =
      Self::load_elf(&args.elf_file).expect("fail creating simulator");

    Self {
      mem,
      fn_sym_tab,
      timeout: args.timeout,
      dlen: option_env!("DESIGN_DLEN")
        .map(|dlen| dlen.parse().expect("fail to parse dlen into u32 digit"))
        .unwrap_or(256),
    }
  }

  // FIXME: In current implementation, all the ELF sections are read without considering bytes order.
  // We might want to take care of those information with lenntoho to convert them into host byte.
  // The *elf* crate hopefully will handle this for us, but I don't do further investigation yet. (assign to @Avimitin)
  pub fn load_elf(path: &Path) -> anyhow::Result<(u64, Vec<u8>, FunctionSymTab)> {
    let file = fs::File::open(path).with_context(|| "reading ELF file")?;
    let mut elf: ElfStream<LittleEndian, _> =
      ElfStream::open_stream(&file).with_context(|| "parsing ELF file")?;

    if elf.ehdr.e_machine != EM_RISCV {
      anyhow::bail!("ELF is not in RISC-V");
    }

    if elf.ehdr.e_type != ET_EXEC {
      anyhow::bail!("ELF is not an executable");
    }

    if elf.ehdr.e_phnum == 0 {
      anyhow::bail!("ELF has zero size program header");
    }

    debug!("ELF entry: 0x{:x}", elf.ehdr.e_entry);
    // FIXME:
    // 1. If we use reduce map, collecting spartial memory into a whole big one,
    //    instead of manipulating mutable memory, does it affect runtime overhead?
    //    Does rustc help us optimize this operation?
    // 2. The default ProgramHeader use u64 for Elf32_phdr and Elf64_phdr, can we optimize this or
    //    just let it go.
    let mut mem: Vec<u8> = vec![0; SIM_MEM_SIZE];
    elf.segments().iter().filter(|phdr| phdr.p_type == PT_LOAD).for_each(|phdr| {
      let vaddr: usize = phdr.p_vaddr.try_into().expect("fail converting vaddr(u64) to usize");
      let filesz: usize = phdr.p_filesz.try_into().expect("fail converting p_filesz(u64) to usize");
      debug!(
        "Read loadable segments 0x{:x}..0x{:x} to memory 0x{:x}",
        phdr.p_offset,
        phdr.p_offset + filesz as u64,
        vaddr
      );
      // Load file start from offset into given mem slice
      // The `offset` of the read_at method is relative to the start of the file and thus independent from the current cursor.
      file.read_at(&mut mem[vaddr..vaddr + filesz], phdr.p_offset).unwrap_or_else(|err| {
        panic!(
          "fail reading ELF into mem with vaddr={}, filesz={}, offset={}. Error detail: {}",
          vaddr, filesz, phdr.p_offset, err
        )
      });
    });

    // FIXME: now the symbol table doesn't contain any function value
    let mut fn_sym_tab = FunctionSymTab::new();
    let symbol_table =
      elf.symbol_table().with_context(|| "reading symbol table(SHT_SYMTAB) from ELF")?;
    if let Some((parsed_table, string_table)) = symbol_table {
      parsed_table
        .iter()
        // st_symtype = symbol.st_info & 0xf (But why masking here?)
        .filter(|sym| sym.st_symtype() == STT_FUNC)
        .for_each(|sym| {
          let name = string_table
            .get(sym.st_name as usize)
            .unwrap_or_else(|_| panic!("fail to get name at st_name={}", sym.st_name));
          fn_sym_tab.insert(
            sym.st_value,
            FunctionSym { name: name.to_string(), info: sym.st_symtype() },
          );
        });
    } else {
      debug!("load_elf: symtab not found");
    };

    Ok((elf.ehdr.e_entry, mem, fn_sym_tab))
  }

  fn write_mem(&mut self, addr: u32, alignment_bytes: u32, masks: &[bool], data: &[u8]) {
    // early return with strobe 0 write
    if !masks.iter().any(|&x| x) {
      return;
    }
    let size = data.len() as u32;
    debug!("write mem: size={size}, addr={addr:#x}");

    assert!(
      (addr % size == 0 || addr % alignment_bytes == 0) && size >= alignment_bytes,
      "unaligned write access addr={addr} size={size}bytes dlen={alignment_bytes}bytes"
    );

    masks.iter().enumerate().filter(|(_, &m)| m).for_each(|(i, _)| {
      self.mem[addr as usize + i] = data[i];
    });
  }

  pub fn axi_write(&mut self, addr: u32, strobe: &[bool], data: &[u8]) {
    // panic on misalign mask and data
    assert_eq!(
      strobe.len(),
      data.len(),
      "write_mem: strobe size is not equal to data size"
    );
    self.write_mem(addr, self.dlen / 8, strobe, data);
  }

  fn read_mem(&mut self, addr: u32, size: u32, alignment_bytes: u32) -> Vec<u8> {
    assert!(
      addr % size == 0 || addr % alignment_bytes == 0,
      "unaligned access addr={addr} size={size}bytes dlen={alignment_bytes}bytes"
    );
    let residue_addr = addr % alignment_bytes;
    let aligned_addr = addr - residue_addr;
    if size < alignment_bytes {
      // narrow
      (0..alignment_bytes)
        .map(|i| {
          let i_addr = aligned_addr + i;
          if addr <= i_addr && i_addr < addr + size {
            self.mem[i_addr as usize]
          } else {
            0
          }
        })
        .collect()
    } else {
      // normal
      (0..size).map(|i| self.mem[(addr + i) as usize]).collect()
    }
  }

  pub fn axi_read_instruction(&mut self, addr: u32, arsize: u64) -> AxiReadPayload {
    let size = 1 << arsize;
    assert!(size <= 4);
    let data = self.read_mem(addr, size, 4);
    let data_hex = hex::encode(&data);
    info!(
      "[{}] axi_read_indexed (addr={addr:#x}, size={size}, data={data_hex})",
      0
    );
    AxiReadPayload { data }
  }

  pub(crate) fn axi_read_load_store(&mut self, addr: u32, arsize: u64) -> AxiReadPayload {
    let size = 1 << arsize;
    let data = self.read_mem(addr, size, self.dlen / 8);
    let data_hex = hex::encode(&data);
    info!(
      "[{}] axi_read_high_bandwidth (addr={addr:#x}, size={size}, data={data_hex})",
      0
    );
    AxiReadPayload { data }
  }

  pub(crate) fn watchdog(&mut self) -> u8 {
    let tick = get_t();
    if tick > self.timeout {
      error!("[{}] watchdog timeout", get_t());
      WATCHDOG_TIMEOUT
    } else {
      #[cfg(feature = "trace")]
      if self.dump_end != 0 && tick > self.dump_end {
        info!(
          "[{tick}] run to dump end, exiting (last_commit_cycle={})",
          self.last_commit_cycle
        );
        return WATCHDOG_TIMEOUT;
      }

      #[cfg(feature = "trace")]
      if !self.dump_started && tick >= self.dump_start {
        self.start_dump_wave();
        self.dump_started = true;
      }

      trace!("[{}] watchdog continue", get_t());
      WATCHDOG_CONTINUE
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::process::Command;

  #[test]
  fn test_load_elf() {
    let output = Command::new("nix")
      .args([
        "build",
        "--no-warn-dirty",
        "--print-out-paths",
        "--no-link",
        ".#riscv-tests",
      ])
      .output()
      .expect("fail to get riscv-test path");
    if !output.status.success() {
      panic!("fail to build riscv-test");
    }

    let test_path = String::from_utf8_lossy(&output.stdout).to_string();

    Simulator::load_elf(Path::new(&test_path)).unwrap();
  }

  #[test]
  fn x86_should_fail() {
    let err = Simulator::load_elf(Path::new("/bin/cp")).unwrap_err();
    assert_eq!(format!("{}", err), "ELF is not in RISC-V")
  }
}
