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

#[allow(dead_code)]
static U32_DATA: LazyLock<[u32; 64 * 5]> = LazyLock::new(|| {
    (0..64 * 5)
        .map(|i| i as u32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("u32 data incorrectly initialised")
});

#[allow(dead_code)]
static F32_DATA: LazyLock<[f32; 64 * 5]> = LazyLock::new(|| {
    (0..64 * 5)
        .map(|i| i as f32)
        .collect::<Vec<_>>()
        .try_into()
        .expect("f32 data incorrectly initialised")
});

#[test]
fn test_s32() {
    unsafe {
        test_svld1rq_s32();
    }
}
#[test]
fn test_u32() {
    unsafe {
        test_svld1rq_u32();
    }
}
#[test]
fn test_f32() {
    unsafe {
        test_svld1rq_f32();
    }
}
#[test]
fn test_u325() {
    unsafe {
        test_svld1rq_u32();
    }
}
#[test]
fn test_u324() {
    unsafe {
        test_svld1rq_u32();
    }
}
#[test]
fn test_u323() {
    unsafe {
        test_svld1rq_u32();
    }
}
#[test]
fn test_u322() {
    unsafe {
        test_svld1rq_u32();
    }
}
#[test]
fn test_u321() {
    unsafe {
        test_svld1rq_u32();
    }
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
unsafe fn test_svld1rq_s32() {
    unsafe {
        svsetffr();
        let loaded = svld1rq_s32(svptrue_b32(), I32_DATA.as_ptr());
        assert_vector_matches_i32(
            loaded,
            svdupq_n_s32(0usize as i32, 1usize as i32, 2usize as i32, 3usize as i32),
        );
    }
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
unsafe fn test_svld1rq_u32() {
    unsafe {
        svsetffr();
        let loaded = svld1rq_u32(svptrue_b32(), U32_DATA.as_ptr());
        assert_vector_matches_u32(
            loaded,
            svdupq_n_u32(0usize as u32, 1usize as u32, 2usize as u32, 3usize as u32),
        );
    }
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
unsafe fn test_svld1rq_f32() {
    unsafe {
        svsetffr();
        let loaded = svld1rq_f32(svptrue_b32(), F32_DATA.as_ptr());
        assert_vector_matches_f32(
            loaded,
            svdupq_n_f32(0usize as f32, 1usize as f32, 2usize as f32, 3usize as f32),
        );
    }
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
fn assert_vector_matches_f32(vector: svfloat32_t, expected: svfloat32_t) {
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_f32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
fn assert_vector_matches_i32(vector: svint32_t, expected: svint32_t) {
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_s32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}

#[allow(dead_code)]
#[target_feature(enable = "sve")]
fn assert_vector_matches_u32(vector: svuint32_t, expected: svuint32_t) {
    let defined = svrdffr();
    assert!(svptest_first(svptrue_b32(), defined));
    let cmp = svcmpne_u32(defined, vector, expected);
    assert!(!svptest_any(defined, cmp))
}
