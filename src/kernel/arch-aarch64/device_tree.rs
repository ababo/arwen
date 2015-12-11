use core::iter::Iterator;
use core::mem::transmute;
use core::slice::from_raw_parts;
use core::str::from_utf8;
use libc::strlen;

#[repr(C)]
struct Header {
    magic: u32,
    total_size: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32
}

static mut HEADER: Option<&'static Header> = None;

const HEADER_MAGIC: u32 = 0xD00DFEED;

const TOKEN_BEGIN_NODE: u32 = 1;
const TOKEN_END_NODE: u32 = 2;
const TOKEN_PROPERTY: u32 = 3;
const TOKEN_NOP: u32 = 4;
const TOKEN_END: u32 = 9;

pub unsafe fn init(header_ptr: usize) {
    let header = header_ptr as *const Header;
    if u32::from_be((*header).magic) != HEADER_MAGIC {
        panic!("bad device tree magic");
    }
    HEADER = Some(transmute(header_ptr));
}

#[derive(Clone, Copy)]
pub struct Iter {
    ptr: *const u32
}

impl Iter {
    pub fn new() -> Iter {
        unsafe {
            let addr: usize = transmute(HEADER.unwrap());
            let off = u32::from_be(HEADER.unwrap().off_dt_struct) as usize;
            let ptr: *const u32 = transmute(addr+off);
            assert_eq!(u32::from_be(*ptr), TOKEN_BEGIN_NODE);
            Iter{ptr:ptr}
        }
    }
}


#[derive(Debug)]
pub enum Token {
    BeginNode{ name: &'static str },
    EndNode,
    Property{ name: &'static str, value: &'static [u8]},
    Nop
}

impl Iterator for Iter {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        unsafe {
            match u32::from_be(*self.ptr) {
                TOKEN_BEGIN_NODE => {
                    let ptr = transmute(self.ptr.offset(1));
                    let slice = from_raw_parts(ptr, strlen(ptr));
                    let name = from_utf8(slice).unwrap();
                    let off = 1 + blocks_used!(name.len()+1, 4);
                    self.ptr = self.ptr.offset(off as isize);
                    Some(Token::BeginNode{name:name})
                },
                TOKEN_END_NODE => {
                    self.ptr = self.ptr.offset(1);
                    Some(Token::EndNode)
                },
                TOKEN_PROPERTY => {
                    let len = u32::from_be(*self.ptr.offset(1));
                    let addr: usize = transmute(HEADER.unwrap());
                    let off = u32::from_be(HEADER.unwrap().off_dt_strings);
                    let off2 = u32::from_be(*self.ptr.offset(2));
                    let ptr = transmute(addr + (off + off2) as usize);
                    let slice = from_raw_parts(ptr, strlen(ptr));
                    let name = from_utf8(slice).unwrap();
                    let ptr2 = transmute(self.ptr.offset(3));
                    let value = from_raw_parts(ptr2, len as usize);
                    let off = 3 + blocks_used!(len, 4);
                    self.ptr = self.ptr.offset(off as isize);
                    Some(Token::Property{name:name, value:value})
                },
                TOKEN_NOP => {
                    self.ptr = self.ptr.offset(1);
                    Some(Token::Nop)
                },
                TOKEN_END => None,
                _ => panic!("bad device tree token")
            }
        }
    }
}

pub struct PathIter<'a> {
    iter: Iter,
    path: &'a str,
    ignore_addr: bool,
    component: &'a str
}

impl<'a> PathIter<'a> {
    fn discard_addr<'b>(name: &'b str) -> &'b str {
        name.split('@').next().unwrap()
    }

    fn first_component(path: &'a str) -> &'a str {
        path.split('/').next().unwrap()
    }

    fn component_offset(&self) -> usize {
        let addr1 = self.component.as_ptr() as usize;
        let addr2 = self.path.as_ptr() as usize;
        addr1 - addr2
    }

    fn advance_component<I>(&self, mut split: I) -> Option<&'a str>
        where I: Iterator<Item = &'a str> {
        split.next().unwrap();
        match split.next() {
            Some(next) => Some(next),
            None => None
        }
    }

    fn next_component(&self) -> Option<&'a str> {
        let from = self.component_offset() + self.component.len();
        self.advance_component((&self.path[from..]).split('/'))
    }

    fn prev_component(&self) -> Option<&'a str> {
        let to = self.component_offset();
        self.advance_component((&self.path[..to]).rsplit('/'))
    }

    pub fn new(iter: Iter, path: &'a str, ignore_addr: bool) -> PathIter<'a> {
        PathIter{
            iter: iter,
            path: path,
            ignore_addr: ignore_addr,
            component: Self::first_component(path)
        }
    }

    fn skip_node(&mut self) {
        let mut level = 1;
        while let Some(token) = self.iter.next() {
            match token {
                Token::BeginNode{name:_} => {
                    level += 1;
                },
                Token::EndNode => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                },
                _ => {}
            }
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = Iter;

    // TODO: remove `as_bytes()` in string
    // comparisons when compiler will stop crashing
    fn next(&mut self) -> Option<Iter> {
        let mut prev = self.iter.clone();
        while let Some(token) = self.iter.next() {
            match token {
                Token::BeginNode{mut name} => {
                    if self.ignore_addr { name = Self::discard_addr(name); }
                    if name.as_bytes() == self.component.as_bytes() {
                        match self.next_component() {
                            Some(comp) => {
                                self.component = comp;
                            },
                            None => {
                                self.skip_node();
                                return Some(prev);
                            }
                        }
                    } else {
                        self.skip_node();
                    }
                },
                Token::EndNode => {
                    match self.prev_component() {
                        Some(comp) => {
                            self.component = comp;
                        },
                        None => {
                            return None;
                        }
                    }
                },
                Token::Property{name, value:_} => {
                    if self.next_component().is_none() &&
                        name.as_bytes() == self.component.as_bytes() {
                            return Some(prev);
                    }
                },
                _ => {}
            };
            prev = self.iter.clone();
        }
        None
    }
}
