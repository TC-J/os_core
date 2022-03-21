//! OS related constants (typically C-style 'defines')
pub mod mprot {
    pub const NONE: i32 = libc::PROT_NONE;
    pub const READ: i32 = libc::PROT_READ;
    pub const WRITE: i32 = libc::PROT_WRITE;
    pub const EXEC: i32 = libc::PROT_EXEC;
    pub const GROWSDOWN: i32 = libc::PROT_GROWSDOWN;
    pub const GROWSUP: i32 = libc::PROT_GROWSUP;
}

pub mod map {
    pub const FAILED: *mut () = libc::MAP_FAILED as *mut ();
    pub const FIXED: i32 = libc::MAP_FIXED;
    pub const ANONYMOUS: i32 = libc::MAP_ANONYMOUS;
    pub const ANON: i32 = libc::MAP_ANON;
    pub const GROWSDOWN: i32 = libc::MAP_GROWSDOWN;
    pub const NORESERVE: i32 = libc::MAP_NORESERVE;
    pub const LOCKED: i32 = libc::MAP_LOCKED;
    pub const FIXED_NOREPLACE: i32 = libc::MAP_FIXED_NOREPLACE;
    pub const SHARED_VALIDATE: i32 = libc::MAP_SHARED_VALIDATE;
    pub const SHARED: i32 = libc::MAP_SHARED;
    pub const PRIVATE: i32 = libc::MAP_PRIVATE;
    pub const POPULATE: i32 = libc::MAP_POPULATE;
    //TODO add all linux mmap flags
    //TODO add all osx mmap flags
}

pub mod unshare {
    pub const NSMOUNT: i32 = libc::CLONE_NEWNS;
    pub const NSUSER: i32 = libc::CLONE_NEWUSER;
    pub const NSIPC: i32 = libc::CLONE_NEWIPC;
    pub const NSNET: i32 = libc::CLONE_NEWNET;
    pub const NSPID: i32 = libc::CLONE_NEWPID;
    pub const NSUTS: i32 = libc::CLONE_NEWUTS;
    pub const NSCGROUP: i32 = libc::CLONE_NEWCGROUP;
    pub const FILES : i32 = libc::CLONE_FILES;
    pub const FS : i32 = libc::CLONE_FS;
    pub const VM : i32 = libc::CLONE_VM;
}