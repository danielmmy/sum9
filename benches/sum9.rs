use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Uniform, prelude::Distribution};
use std::{hint::black_box, sync::OnceLock};
use sum9::{
    sum9_bruteforce, sum9_faster_map, sum9_good_multiplier_hash_i32_map,
    sum9_good_multiplier_hash_map, sum9_map, sum9_no_hash_map, sum9_prealloc_vec,
};

const RAND_TEST_DATA_SIZE: usize = 10_000;

fn rand_test_data() -> &'static Vec<i32> {
    static INSTANCE: OnceLock<Vec<i32>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let range: Uniform<i32> = Uniform::new(10, 200_000);
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = Vec::with_capacity(RAND_TEST_DATA_SIZE);
        for _ in 0..RAND_TEST_DATA_SIZE {
            data.push(range.sample(&mut rng));
        }

        data
    })
}

/**********************************************PREALLOC VEC TESTS*************************************************/
fn sum9_prealloc_vec_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Prealloc Vec rand test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(data))));
}

fn sum9_prealloc_vec_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Prealloc Vec test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(&data))));
}

fn sum9_prealloc_vec_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Prealloc Vec test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(&data))));
}

fn sum9_prealloc_vec_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Prealloc Vec test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(&data))));
}

fn sum9_prealloc_vec_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Prealloc Vec test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(&data))));
}

fn sum9_prealloc_vec_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Prealloc Vec test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_prealloc_vec(black_box(&data))));
}
/**********************************************END PREALLOC VEC TESTS*************************************************/

/**********************************************MAP TESTS*************************************************/
fn sum9_map_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Map rand test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_map(black_box(data))));
}

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

/**********************************************FASTER MAP TESTS*************************************************/
fn sum9_faster_map_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Faster Map rand test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(data))));
}

fn sum9_faster_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Faster Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(&data))));
}

fn sum9_faster_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Faster Map test {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(&data))));
}

fn sum9_faster_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Faster Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(&data))));
}

fn sum9_faster_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Faster Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(&data))));
}

fn sum9_faster_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Faster Map test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_faster_map(black_box(&data))));
}
/**********************************************END FASTER MAP TESTS*************************************************/

/**********************************************GOOD MULTIPLIER MAP TESTS*************************************************/
fn sum9_good_multiplier_hash_map_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Good Multiplier Map rand test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(data)))
    });
}

fn sum9_good_multiplier_hash_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Good Multiplier Map test {:?}", data);
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Good Multiplier Map test {:?}", data);
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_map(black_box(&data)))
    });
}
/**********************************************END GOOD MULTIPLIER MAP TESTS*************************************************/

/**********************************************GOOD MULTIPLIER I32 MAP TESTS*************************************************/
fn sum9_good_multiplier_hash_i32_map_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Good Multiplier I32 Map rand test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(data)))
    });
}

fn sum9_good_multiplier_hash_i32_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("Good Multiplier I32 Map  {:?}", data);
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_i32_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("Good Multiplier I32 Map  {:?}", data);
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_i32_map_benchmark3(c: &mut Criterion) {
    let test_size = 1_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier I32 Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_i32_map_benchmark4(c: &mut Criterion) {
    let test_size = 10_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier I32 Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(&data)))
    });
}

fn sum9_good_multiplier_hash_i32_map_benchmark5(c: &mut Criterion) {
    let test_size = 100_000;
    let mut data: Vec<i32> = Vec::with_capacity(test_size);
    for i in 0..test_size {
        data.push(10 + i as i32);
    }
    let name = format!("Good Multiplier I32 Map test size {}", data.len());
    c.bench_function(&name, |b| {
        b.iter(|| sum9_good_multiplier_hash_i32_map(black_box(&data)))
    });
}
/**********************************************END GOOD MULTIPLIER I32 MAP TESTS*************************************************/

/**********************************************NO HASH MAP TESTS*************************************************/
fn sum9_no_hash_map_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("No Hash Map rand test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(data))));
}

fn sum9_no_hash_map_benchmark1(c: &mut Criterion) {
    let data = vec![2, 7, 11, 15];
    let name = format!("No Hash Map  {:?}", data);
    c.bench_function(&name, |b| b.iter(|| sum9_no_hash_map(black_box(&data))));
}

fn sum9_no_hash_map_benchmark2(c: &mut Criterion) {
    let data = vec![11, 15, 2, 7];
    let name = format!("No Hash Map  {:?}", data);
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

/**********************************************BRUTEFORCE TESTS*************************************************/
fn sum9_bruteforce_benchmark0(c: &mut Criterion) {
    let data = rand_test_data();
    let name = format!("Bruteforce rand test size {}", data.len());
    c.bench_function(&name, |b| b.iter(|| sum9_bruteforce(black_box(data))));
}

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
    sum9_prealloc_vec_benchmark0,
    sum9_map_benchmark0,
    sum9_faster_map_benchmark0,
    sum9_good_multiplier_hash_map_benchmark0,
    sum9_good_multiplier_hash_i32_map_benchmark0,
    sum9_no_hash_map_benchmark0,
    sum9_bruteforce_benchmark0,
    sum9_prealloc_vec_benchmark1,
    sum9_map_benchmark1,
    sum9_faster_map_benchmark1,
    sum9_good_multiplier_hash_map_benchmark1,
    sum9_good_multiplier_hash_i32_map_benchmark1,
    sum9_no_hash_map_benchmark1,
    sum9_bruteforce_benchmark1,
    sum9_prealloc_vec_benchmark2,
    sum9_map_benchmark2,
    sum9_faster_map_benchmark2,
    sum9_good_multiplier_hash_map_benchmark2,
    sum9_good_multiplier_hash_i32_map_benchmark2,
    sum9_no_hash_map_benchmark2,
    sum9_bruteforce_benchmark2,
    sum9_prealloc_vec_benchmark3,
    sum9_map_benchmark3,
    sum9_faster_map_benchmark3,
    sum9_good_multiplier_hash_map_benchmark3,
    sum9_good_multiplier_hash_i32_map_benchmark3,
    sum9_no_hash_map_benchmark3,
    sum9_bruteforce_benchmark3,
    sum9_prealloc_vec_benchmark4,
    sum9_map_benchmark4,
    sum9_faster_map_benchmark4,
    sum9_good_multiplier_hash_map_benchmark4,
    sum9_good_multiplier_hash_i32_map_benchmark4,
    sum9_no_hash_map_benchmark4,
    sum9_bruteforce_benchmark4,
    sum9_prealloc_vec_benchmark5,
    sum9_map_benchmark5,
    sum9_faster_map_benchmark5,
    sum9_good_multiplier_hash_map_benchmark5,
    sum9_good_multiplier_hash_i32_map_benchmark5,
    sum9_no_hash_map_benchmark5,
    sum9_bruteforce_benchmark5,
);
criterion_main!(benches);
