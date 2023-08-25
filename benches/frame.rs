use criterion::{criterion_group, criterion_main, Criterion};
use raytracer::{config::Config, renderer};

pub fn frame_benchmark(c: &mut Criterion) {
    let config = Config::from_args(&[
        "-w".to_string(),
        "40".to_string(),
        "-h".to_string(),
        "40".to_string(),
    ]);
    let renderer = renderer::Renderer::get_renderer_from_file(&config).unwrap();
    c.bench_function("bench 3 json (64x36)", |b| {
        b.iter(|| renderer.pull_new_image(&config))
    });
}
criterion_group!(frame, frame_benchmark);
criterion_main!(frame);
