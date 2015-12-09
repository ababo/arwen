use arch::multiboot;
use klog;

fn write(s: &str) {
    let port = 0x400 as *const u16;
    for b in s.chars() {
        unsafe {
            asm!("outb $0, $1" : : "{al}"(b as u8), "{dx}"(*port));
        }
    }
}

#[no_mangle]
#[linkage="external"]
pub unsafe extern fn __boot(magic: u32, info_ptr: usize) {
    klog::init(write, klog::Level::Debug);
    multiboot::init(magic, info_ptr);
    klog_debug!("ok");
}
