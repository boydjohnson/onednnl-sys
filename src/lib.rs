#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "opencl-gpu-runtime")]
pub mod ocl;
#[cfg(feature = "sycl-gpu-runtime")]
pub mod sycl;
