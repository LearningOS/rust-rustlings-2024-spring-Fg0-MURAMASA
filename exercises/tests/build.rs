//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    // 设置环境变量 TEST_FOO 为当前时间戳
    env::set_var("TEST_FOO", timestamp.to_string());
    // 7 end

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
