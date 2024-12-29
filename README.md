# oneAPI oneDNN Rust -sys bindings

[![Build Status](https://github.com/boydjohnson/onednn-sycl-sys/actions/workflows/ci-intel.yml/badge.svg)](https://github.com/boydjohnson/onednn-sycl-sys/actions/workflows/ci-intel.yml)

high-level bindings at [onednnl-rs](https://github.com/boydjohnson/onednnl-rs)

## build

```bash
source /opt/intel/oneapi/setvars.sh
cargo build
cargo build --all-features
```

## test

```bash
source /opt/intel/oneapi/setvars.sh
cargo test
cargo test --all-features
```

## docs
```bash
source /opt/intel/oneapi/setvars.sh
cargo doc
cargo doc --all-features
```
