use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .constified_enum_module("dnnl_accumulation_mode_t")
        .constified_enum_module("dnnl_alg_kind_t")
        .constified_enum_module("dnnl_cpu_isa_hints_t")
        .constified_enum_module("dnnl_cpu_isa_t")
        .constified_enum_module("dnnl_data_type_t")
        .constified_enum_module("dnnl_.*")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
