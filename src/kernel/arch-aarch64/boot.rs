use klog;

fn write(s: &str) {
    let ptr = 0x9000000 as *mut u8;
    for c in s.chars() {
        unsafe {   
        	*ptr = c as u8;
        }
    }
}

#[no_mangle]
pub extern fn __boot() {
	klog::init(write, klog::Level::Debug);
    klog_debug!("Hello {} {} {}!", "world", 123, "!!");
}
