mod deferred;
mod injector;
mod shellcode;

use deferred::Deferred;
pub use injector::{Error, Result};

pub fn inject_dll_manual_map(pid: u32, mut test: Vec<u8>) -> Result<()> {
    unsafe {
        let mut deferred = Deferred::new();
        let h_process = injector::open_process(&mut deferred, pid)?;

        injector::inject_dll_manual_map(&mut deferred, h_process, &mut test)
    }
}
