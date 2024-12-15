use onednnl_sys::{
    dnnl_data_type_t, dnnl_engine_create, dnnl_engine_destroy, dnnl_engine_kind_t, dnnl_engine_t,
    dnnl_format_tag_t, dnnl_memory_desc_create_with_tag, dnnl_memory_desc_t, dnnl_status_t,
    dnnl_stream_create, dnnl_stream_destroy, dnnl_stream_flags_t, dnnl_stream_t,
};

#[test]
fn test_engine_stream_memory_descriptor_smoke() {
    // Initialize the engine and stream for DNNL
    let mut engine: dnnl_engine_t = std::ptr::null_mut();
    let mut status = unsafe { dnnl_engine_create(&mut engine, dnnl_engine_kind_t::dnnl_cpu, 0) };
    assert_eq!(
        status,
        dnnl_status_t::dnnl_success,
        "Failed to create DNNL engine"
    );

    let mut stream: dnnl_stream_t = std::ptr::null_mut();
    status = unsafe {
        dnnl_stream_create(
            &mut stream,
            engine,
            dnnl_stream_flags_t::dnnl_stream_default_flags,
        )
    };
    assert_eq!(
        status,
        dnnl_status_t::dnnl_success,
        "Failed to create DNNL stream"
    );

    // Create a memory descriptor
    let dims: [i64; 4] = [1, 1, 28, 28]; // Example for a 28x28 single-channel image
    let mut mem_desc: dnnl_memory_desc_t = unsafe { std::mem::zeroed() };
    status = unsafe {
        dnnl_memory_desc_create_with_tag(
            &mut mem_desc,
            dims.len() as i32,
            dims.as_ptr(),
            dnnl_data_type_t::dnnl_f32,
            dnnl_format_tag_t::dnnl_nchw,
        )
    };
    assert_eq!(
        status,
        dnnl_status_t::dnnl_success,
        "Failed to create memory descriptor"
    );

    // Clean up resources
    unsafe {
        dnnl_stream_destroy(stream);
        dnnl_engine_destroy(engine);
    }
}
