use arch::device_tree;
use arch::memory;
use klog;

const SERIAL_PORT_ADDRESS: usize = 0x0900_0000;
const DEVICE_TREE_ADDRESS: usize = 0x4000_0000;

// TODO: replace with a proper serial port handling code
fn write(s: &str) {
    let ptr = SERIAL_PORT_ADDRESS as *mut u8;
    for c in s.chars() {
        unsafe {   
            *ptr = c as u8;
        }
    }
}

#[no_mangle]
pub unsafe extern fn __boot() {
    klog::init(write, klog::Level::Debug);
    device_tree::init(DEVICE_TREE_ADDRESS);
    memory::init();
}
