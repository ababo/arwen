use arch::multiboot;
use klog;

const HEADER_FLAGS: u32 = multiboot::HEADER_MEMORY_INFO;

pub static MULTIBOOT_HEADER: multiboot::Header = multiboot::Header {
    magic: multiboot::HEADER_MAGIC,
    flags: HEADER_FLAGS,
    checksum: (-((multiboot::HEADER_MAGIC + HEADER_FLAGS) as i32) as u32),
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

fn write(s: &str) {
    let port = 0x400 as *const u16;
    for b in s.chars() {
        unsafe {
            asm!("outb $0, $1" : : "{al}"(b as u8), "{dx}"(*port));
        }
    }
}

#[no_mangle]
pub extern fn __boot(magic: u32, _info: &multiboot::Info) {
    if magic != multiboot::BOOTLOADER_MAGIC {
        return
    }

    klog::init(write, klog::Level::Debug);
    klog_debug!("Hello {} {} {}!", "world", 123, "!!");
}
