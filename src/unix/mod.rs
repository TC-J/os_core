//! Operating system calls that work on OSX and Linux
/// Allocates physical memory for a file, device, or anonymous-file (ie, a file that is only stored in-memory; just a memory allocation for the running process).
/// 
/// Typically, the data in the device (eg, disk) is loaded when the page of memory is accessed. I use the word "typically" because 
/// the implementation of the device determines how the data is populated in the physical-page, for instance you could implement the mmap functions 
/// for a device that calls out over the network to populate the physical-page when the first-access to the memory occurs (per page). 
/// 
/// For anonomyous files: the page of memory being accessed is allocated a physical-page in RAM whenever it is accessed. 
///
/// *NOTE*: This is the best function for allocating program memory, and is the most effecient function for loading files
/// when the file-contents are accessed randomly; otherwise, if read sequentially once or written sequentially once, use direct-io (see open(2))
///  
/// *NOTE*: Accessing outside of the logically-allocated-range will not result in a physical-page being allocated, to clarify.
/// 
/// # Example
/// mapping memory (not based on device or file). give it an address you want the mapping to be at, or just leave it blank (99% of the time)
/// then set the memory protection flags and allocation flags, use ANON for in-memory only, use -1 for the file-descriptor, and 0 as the offset.
/// 
/// the minimum flags for `map_flags` is one of either SHARED or PRIVATE.
/// ```
/// use os_core::{unix::{sys_mmap, sys_munmap}, constants::{map, mprot}};
/// 
/// let mut mapping = sys_mmap(0 as *mut (), 4096, mprot::READ | mprot::WRITE, map::ANON | map::SHARED, -1, 0);
/// assert!(mapping != map::FAILED);
/// 
/// let mut data = mapping as *mut u8;
/// unsafe {*(data.offset(4095)) = 0xFF;}
/// assert_eq!(unsafe {*(data.offset(4095))}, 0xFF);
/// sys_munmap(mapping, 4096);
/// // sanity: this would also work -> sys_munmap(data, 4096); <- since it's the same address (data and mapping)
/// ```
pub fn sys_mmap(addr: *mut (), len: usize, mprot_flags: i32, map_flags: i32, fd: i32, offset: i64) -> *mut () {
    unsafe {
        libc::mmap(addr as *mut libc::c_void, len, mprot_flags, map_flags, fd, offset) as *mut ()
    }
}

/// Unmap program memory and memory-mapped devices/files
pub fn sys_munmap(addr: *mut (), len: usize) -> i32 {
    unsafe {
        libc::munmap(addr as *mut libc::c_void, len)
    }
}

/// Create a new process by copying the memory-mappings in the current process into a new address space and get a separate
/// scheduling-context in the OS.
pub fn sys_fork() -> i32 {
    unsafe {
        libc::fork()
    }
}

/// Wait for any of the caller's child-process's scheduling-statuses to change (not including going from idle to runnable, 
/// more signal related statuses; see manpage wait(2))
/// 
/// Returns a 2-tuple with the PID first, then the status of the child process.
pub fn sys_wait() -> (i32, i32) {
    unsafe {
        let mut status = 0;
        let pid = libc::wait(&mut status);
        (pid, status)
    }
}

/// Create a pipe
/// 
/// The first i32 is the read-end of the pipe, the second i32 is the write-end
pub fn sys_pipe (pfds: &mut [i32; 2]) -> i32 {
    unsafe {
        libc::pipe(pfds as *mut i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{map, mprot};

    #[test]
    fn test_sys_mmap_munmap() {
        use crate::constants::{map, mprot};
        let mut mem = 0 as *mut ();

        mem = sys_mmap(0 as *mut (), 4096, mprot::READ | mprot::WRITE, map::ANON | map::SHARED, -1, 0);
        
        assert!(map::FAILED != mem);
        assert!(mem as usize > 0);

        let mut data = mem as *mut u8;
        for i in 0..4096 {
            unsafe {*(data.offset(i)) = (i % 256) as u8; }
        }

        for i in 0..4096 {
            assert_eq!(unsafe {*(data.offset(i))}, (i % 256) as u8);
        }

        assert_eq!(sys_munmap(mem, 4096), 0);
    }
    
    #[test]
    fn test_sys_fork() {
        let pid = unsafe {libc::getpid()};
        let mut ch_pid = sys_fork();

        if ch_pid == 0 {
        // child
            unsafe {assert_eq!(libc::getppid(), pid);}
            std::process::exit(0);
        }

        let mut status = -1;
        unsafe {
            assert_eq!(libc::wait(&mut status as *mut i32), ch_pid);
            assert!(libc::WIFEXITED(status));
        }
    }
}