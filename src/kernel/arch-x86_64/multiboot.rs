// The magic field should contain this.
const HEADER_MAGIC: u32 = 0x1BADB002;

// Must pass memory information to OS.
const HEADER_MEMORY_INFO: u32 = 0x00000002;

// This should be in %eax.
const BOOTLOADER_MAGIC: u32 = 0x2BADB002;

#[repr(C)]
struct Header {
    // Must be MAGIC - see above.
    pub magic: u32,
    // Feature flags.
    pub flags: u32,

    // The above fields plus this one must equal 0 mod 2^32.
    pub checksum: u32,

    // These are only valid if AOUT_KLUDGE is set.
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
    pub entry_addr: u32,

    // These are only valid if VIDEO_MODE is set.
    pub mode_type: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

unsafe impl Sync for Header {}

#[repr(C)]
struct ElfSectionHeaderTable {
    pub num : u32,
    pub size : u32,
    pub addr : u32,
    pub shndx : u32,
}

#[repr(C)]
struct Info {
    // Multiboot info version number
    pub flags: u32,

    // Available memory from BIOS
    pub mem_lower: u32,
    pub mem_upper: u32,

    // "root" partition
    pub boot_device: u32,

    // Kernel command line
    pub cmdline: u32,

    // Boot-Module list
    pub mods_count: u32,
    pub mods_addr: u32,

    // The section header table for ELF
    pub elf_sec: ElfSectionHeaderTable,

    // Memory Mapping buffer
    pub mmap_length: u32,
    pub mmap_addr: u32,

    // Drive Info buffe
    pub drives_length: u32,
    pub drives_addr: u32,

    // ROM configuration table
    pub config_table: u32,

    // Boot Loader Name
    pub boot_loader_name: u32,

    // APM table
    pub apm_table: u32,

    // Video
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
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

pub fn init(magic: u32, info_ptr: usize) {

}
