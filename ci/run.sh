#!/usr/bin/env sh

set -ex

: "${TARGET?The TARGET environment variable must be set.}"

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
#export RUST_BACKTRACE=full
#export RUST_TEST_NOCAPTURE=1
#export RUST_TEST_THREADS=1

export RUSTFLAGS="${RUSTFLAGS} -D warnings -Z merge-functions=disabled -Z verify-llvm-ir"
export HOST_RUSTFLAGS="${RUSTFLAGS}"
export PROFILE="${PROFILE:="release"}"

case ${TARGET} in
    # On Windows the linker performs identical COMDAT folding (ICF) by default
    # in release mode which removes identical COMDAT sections. This interferes
    # with our instruction assertions just like LLVM's MergeFunctions pass so
    # we disable it.
    *-pc-windows-msvc)
        export RUSTFLAGS="${RUSTFLAGS} -Clink-args=/OPT:NOICF"
        ;;
    # On 32-bit use a static relocation model which avoids some extra
    # instructions when dealing with static data, notably allowing some
    # instruction assertion checks to pass below the 20 instruction limit. If
    # this is the default, dynamic, then too many instructions are generated
    # when we assert the instruction for a function and it causes tests to fail.
    i686-* | i586-*)
        export RUSTFLAGS="${RUSTFLAGS} -C relocation-model=static"
        ;;
    # Some x86_64 targets enable by default more features beyond SSE2,
    # which cause some instruction assertion checks to fail.
    x86_64-*)
        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=-sse3"
        ;;
    #Unoptimized build uses fast-isel which breaks with msa
    mips-* | mipsel-*)
	export RUSTFLAGS="${RUSTFLAGS} -C llvm-args=-fast-isel=false"
	;;
    armv7-*eabihf | thumbv7-*eabihf)
        export RUSTFLAGS="${RUSTFLAGS} -Ctarget-feature=+neon,+fp16"
        ;;
    amdgcn-*)
        export RUSTFLAGS="${RUSTFLAGS} -Ctarget-cpu=gfx1200"
        ;;
    # Some of our test dependencies use the deprecated `gcc` crates which
    # doesn't detect RISC-V compilers automatically, so do it manually here.
    riscv*)
        export RUSTFLAGS="${RUSTFLAGS} -Ctarget-feature=+zk,+zks,+zbb,+zbc"
        ;;
    hexagon*)
        export RUSTFLAGS="${RUSTFLAGS} -Ctarget-feature=+hvxv60,+hvx-length128b"
        ;;
esac

echo "RUSTFLAGS=${RUSTFLAGS}"
echo "OBJDUMP=${OBJDUMP}"
echo "STDARCH_DISABLE_ASSERT_INSTR=${STDARCH_DISABLE_ASSERT_INSTR}"
echo "STDARCH_TEST_EVERYTHING=${STDARCH_TEST_EVERYTHING}"
echo "STDARCH_TEST_SKIP_FEATURE=${STDARCH_TEST_SKIP_FEATURE}"
echo "STDARCH_TEST_SKIP_FUNCTION=${STDARCH_TEST_SKIP_FUNCTION}"
echo "PROFILE=${PROFILE}"

cargo_run() {
    cmd="cargo"
    subcmd="run"
    cmd="$cmd ${subcmd} --target=$TARGET --profile=$PROFILE $1"
    cmd="$cmd -- $2"

   $cmd
}

cargo_test() {
    cmd="cargo"
    subcmd="test"
    cmd="$cmd ${subcmd} --target=$TARGET --profile=$PROFILE $1"
    cmd="$cmd -- $2"

   $cmd
}

CORE_ARCH="--manifest-path=crates/core_arch/Cargo.toml"
STDARCH_EXAMPLES="--manifest-path=examples/Cargo.toml"
MIN_REPR="--manifest-path=minrepr/Cargo.toml"
THREAD_REPR="--manifest-path=threadrepr/Cargo.toml"

for i in {1..40}; do
  cargo_run "${THREAD_REPR}" 
done


