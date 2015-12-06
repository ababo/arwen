use config::MEMORY_SEGMENTS_MAX;

#[derive(Clone, Copy)]
pub enum SegmentKind {
    Unknown = 0,
    Regular = 1,
    Reserved = 2,
}

#[derive(Clone, Copy)]
pub struct Segment {
    pub kind: SegmentKind,
    pub addr: usize,
    pub size: usize
}

pub struct MemoryMap {
    segment_count: usize,
    segments: [Segment; MEMORY_SEGMENTS_MAX]
}

static mut MEMORY_MAP: MemoryMap = MemoryMap {
    segment_count: 0,
    segments: [Segment{kind: SegmentKind::Unknown, addr: 0, size: 0};
               MEMORY_SEGMENTS_MAX]
};

pub unsafe fn memory_map_mut() -> &'static mut MemoryMap {
    &mut MEMORY_MAP
}

pub fn memory_map() -> &'static MemoryMap {
    unsafe { &MEMORY_MAP }
}
