
use criterion::{criterion_group, criterion_main, Criterion};
use rust_fizzbuzz::{fizzbuzz_kronke, fizzbuzz_kronke_branchless};
use rust_fizzbuzz::fizzbuzz_kronke_inverted;
use rust_fizzbuzz::fizzbuzz_lsitarski;
use std::time;

pub fn fizzbuzz(c: &mut Criterion) {
    let mut group = c.benchmark_group("FizzBuzz");
    group.sample_size(1000);
    group.warm_up_time(time::Duration::from_secs(10));
    group.bench_function("fizzbuzz lsitarski", |b| b.iter(||  {
        for i in 0..60000 {
            fizzbuzz_lsitarski(i);
        }
    }));
    
    group.bench_function("fizzbuzz kronke", |b| b.iter(||  {
        for i in 0..60000 {
            fizzbuzz_kronke(i);
        }
    }));
    group.bench_function("fizzbuzz kronke inverted", |b| b.iter(||  {
        for i in 0..60000 {
            fizzbuzz_kronke_inverted(i);
        }
    }));
    group.bench_function("fizzbuzz kronke branchless", |b| b.iter(||  {
        for i in 0..60000 {
            fizzbuzz_kronke_branchless(i);
        }
    }));


    group.finish();
}

criterion_group!(benches, fizzbuzz);
criterion_main!(benches);
