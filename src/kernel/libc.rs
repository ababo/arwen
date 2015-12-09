#[no_mangle]
pub unsafe extern fn memcpy(dst: *mut (), src: *const (),
                            num: usize) -> *mut () {
    let mut curd = dst as *mut u8;
    let mut curs = src as *mut u8;
    let endd = curd.offset(num as isize);
    while curd < endd {
        *curd = *curs;
        curd = curd.offset(1);
        curs = curs.offset(1);
    }
    dst
}

#[no_mangle]
pub unsafe extern fn memset(ptr: *mut (), value: i32, num: usize) -> *mut () {
    let mut cur = ptr as *mut u8;
    let end = cur.offset(num as isize);
    while cur < end {
        *cur = value as u8;
        cur = cur.offset(1);
    }
    ptr
}

#[no_mangle]
pub unsafe extern fn strlen(str: *const u8) -> usize {
    let mut len = 0;
    let mut ptr = str;
    while *ptr != 0 {
        len += 1;
        ptr = ptr.offset(1);
    }
    len
}

macro_rules! dummy_syms {
    ($($sym:ident)*) => ($(
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static $sym: usize = 0;
    )*)
}

dummy_syms! {
    memcmp floor ceil round trunc floorf ceilf roundf truncf
    exp exp2 expf exp2f fmod fmodf pow powf __powisf2 __powidf2
    log log2 log10 logf log2f log10f fma fmaf
}
