use crate::benchmarks::util;
use criterion::{criterion_group, Criterion};

/// Benchmarks for the default options.
///
/// The default options can be viewed at [OptionBuilder::default()], in short
/// it will use 1 thread, a target size of 80 and a scale of 0.42 as well as the
/// default density.
fn default_options_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("default options");

    let options = artem::options::OptionBuilder::new();

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    //test on different resolutions

    group.bench_function("low resolution", |b| {
        b.iter_batched(
            || util::load_low_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("normal resolution", |b| {
        b.iter_batched(
            || util::load_normal_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("high resolution", |b| {
        b.iter_batched(
            || util::load_high_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, default_options_benchmark);
