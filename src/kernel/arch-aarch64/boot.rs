use arch::device_tree;
use klog;

#[allow(dead_code)]
fn write(s: &str) {
    let ptr = 0x9000000 as *mut u8;
    for c in s.chars() {
        unsafe {   
        	*ptr = c as u8;
        }
    }
}

#[allow(dead_code)]
const DEVICE_TREE_PTR: usize = 0x4000_0000;

#[no_mangle]
#[linkage="external"]
#[allow(private_no_mangle_fns)]
unsafe extern fn __boot() {
	klog::init(write, klog::Level::Debug);
	device_tree::init(DEVICE_TREE_PTR);

    let root = device_tree::Iter::new();
    let piter = device_tree::PathIter::new(root, "/memory/reg", true);
    for mut iter in piter {
        klog_debug!("token: {:?}", iter.next().unwrap());
    }
}
