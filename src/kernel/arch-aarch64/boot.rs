use arch::device_tree;
use klog;

fn write(s: &str) {
    let ptr = 0x9000000 as *mut u8;
    for c in s.chars() {
        unsafe {   
        	*ptr = c as u8;
        }
    }
}

const DEVICE_TREE_PTR: usize = 0x4000_0000;

#[no_mangle]
pub unsafe extern fn __boot() {
	klog::init(write, klog::Level::Debug);
	device_tree::init(DEVICE_TREE_PTR);

    for token in device_tree::Iter::new() {
        klog_debug!("token: {:?}", token);
    }
}
