/// unshare copies resources into a new context that will not affect the other contexts (from other processes and threads)
/// 
/// create new namespaces, open-file-tables, virtual-memory mappings, and filesystem information (current directory, root directory, and umask)
pub fn sys_unshare (flags: i32) -> i32 {
    unsafe {
        libc::unshare(flags)
    }
}

/// change the root directory for pathname resolution in the calling process
pub fn sys_chroot(path: &str) -> i32 {
    let mut path = path.to_owned();
    path.push('\0');

    let path = (*path).as_ptr() as *const i8;
    unsafe {
        libc::chroot(path)
    }
}

/// Switch the current root, in the current mount-namespace, to a directory (that must be in a separate mount from the current root)
/// and then mount the current root (now the previous root) on a directory inside the new root -- should probably unshare the
/// mount namespace prior
pub fn sys_pivot_root(newroot: &str, oldroot_mnt_point: &str) -> i32 {
    let mut newroot = newroot.to_owned();
    let mut oldroot_mnt_point = oldroot_mnt_point.to_owned();

    newroot.push('\0');
    oldroot_mnt_point.push('\0');

    unsafe {
        libc::syscall(libc::SYS_pivot_root, newroot.as_ptr() as *const u8, oldroot_mnt_point.as_ptr() as *const u8) as i32
    }
}