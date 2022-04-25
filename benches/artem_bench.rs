use criterion::criterion_main;

//import benchmarks
mod benchmarks;

criterion_main!(
    //without any options set
    benchmarks::default::benches,
    //using different threads
    benchmarks::threads::benches,
    //different size options
    benchmarks::size::benches,
    //using the outline algorithm
    benchmarks::outline::benches,
    //using the outline algorithm with hysteresis and double threshold
    benchmarks::hysteresis::benches,
);
