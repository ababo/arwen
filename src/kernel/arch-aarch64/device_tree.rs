const HEADER_MAGIC: u32 = 0xD00DFEED;

#[repr(C)]
struct Header {
	pub magic: u32,
	pub total_size: u32,
	pub off_dt_struct: u32,
	pub off_dt_strings: u32,
	pub off_mem_rsvmap: u32,
	pub version: u32,
	pub last_comp_version: u32,
	pub boot_cpuid_phys: u32,
	pub size_dt_strings: u32,
	pub size_dt_struct: u32
}

pub fn init(header_ptr: usize) {
	let header = header_ptr as *const Header;
	unsafe {
		if u32::from_be((*header).magic) != HEADER_MAGIC {
			panic!("bad device tree magic");
		}

	}


}
