use criterion::{criterion_group, criterion_main, Criterion};
use rust_jabs::tests::ecs_speed_test::ecs_test;

pub fn event_speed(c: &mut Criterion) {
    c.bench_function("specs ECS speed test", |b| b.iter(ecs_test));
}

criterion_group!(benches, event_speed);
criterion_main!(benches);
