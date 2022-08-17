use criterion::{black_box, criterion_group, criterion_main, Criterion};
use weighted_median::{weighted_median, Data};

struct TestData {
    weight: f64,
    value: f64,
}

impl Data for TestData {
    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}
fn generate_test_data() -> Vec<TestData>{
    let mut data = Vec::<TestData>::new();

    for count in 0..100 {
        data.push(TestData {
            weight: (count % 19) as f64,
            value: ((count * 119) % 129) as f64,
        });
    }
    data
}

pub fn criterion_benchmark(c: &mut Criterion) {
  let test_data = &mut generate_test_data()[..];


  c.bench_function("weighted_median", |b| b.iter(|| weighted_median(black_box(test_data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
