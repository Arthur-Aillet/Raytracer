use raytracer::renderer;
use criterion::{
    black_box,
    criterion_main,
    criterion_group,
    Criterion
};

pub fn frame_benchmark1(c: &mut Criterion) {
    let renderer : renderer::Renderer = renderer::Renderer::get_renderer_from_file(String::from("benches/bench1.json"), 9, 16);
    c.bench_function("bench 1 json (16x9)", |b| b.iter(|| renderer.render()));
}

pub fn frame_benchmark2(c: &mut Criterion) {
    let renderer : renderer::Renderer = renderer::Renderer::get_renderer_from_file(String::from("benches/bench1.json"), 18, 32);
    c.bench_function("bench 2 json (32x18)", |b| b.iter(|| renderer.render()));
}

pub fn frame_benchmark3(c: &mut Criterion) {
    let renderer : renderer::Renderer = renderer::Renderer::get_renderer_from_file(String::from("benches/bench1.json"), 36, 64);
    c.bench_function("bench 3 json (64x36)", |b| b.iter(|| renderer.render()));
}

criterion_group!(frame, frame_benchmark1, frame_benchmark2, frame_benchmark3);
criterion_main!(frame);
