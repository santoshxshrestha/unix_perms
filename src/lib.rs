use libc::{getpwuid, uid_t};
use std::ffi::CStr;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

/// Converts a file's mode into a `ls -l` style permission string.
pub fn display_permissions(metadata: &Metadata) -> String {
    let mode = metadata.mode();
    let file_type = match mode & libc::S_IFMT {
        libc::S_IFREG => '-',
        libc::S_IFDIR => 'd',
        libc::S_IFLNK => 'l',
        libc::S_IFCHR => 'c',
        libc::S_IFBLK => 'b',
        libc::S_IFSOCK => 's',
        libc::S_IFIFO => 'p',
        _ => '?',
    };

    let mut perm_string = String::with_capacity(10);
    perm_string.push(file_type);

    let permission_bits = [
        (libc::S_IRUSR, 'r'),
        (libc::S_IWUSR, 'w'),
        (libc::S_IXUSR, 'x'),
        (libc::S_IRGRP, 'r'),
        (libc::S_IWGRP, 'w'),
        (libc::S_IXGRP, 'x'),
        (libc::S_IROTH, 'r'),
        (libc::S_IWOTH, 'w'),
        (libc::S_IXOTH, 'x'),
    ];

    for (bit, chr) in permission_bits.iter() {
        perm_string.push(if mode & bit != 0 { *chr } else { '-' });
    }

    perm_string
}

pub fn get_name(uid: u32) -> String {
    let p = unsafe { getpwuid(uid as uid_t) };

    if p.is_null() {
        return "_".to_string();
    }

    let username = unsafe { CStr::from_ptr((*p).pw_name) };

    username.to_str().unwrap_or("_").to_string()
}
