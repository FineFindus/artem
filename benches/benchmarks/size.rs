use std::num::NonZeroU32;

use crate::benchmarks::util;
use criterion::{criterion_group, Criterion};

/// Benchmarks for the target size of 10.
///
/// All other options will remain as default. This benchmark, together
/// with similar size benchmarks ensures that there is no gigantic and unexpected
/// performance differences for different target sizes.
fn size_10_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("size 10");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //set target size for all benches
    options.target_size(NonZeroU32::new(10).unwrap());

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

/// Benchmarks for the target size of 100.
///
/// All other options will remain as default. This benchmark, together
/// with similar size benchmarks ensures that there is no gigantic and unexpected
/// performance differences for different target sizes.
fn size_100_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("size 100");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //set target size for all benches
    options.target_size(NonZeroU32::new(100).unwrap());

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

/// Benchmarks for the target size of 500.
///
/// All other options will remain as default. This benchmark, together
/// with similar size benchmarks ensures that there is no gigantic and unexpected
/// performance differences for different target sizes.
fn size_500_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("size 500");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //set target size for all benches
    options.target_size(NonZeroU32::new(500).unwrap());

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

criterion_group!(
    benches,
    size_10_benchmark,
    size_100_benchmark,
    size_500_benchmark
);
