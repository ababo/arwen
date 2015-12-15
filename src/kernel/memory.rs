use config::MEMORY_REGIONS_MAX;

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub address: usize,
    pub size: usize
}

static mut AVAILABLE_BUF: [MemoryRegion; MEMORY_REGIONS_MAX] =
    [MemoryRegion{address:0, size:0}; MEMORY_REGIONS_MAX];
static mut AVAILABLE: &'static [MemoryRegion] =
    &[MemoryRegion{address:0, size:0}; 0];

unsafe fn set_available_memory(available: &[MemoryRegion]) {
    let mut iter = available.iter();
    if let Some(mut prev) = iter.next() {
        for cur in iter {
            assert!(cur.address >= prev.address + prev.size,
                "available memory regions are overlapping or non-sorted");
            prev = cur;
        }
    }

    for (to, from) in AVAILABLE_BUF.iter_mut().zip(available.iter()) {
        *to = *from
    }
    AVAILABLE = &AVAILABLE_BUF[..available.len()];
}

pub unsafe fn init(available: &[MemoryRegion]) {
    set_available_memory(available);
    for region in available {
        klog_debug!("available memory: {}KiB from 0x{:X}",
            region.size/1024, region.address);
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
