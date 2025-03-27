//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置 `TEST_FOO` 环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 监听环境变量变化，确保 Cargo 在 `TEST_FOO` 变化时重新构建
    println!("cargo:rerun-if-env-changed=TEST_FOO");

    // 在 `tests8` 里启用 "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
