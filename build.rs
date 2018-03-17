extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .files(&[
            "./zopfli/src/zopfli/blocksplitter.c",
            "./zopfli/src/zopfli/cache.c",
            "./zopfli/src/zopfli/deflate.c",
            "./zopfli/src/zopfli/gzip_container.c",
            "./zopfli/src/zopfli/hash.c",
            "./zopfli/src/zopfli/katajainen.c",
            "./zopfli/src/zopfli/lz77.c",
            "./zopfli/src/zopfli/squeeze.c",
            "./zopfli/src/zopfli/tree.c",
            "./zopfli/src/zopfli/util.c",
            "./zopfli/src/zopfli/zlib_container.c",
            "./zopfli/src/zopfli/zopfli_lib.c",
        ])
        .opt_level(2)
        .flag_if_supported("-ansi")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-c")
        .pic(true)
        .debug(true)
        .compile("zopfli");

    let bindings = bindgen::Builder::default()
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++14",
        ])
        .header("./zopfli/src/zopfli/zopfli.h")
        .whitelist_type("ZopfliOptions")
        .whitelist_var("ZopfliFormat")
        .whitelist_var("ZopfliFormat_ZOPFLI_FORMAT_DEFLATE")
        .whitelist_var("ZopfliFormat_ZOPFLI_FORMAT_GZIP")
        .whitelist_var("ZopfliFormat_ZOPFLI_FORMAT_ZLIB")
        .whitelist_function("ZopfliCompress")
        .whitelist_function("ZopfliInitOptions")
        .derive_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
