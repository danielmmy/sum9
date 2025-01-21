use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use sum9::{sum9_bruteforce, sum9_map, sum9_no_hash_i32_map, sum9_no_hash_map};

/**********************************************MAP TESTS*************************************************/
fn sum9_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(&data))));
}

fn sum9_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(&data))));
}

fn sum9_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(&data))));
}

fn sum9_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(&data))));
}

fn sum9_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(&data))));
}
/**********************************************END MAP TESTS*************************************************/

/**********************************************NO HASH MAP TESTS*************************************************/
fn sum9_no_hash_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("No Hash Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}

fn sum9_no_hash_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("No Hash Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}

fn sum9_no_hash_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}

fn sum9_no_hash_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}

fn sum9_no_hash_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}
/**********************************************END NO HASH MAP TESTS*************************************************/

/**********************************************NO HASH I32 MAP TESTS*************************************************/
fn sum9_no_hash_i32_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("No Hash I32 Map  {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_i32_map(black_box(&data))));
}

fn sum9_no_hash_i32_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("No Hash I32 Map  {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_i32_map(black_box(&data))));
}

fn sum9_no_hash_i32_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash I32 Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_i32_map(black_box(&data))));
}

fn sum9_no_hash_i32_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash I32 Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_i32_map(black_box(&data))));
}

fn sum9_no_hash_i32_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("No Hash I32 Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_i32_map(black_box(&data))));
}
/**********************************************END NO HASH I32 MAP TESTS*************************************************/

/**********************************************BRUTEFORCE TESTS*************************************************/
fn sum9_bruteforce_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Bruteforce test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Bruteforce test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Bruteforce test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Bruteforce test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}

fn sum9_bruteforce_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Bruteforce test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(&data))));
}
/**********************************************END BRUTEFORCE TESTS*************************************************/

criterion_group!(
    benches,
    sum9_map_benchmark1,
    sum9_no_hash_map_benchmark1,
    sum9_no_hash_i32_map_benchmark1,
    sum9_bruteforce_benchmark1,
    sum9_map_benchmark2,
    sum9_no_hash_map_benchmark2,
    sum9_no_hash_i32_map_benchmark2,
    sum9_bruteforce_benchmark2,
    sum9_map_benchmark3,
    sum9_no_hash_map_benchmark3,
    sum9_no_hash_i32_map_benchmark3,
    sum9_bruteforce_benchmark3,
    sum9_map_benchmark4,
    sum9_no_hash_map_benchmark4,
    sum9_no_hash_i32_map_benchmark4,
    sum9_bruteforce_benchmark4,
    sum9_map_benchmark5,
    sum9_no_hash_map_benchmark5,
    sum9_no_hash_i32_map_benchmark5,
    sum9_bruteforce_benchmark5,
);
criterion_main!(benches);
