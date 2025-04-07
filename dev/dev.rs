use winapi::um::winuser::*;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Arc;
use std::ptr::null_mut;



struct KeepAliveMessageLoop(Arc<AtomicBool>);
impl Drop for KeepAliveMessageLoop {
    fn drop(&mut self) { self.0.store(true, Relaxed); }
}

/// Like `std::thread::spawn(|| ...).join().unwrap()`, but pump the message loop of the calling thread while you're at it.
///
/// Pumping the message loop allows COM objects associated with an STA thread to respond to marshaling events from another COM apartment.
pub fn spawn_pump_join<R: Send + 'static>(thread: impl 'static + Send + FnOnce() -> R) -> R {
    let quit = Arc::new(AtomicBool::new(false));
    let kaml = KeepAliveMessageLoop(quit.clone());

    let thread = std::thread::spawn(move ||{
        let _kaml = kaml;
        thread()
    });

    while !quit.load(Relaxed) {
        let mut msg = unsafe { std::mem::zeroed::<MSG>() };
        let avail = unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) };
        if avail == 0 {
            std::thread::yield_now();
            continue;
        }

        let _ = unsafe { TranslateMessage(&msg) }; // return indicates if `msg` was translated.  Many/most aren't, and that's fine.
        let _ = unsafe { DispatchMessageW(&msg) }; // return is message specific
    }

    thread.join().unwrap()
}
