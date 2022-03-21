/// unshare copies resources into a new context that will not affect the other contexts (from other processes and threads)
/// 
/// create new namespaces, open-file-tables, virtual-memory mappings, and filesystem information (current directory, root directory, and umask)
pub fn sys_unshare (flags: i32) -> i32 {
    unsafe {
        libc::unshare(flags)
    }
}