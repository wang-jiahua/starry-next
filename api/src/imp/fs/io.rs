use core::ffi::{c_char, c_void};

use arceos_posix_api::{self as api, AT_FDCWD, ctypes::mode_t};
use axerrno::{LinuxError, LinuxResult};

use crate::ptr::{PtrWrapper, UserConstPtr, UserPtr};

pub fn sys_access(pathname: UserConstPtr<c_char>, mode: i32) -> LinuxResult<isize> {
    sys_faccessat(AT_FDCWD as _, pathname, mode, 0)
}

pub fn sys_faccessat(
    dirfd: i32,
    pathname: UserConstPtr<c_char>,
    mode: i32,
    _flags: i32, // TODO: support flags
) -> LinuxResult<isize> {
    let path = pathname.get_as_null_terminated()?;
    let file_path =
        arceos_posix_api::handle_file_path(dirfd as _, Some(path.as_ptr() as _), false)?;
    axfs::api::metadata(file_path.as_str())
        .map(|metadata| {
            if mode == 0 {
                // F_OK
                if axfs::api::absolute_path_exists(file_path.as_str()) {
                    Ok(0)
                } else {
                    Err(LinuxError::ENOENT)
                }
            } else {
                let mut ret = true;
                if mode & 1 != 0 {
                    // X_OK
                    ret &= metadata.permissions().owner_executable();
                }
                if mode & 2 != 0 {
                    // W_OK
                    ret &= metadata.permissions().owner_writable();
                }
                if mode & 4 != 0 {
                    // R_OK
                    ret &= metadata.permissions().owner_readable();
                }
                Ok(ret as isize - 1)
            }
        })
        .unwrap_or_else(|_| Err(LinuxError::ENOENT))
}

pub fn sys_read(fd: i32, buf: UserPtr<c_void>, count: usize) -> LinuxResult<isize> {
    let buf = buf.get_as_bytes(count)?;
    Ok(api::sys_read(fd, buf, count))
}

pub fn sys_write(fd: i32, buf: UserConstPtr<c_void>, count: usize) -> LinuxResult<isize> {
    let buf = buf.get_as_bytes(count)?;
    Ok(api::sys_write(fd, buf, count))
}

pub fn sys_writev(
    fd: i32,
    iov: UserConstPtr<api::ctypes::iovec>,
    iocnt: i32,
) -> LinuxResult<isize> {
    let iov = iov.get_as_bytes(iocnt as _)?;
    unsafe { Ok(api::sys_writev(fd, iov, iocnt)) }
}

pub fn sys_openat(
    dirfd: i32,
    path: UserConstPtr<c_char>,
    flags: i32,
    modes: mode_t,
) -> LinuxResult<isize> {
    let path = path.get_as_null_terminated()?;
    Ok(api::sys_openat(dirfd, path.as_ptr(), flags, modes) as _)
}

pub fn sys_open(path: UserConstPtr<c_char>, flags: i32, modes: mode_t) -> LinuxResult<isize> {
    use arceos_posix_api::AT_FDCWD;
    sys_openat(AT_FDCWD as _, path, flags, modes)
}
