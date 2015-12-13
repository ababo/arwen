#![allow(dead_code)]

use arch::device_tree::{self, to_memory_regions};
use config::MEMORY_REGIONS_MAX;
use klog;
use memory::{self, MemoryRegion};

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

pub fn kernel_memory_region() -> MemoryRegion {
    extern {
        static __kbaddr: usize;
        static __keaddr: usize;
    }
    let kbaddr = &__kbaddr as *const usize as usize;
    let keaddr = &__keaddr as *const usize as usize;
    MemoryRegion{address:kbaddr, size:(keaddr-kbaddr)}
}

// TODO: add proper treatment of reserved memory regions
fn detect_memory<'a>(buf: &'a mut [MemoryRegion]) -> &'a [MemoryRegion] {
    use arch::device_tree::{Iter, PathIter, Token};

    // TODO: exclude device tree and kernel memory regions

    let mut len = 0;
    for mut iter in PathIter::new(Iter::new(), "/memory/reg", true) {
        if let Some(Token::Property{name:_, value}) = iter.next() {
            for reg in to_memory_regions(value) {
                buf[len] = MemoryRegion{
                    address: reg.address() as usize,
                    size: reg.size() as usize
                };
                len += 1;
            }
        }
    }
    &buf[..len]
}

#[no_mangle]
#[linkage="external"]
#[allow(private_no_mangle_fns)]
unsafe extern fn __boot() {
    klog::init(write, klog::Level::Debug);
    device_tree::init(DEVICE_TREE_ADDRESS);
    memory::init(detect_memory(
        &mut [MemoryRegion{address:0, size:0}; MEMORY_REGIONS_MAX][..]));
    klog_debug!("ok");
}
