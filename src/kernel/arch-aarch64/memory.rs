use memory::{self, MemoryRegion};

// TODO: add proper treatment of reserved memory regions
fn detect_memory<'a>(buf: &'a mut [MemoryRegion]) -> &'a [MemoryRegion] {
    use arch::device_tree as dt;

    let dtreg = dt::device_tree_memory_region();
    let kreg = memory::kernel_memory_region();

    let mut len = 0;
    for mut iter in dt::PathIter::new(dt::Iter::new(), "/memory/reg", true) {
        if let Some(dt::Token::Property{name:_, value}) = iter.next() {
            for reg in dt::to_memory_regions(value) {
                buf[len] = MemoryRegion{
                    address: reg.address() as usize,
                    size: reg.size() as usize
                };

                // TODO: replace this with more generic code 
                if buf[len].address == dtreg.address {
                    let diff = dtreg.size + kreg.size;
                    buf[len].address += diff;
                    buf[len].size -= diff;
                }

                len += 1;
            }
        }
    }

    &buf[..len]
}

pub unsafe fn init() {
    use config::MEMORY_REGIONS_MAX;
    let mut buf = [MemoryRegion{address:0, size:0}; MEMORY_REGIONS_MAX];
    memory::init(detect_memory(&mut buf[..]));
}
