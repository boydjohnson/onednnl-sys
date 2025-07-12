use std::{env, path::PathBuf};

fn main() {
    if !cfg!(feature = "bindings") {
        if let Ok(_) = pkg_config::probe_library("dnnl") {
            let bindings = bindgen::Builder::default()
                .header("wrapper.h")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .allowlist_item("dnnl_.*|DNNL_.*|BUILD_.*")
                .constified_enum_module("dnnl_.*")
                .generate_comments(true)
                .clang_arg("-fretain-comments-from-system-headers")
                .generate()
                .unwrap();

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");

            if cfg!(feature = "opencl-gpu-runtime") {
                let bindings = bindgen::Builder::default()
                .header("wrapper_cl_interop.h")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .allowlist_item("dnnl_ocl.*")
                .allowlist_var("CL_.*")
                .constified_enum_module("dnnl_.*")
                .allowlist_function(
                    "cl.*Context|clGetPlatform.*|clGetDevice.*|cl.*CommandQueue|cl.*Buffer|clCreateImage2D|clCreateImage3D",
                )
                .raw_line("use crate::*;")
                .blocklist_type("dnnl_engine_t|dnnl_engine|dnnl_status_t|dnnl_memory|dnnl_memory_t|dnnl_primitive|const_dnnl_primitive_t|dnnl_exec_arg_t|dnnl_memory_desc|const_dnnl_memory_desc_t|dnnl_stream|dnnl_stream_t|const_dnnl_memory_t")
                .generate_comments(true)
                .clang_arg("-fretain-comments-from-system-headers")
                .clang_arg("-I/opt/intel/oneapi/compiler/latest/include/sycl")
                .generate()
                .unwrap();

                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                bindings
                    .write_to_file(out_path.join("ocl_bindings.rs"))
                    .expect("Couldn't write bindings!");
                println!("cargo:rustc-link-lib=OpenCL");
            }

            if cfg!(feature = "sycl-gpu-runtime") {
                let bindings = bindgen::Builder::default()
                .header("wrapper_sycl_interop.h")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .allowlist_item("dnnl_sycl.*")
                .constified_enum_module("dnnl_.*")
                .blocklist_type("dnnl_engine_t|dnnl_engine|dnnl_status_t|dnnl_memory|dnnl_memory_t|dnnl_primitive|const_dnnl_primitive_t|dnnl_exec_arg_t|dnnl_memory_desc|const_dnnl_memory_desc_t|dnnl_stream|dnnl_stream_t|const_dnnl_memory_t")
                .generate_comments(true)
                .raw_line("use crate::*;")
                .clang_arg("-fretain-comments-from-system-headers")
                .clang_arg("-I/opt/intel/oneapi/compiler/latest/include/sycl")
                .generate()
                .unwrap();

                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                bindings
                    .write_to_file(out_path.join("sycl_bindings.rs"))
                    .expect("Couldn't write bindings!");
            }
        } else {
            todo!("Build static oneDNN");
        }
    }

    println!("cargo:rustc-link-lib=dnnl");
}
