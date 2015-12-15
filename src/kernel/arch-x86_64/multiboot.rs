#![allow(dead_code)]

use core::mem::transmute;

// The magic field should contain this.
const HEADER_MAGIC: u32 = 0x1BADB002;

// Must pass memory information to OS.
const HEADER_MEMORY_INFO: u32 = 0x00000002;

// This should be in %eax.
const BOOTLOADER_MAGIC: u32 = 0x2BADB002;

// Is there a full memory map?
const INFO_MEMORY_MAP: u32 = 0x00000040;

#[repr(C)]
struct Header {
    // Must be MAGIC - see above.
    magic: u32,
    // Feature flags.
    flags: u32,

    // The above fields plus this one must equal 0 mod 2^32.
    checksum: u32,

    // These are only valid if AOUT_KLUDGE is set.
    header_addr: u32,
    load_addr: u32,
    load_end_addr: u32,
    bss_end_addr: u32,
    entry_addr: u32,

    // These are only valid if VIDEO_MODE is set.
    mode_type: u32,
    width: u32,
    height: u32,
    depth: u32
}

unsafe impl Sync for Header {}

#[repr(C)]
struct ElfSectionHeaderTable {
    num : u32,
    size : u32,
    addr : u32,
    shndx : u32,
}

#[repr(packed)]
struct Info {
    // Multiboot info version number
    flags: u32,

    // Available memory from BIOS
    memory: u64,

    // "root" partition
    boot_device: u32,

    // Kernel command line
    cmdline: u32,

    // Boot-Module list
    mods_count: u32,
    mods_addr: u32,

    // The section header table for ELF
    elf_sec: ElfSectionHeaderTable,

    // Memory Mapping buffer
    mmap_length: u32,
    mmap_addr: u32,

    // Drive Info buffe
    drives_length: u32,
    drives_addr: u32,

    // ROM configuration table
    config_table: u32,

    // Boot Loader Name
    boot_loader_name: u32,

    // APM table
    apm_table: u32,

    // Video
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

const HEADER_FLAGS: u32 = HEADER_MEMORY_INFO;

#[linkage="external"]
#[link_section= ".header"]
static MULTIBOOT_HEADER: Header = Header {
    magic: HEADER_MAGIC,
    flags: HEADER_FLAGS,
    checksum: (-((HEADER_MAGIC + HEADER_FLAGS) as i32) as u32),
    header_addr: 0,
    load_addr: 0,
    load_end_addr: 0,
    bss_end_addr: 0,
    entry_addr: 0,
    mode_type: 0,
    width: 0,
    height: 0,
    depth: 0
};

static mut INFO: Option<*const Info> = None;

pub unsafe fn init(magic: u32, info_ptr: usize) {
    if magic != BOOTLOADER_MAGIC {
        panic!("bad multiboot magic");
    }
    INFO = Some(info_ptr as *const Info);
    if (*INFO.unwrap()).flags & INFO_MEMORY_MAP == 0 {
        panic!("no memory map in multiboot info");
    }
}

pub const MEM_KIND_AVAILABLE: u32 = 1;

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub kind: u32
}

pub struct MemoryMapIter {
    ptr: usize
}

impl MemoryMapIter {
    pub fn new() -> MemoryMapIter {
        unsafe { MemoryMapIter{ptr: (*INFO.unwrap()).mmap_addr as usize} }
    }
}

impl Iterator for MemoryMapIter {
    type Item = MemoryRegion;

    fn next(&mut self) -> Option<MemoryRegion> {
        unsafe {
            let mmap_ptr = (*INFO.unwrap()).mmap_addr as usize;
            let mmap_len = (*INFO.unwrap()).mmap_length as usize;

            if self.ptr >= mmap_ptr + mmap_len {
                return None;
            }

            let reg: *const MemoryRegion = transmute(self.ptr);
            self.ptr += 4 + (*reg).size as usize;
            Some(*reg)
        }
    }
}
