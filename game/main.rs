fn main() {
    pollster::block_on(wgpu_wasm::start());
    // pollster::block_on(wgpu_wasm::rtt_test_run());
}
