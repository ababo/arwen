use arch::memory;
use arch::multiboot;
use klog;

// TODO: replace with a proper serial port handling code
fn write(s: &str) {
    let port = 0x400 as *const u16;
    for b in s.chars() {
        unsafe {
            asm!("outb $0, $1" : : "{al}"(b as u8), "{dx}"(*port));
        }
    }
}

#[no_mangle]
pub unsafe extern fn __boot(magic: u32, info_ptr: usize) {
    klog::init(write, klog::Level::Debug);
    multiboot::init(magic, info_ptr);
    memory::init();
}
