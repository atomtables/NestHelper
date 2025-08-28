/**
    I had to use AI for these functions because I legitimately am not trying to tryhard learning
about every bloody crate in Rust. This is why I use C. Just race condition your way through life.
*/

#[cfg(unix)]
pub(crate) fn kill_process_by_pid(pid: u32) -> std::io::Result<()> {
    use libc::{kill, SIGKILL};
    unsafe {
        if kill(pid as i32, SIGKILL) == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}
// if this doesn't work i quit rust
#[cfg(windows)]
pub(crate) fn kill_process_by_pid(pid: u32) -> std::io::Result<()> {
    use windows::Win32::Foundation::{HANDLE, CloseHandle};
    use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, false, pid)?;
        if handle.is_invalid() {
            return Err(std::io::Error::last_os_error());
        }

        let result = if TerminateProcess(handle, 1) != 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        };

        CloseHandle(handle);
        result
    }
}
