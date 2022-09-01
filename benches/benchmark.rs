use criterion::{black_box, criterion_group, criterion_main, Criterion};
use weighted_median::{weighted_median, Data};

struct BenchData {
    value: f64,
    weight: f64,
}

impl Data for BenchData {
    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

fn generate_test_data<F: Fn(usize) -> (usize, usize)>(func: F) -> Vec<BenchData> {
    let mut data = Vec::new();

    for count in 0..100 {
        let (weight, value) = func(count);
        data.push(BenchData {
            weight: weight as f64,
            value: value as f64,
        });
    }
    data
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let unsorted_test_data = &mut generate_test_data(|count| (count % 19, (count * 119) % 129))[..];
    let sorted_test_data = &mut generate_test_data(|count| (count % 2, count))[..];

    c.bench_function("weighted_median - unsorted", |b| {
        b.iter(|| weighted_median(black_box(unsorted_test_data)))
    });

    c.bench_function("weighted_median - sorted", |b| {
        b.iter(|| weighted_median(black_box(sorted_test_data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
