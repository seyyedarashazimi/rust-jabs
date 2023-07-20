use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_jabs::scenario::bitcoin_global_network_scenario::run;

pub fn bitcoin_scenario_benchmark(c: &mut Criterion) {
    c.bench_function("specs ECS speed test", |b| {
        b.iter(|| {
            run(
                black_box(600.0),
                black_box(6),
                black_box(86400.0),
                black_box(0),
            )
        })
    });
}

criterion_group!(benches, bitcoin_scenario_benchmark);
criterion_main!(benches);
