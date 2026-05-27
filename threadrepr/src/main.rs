#![feature(stdarch_aarch64_sve)]
use core::arch::aarch64::*;
#[cfg(windows)]
use core::ffi::c_void;
use std::sync::LazyLock;
use std::thread::Builder;

static I32_DATA: LazyLock<[i32; 64 * 5]> = LazyLock::new(|| {
    (0..64 * 5)
        .map(|i| i as i32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i32 data incorrectly initialised")
});

const THREADS: usize = 45;

fn main() {
    let threads = (0..THREADS)
        .map(|i| {
            Builder::new()
                .name(format!("thread_{}", i))
                .spawn(|| {
                    pin_thread_to_cpu_0();
                    unsafe { test_thread_function() }
                })
                .expect("failed to spawn thread")
        })
        .collect::<Vec<_>>();

    for thread in threads {
        thread.join().expect("A thread panicked");
    }
}

fn pin_thread_to_cpu_0() {
    unsafe {
        let previous_mask = SetThreadAffinityMask(GetCurrentThread(), 1);
        assert_ne!(previous_mask, 0, "failed to pin thread to CPU 0");
    }
}

#[cfg(windows)]
unsafe extern "system" {
    fn GetCurrentThread() -> *mut c_void;
    fn SetThreadAffinityMask(thread: *mut c_void, thread_affinity_mask: usize) -> usize;
}

/// Set FFR, load, then read FFR
#[target_feature(enable = "sve")]
unsafe fn test_thread_function() {
    unsafe {
        svsetffr();
        let _loaded = svldff1_s32(svptrue_b32(), I32_DATA.as_ptr());
        let defined = svrdffr();
        assert!(!svptest_any(
            svptrue_b32(),
            svnot_b_z(svptrue_b32(), defined)
        ));
    }
}
