//! # os_core
//! Core operating system functions
//! 
//! These work on Linux and OSX
#![allow(unused, non_camel_case_types)]
pub extern crate libc;

pub mod constants;
pub mod types;
pub mod linux;
pub mod unix;

/// allocate raw program memory (atleast around the size of the system's page-size (usually 4096); ie, not small allocations)
/// 
/// ```
/// # Example
/// use os_core::{memory, release};
/// 
/// let mut mem: *mut u8 = memory(4096, true, true, true);
/// 
/// unsafe {*mem} = 0xAB;
/// 
/// assert_eq!(unsafe {*mem}, 0xAB);
/// 
/// release(mem, 4096);
/// ```
/// 
pub fn memory <T> (size: usize, read: bool, write: bool, exec: bool, shared: bool) -> *mut T {
    use crate::{constants::{map, mprot}, unix::sys_mmap};

    let mut prot = mprot::NONE;
    let mut flags = map::ANON;

    if read { prot |= mprot::READ; }
    if write { prot |= mprot::WRITE; }
    if exec { prot |= mprot::EXEC; }

    if shared { flags |= map::SHARED; }
    else { flags |= map::PRIVATE; }

    sys_mmap(0 as *mut (), size, prot, flags, -1, 0) as *mut T
}

/// release raw program memory (unmanaged memory); ie, allocated with `fn memory` or `fn sys_munmap`
pub fn release <T> (mem: *mut T, size: usize) {
    crate::unix::sys_munmap(mem as *mut (), size);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
