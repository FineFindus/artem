use std::num::NonZeroU32;

use crate::benchmarks::util;
use criterion::{criterion_group, Criterion};

/// Benchmarks with only a single thread
///
/// All other options will remain as default. This benchmark, together
/// with similar thread benchmarks ensures that an increase in threads will lead to a decrease in time spent.
fn threads_1_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("threads 1");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //change number of threads for all following images
    options.threads(NonZeroU32::new(1).unwrap());

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

/// Benchmarks with 2 threads.
///
/// All other options will remain as default. This benchmark, together
/// with similar thread benchmarks ensures that an increase in threads will lead to a decrease in time spent.
fn threads_2_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("threads 2");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //change number of threads for all following images
    options.threads(NonZeroU32::new(2).unwrap());

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

/// Benchmarks with 4 threads.
///
/// All other options will remain as default. This benchmark, together
/// with similar thread benchmarks ensures that an increase in threads will lead to a decrease in time spent.
fn threads_4_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("threads 4 (default)");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //change number of threads for all following images
    options.threads(NonZeroU32::new(4).unwrap());

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

/// Benchmarks with 6 threads.
///
/// All other options will remain as default. This benchmark, together
/// with similar thread benchmarks ensures that an increase in threads will lead to a decrease in time spent.
fn threads_6_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("threads 6");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //change number of threads for all following images
    options.threads(NonZeroU32::new(6).unwrap());

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

/// Benchmarks with 8 threads.
///
/// All other options will remain as default. This benchmark, together
/// with similar thread benchmarks ensures that an increase in threads will lead to a decrease in time spent.
fn threads_8_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("threads 8");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //change number of threads for all following images
    options.threads(NonZeroU32::new(8).unwrap());

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
    threads_1_benchmark,
    threads_2_benchmark,
    threads_4_benchmark,
    threads_6_benchmark,
    threads_8_benchmark,
);
