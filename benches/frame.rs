use raytracer::renderer;
use criterion::{
    black_box,
    criterion_main,
    criterion_group,
    Criterion
};

pub fn frame_benchmark(c: &mut Criterion) {
    let renderer : renderer::Renderer = renderer::Renderer::get_renderer_from_file(String::from("benches/bench1.json"));
    c.bench_function("bench 1 json", |b| b.iter(|| renderer.render()));
}

criterion_group!(frame, frame_benchmark);
criterion_main!(frame);
