use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use sum9::sum9_bruteforce;

fn sum9_bruteforce_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("{:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("{:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark3(c: &mut Criterion) {
    let test_size = 1000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

criterion_group!(
    benches,
    sum9_bruteforce_benchmark1,
    sum9_bruteforce_benchmark2,
    sum9_bruteforce_benchmark3
);
criterion_main!(benches);
