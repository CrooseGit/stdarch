#![feature(stdarch_aarch64_sve)]
use core::arch::aarch64::*;
use std::sync::LazyLock;
#[allow(dead_code)]
static I32_DATA: LazyLock<[i32; 64 * 5]> = LazyLock::new(|| {
    (0..64 * 5)
        .map(|i| i as i32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("i32 data incorrectly initialised")
});

#[test]
fn test_1() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_2() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_3() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_4() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_5() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_6() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_7() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_8() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_9() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_10() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_11() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_12() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_13() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_14() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_15() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_16() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_17() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_18() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_19() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_20() {
    unsafe {
        test_svld1rq_s32();
    }
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
unsafe fn test_svld1rq_s32() {
    unsafe {
        svsetffr();
        let _loaded = svld1rq_s32(svptrue_b32(), I32_DATA.as_ptr());
        let defined = svrdffr();
        assert!(svptest_first(svptrue_b32(), defined));
    }
}
