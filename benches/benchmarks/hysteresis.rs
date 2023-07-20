use crate::benchmarks::util;
use criterion::{criterion_group, Criterion};

/// Benchmarks for outlining an image with hysteresis.
fn hysteresis_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hysteresis");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::config::ConfigBuilder::new();
    //need to have outline enabled
    options.outline(true);
    //enable hysteresis
    options.hysteresis(true);

    //test on different resolutions

    group.bench_function("low resolution", |b| {
        b.iter_batched(
            util::load_low_res_image,
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("normal resolution", |b| {
        b.iter_batched(
            util::load_normal_res_image,
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("high resolution", |b| {
        b.iter_batched(
            util::load_high_res_image,
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, hysteresis_benchmark);
