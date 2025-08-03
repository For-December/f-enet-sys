use std::{env, path::PathBuf};

fn main() {
    // 告诉 Cargo 何时重新运行构建脚本
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=vendor/enet");

    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include")
        .header("wrapper.h")
        .derive_debug(false)
        .blocklist_type("ENetPacket") // 这两个类型需要外部使用
        .blocklist_type("_ENetPacket")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 写入绑定文件
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // 构建 ENet 库
    let dst = cmake::Config::new("vendor/enet")
        .define("BUILD_SHARED_LIBS", "OFF") // 强制构建静态库
        .define("CMAKE_BUILD_TYPE", "Release") // 优化构建
        .build();

    // Windows 特定链接
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=winmm");
    }

    // 设置链接参数
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=enet");
}
