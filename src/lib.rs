#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "bindings"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(feature = "opencl-gpu-runtime", not(feature = "bindings")))]
pub mod ocl;
#[cfg(all(feature = "sycl-gpu-runtime", not(feature = "bindings")))]
pub mod sycl;

#[cfg(feature = "bindings")]
mod bindings;

#[cfg(feature = "bindings")]
pub use bindings::bindings::*;

#[cfg(all(feature = "opencl-gpu-runtime", feature = "bindings"))]
pub use bindings::ocl_bindings as ocl;

#[cfg(all(feature = "sycl-gpu-runtime", feature = "bindings"))]
pub use bindings::sycl_bindings as sycl;
