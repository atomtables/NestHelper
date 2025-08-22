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
#[cfg(windows)]
pub(crate) fn kill_process_by_pid(pid: u32) -> std::io::Result<()> {
    use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
    use windows::Win32::Foundation::HANDLE;

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, false, pid);
        if handle.is_invalid() {
            return Err(std::io::Error::last_os_error());
        }

        if TerminateProcess(handle, 1).as_bool() {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}