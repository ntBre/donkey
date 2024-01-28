use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bind = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("PI")
        .blocklist_item("M_PI")
        .blocklist_item("M_PI_2")
        .blocklist_item("M_PI_4")
        .blocklist_item("M_1_PI")
        .blocklist_item("M_2_PI")
        .blocklist_item("M_2_SQRTPI")
        .blocklist_item("M_SQRT2")
        .blocklist_item("M_SQRT1_2")
        .blocklist_item("M_E")
        .blocklist_item("M_LOG2E")
        .blocklist_item("M_LOG10E")
        .blocklist_item("M_LN2")
        .blocklist_item("M_LN10")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bind.write_to_file(out_path.join("bindings.rs")).unwrap();
}
