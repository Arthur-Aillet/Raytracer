use raytracer::renderer;
use criterion::{
    black_box,
    criterion_main,
    criterion_group,
    Criterion
};

pub fn frame_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| print!("")));
}

criterion_group!(frame, frame_benchmark);
criterion_main!(frame);
