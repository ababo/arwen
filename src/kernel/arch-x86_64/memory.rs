use memory::{self, MemoryRegion};

// TODO: add proper treatment of reserved memory regions
fn detect_memory<'a>(buf: &'a mut [MemoryRegion]) -> &'a [MemoryRegion] {
    use arch::multiboot;

    let kreg = memory::kernel_memory_region();

    let mut len = 0;
    for reg in multiboot::MemoryMapIter::new() {
        if reg.kind == multiboot::MEM_KIND_AVAILABLE {
            buf[len] = MemoryRegion{
                address: reg.base_addr as usize,
                size: reg.length as usize
            };

            // TODO: replace this with more generic code 
            if buf[len].address == kreg.address {
                buf[len].address += kreg.size;
                buf[len].size -= kreg.size;
            }

            len += 1;
        }
    }

    &buf[..len]
}

pub unsafe fn init() {
    use config::MEMORY_REGIONS_MAX;
    let mut buf = [MemoryRegion{address:0, size:0}; MEMORY_REGIONS_MAX];
    memory::init(detect_memory(&mut buf[..]));
}
