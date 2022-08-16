use is_sorted::IsSorted;
mod partition;

pub trait Data {
    fn get_value(&self) -> f64;
    fn get_weight(&self) -> f64;
}

fn weight_sum<T: Data>(input: &mut [T]) -> f64 {
    input
        .into_iter()
        .fold(0.0, |accum, item| accum + item.get_weight())
}

struct WeightedMedian<'slice, T: Data> {
    data: &'slice mut [T],
    lower_weight_delta: f64,
    higher_weight_delta: f64,
}

impl<'slice, T: Data> WeightedMedian<'slice, T> {
    fn new(data: &'slice mut [T], lower_weight_delta: f64, higher_weight_delta: f64) -> Self {
        Self {
            data,
            lower_weight_delta,
            higher_weight_delta,
        }
    }

    fn calculate_sorted(self) -> f64 {
        let sum: f64 = self.lower_weight_delta + self.higher_weight_delta + weight_sum(self.data);
        let mut current_weight = self.lower_weight_delta;
        let mut iterator = self.data.iter();

        loop {
            match iterator.next() {
                Some(row) => {
                    current_weight = current_weight + row.get_weight();

                    if current_weight / sum == 0.5 {
                        break (row.get_value() + iterator.next().unwrap().get_value()) / 2.0;
                    } else if current_weight / sum > 0.5 {
                        break row.get_value();
                    }
                }
                None => panic!(),
            }
        }
    }

    fn calculate_not_sorted(self) -> f64 {
        let pivot_index = partition::partition(self.data);

        let lower_weight_sum = self.lower_weight_delta + weight_sum(&mut self.data[..pivot_index]);
        let higher_weight_sum =
            self.higher_weight_delta + weight_sum(&mut self.data[pivot_index + 1..]);
        let weight_sum = lower_weight_sum + self.data[pivot_index].get_weight() + higher_weight_sum;

        if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
            self.data[pivot_index].get_value()
        } else if lower_weight_sum / weight_sum >= 0.5 {
            WeightedMedian::new(
                &mut self.data[..pivot_index + 1],
                self.lower_weight_delta,
                higher_weight_sum,
            )
            .calculate()
        } else {
            WeightedMedian::new(
                &mut self.data[pivot_index..],
                lower_weight_sum,
                self.higher_weight_delta,
            )
            .calculate()
        }
    }

    fn calculate(self) -> f64 {
        let n = self.data.len();

        if n == 1 {
            self.data[0].get_value()
        } else if n == 2 {
            if self.lower_weight_delta + self.data[0].get_weight()
                == self.data[1].get_weight() + self.higher_weight_delta
            {
                (self.data[0].get_value() + self.data[1].get_value()) / 2.0
            } else if self.data[0].get_weight() > self.data[1].get_weight() {
                self.data[0].get_value()
            } else {
                self.data[1].get_value()
            }
        } else if IsSorted::is_sorted_by_key(&mut self.data.into_iter(), |data| data.get_value()) {
            self.calculate_sorted()
        } else {
            self.calculate_not_sorted()
        }
    }
}

pub fn weighted_median<T: Data>(input: &mut [T]) -> f64 {
    WeightedMedian::new(input, 0.0, 0.0).calculate()
}

#[cfg(test)]
mod tests {
    use crate::{weighted_median, Data};

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

    #[test]
    fn one_element() {
        assert_eq!(
            weighted_median(&mut [TestData {
                value: 7.0,
                weight: 9.0
            }]),
            7.0
        );
    }

    #[test]
    fn two_elements_different_weight() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 7.0,
                    weight: 1.0
                },
                TestData {
                    value: 8.0,
                    weight: 2.0
                }
            ]),
            8.0
        );
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 8.0,
                    weight: 2.0
                },
                TestData {
                    value: 7.0,
                    weight: 1.0
                },
            ]),
            8.0
        );
    }

    #[test]
    fn two_elements_same_weight() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 7.0,
                    weight: 1.0
                },
                TestData {
                    value: 8.0,
                    weight: 1.0
                }
            ]),
            7.5
        )
    }

    #[test]
    fn three_elements_is_first_element() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
                TestData {
                    value: 3.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
            ]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_middle_element() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 3.0,
                    weight: 1.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
            ]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_last_element() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 3.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
            ]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_smallest_element() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 3.0,
                    weight: 1.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 5.0
                },
            ]),
            1.0
        )
    }

    #[test]
    fn three_elements_is_biggest_element() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 3.0,
                    weight: 5.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
            ]),
            3.0
        )
    }

    #[test]
    fn three_elements_is_even() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 3.0,
                    weight: 2.0,
                },
                TestData {
                    value: 2.0,
                    weight: 1.0,
                },
                TestData {
                    value: 1.0,
                    weight: 1.0,
                },
            ]),
            2.5
        );
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 1.0,
                    weight: 1.0,
                },
                TestData {
                    value: 2.0,
                    weight: 1.0,
                },
                TestData {
                    value: 3.0,
                    weight: 2.0,
                },
            ]),
            2.5
        );
    }

    #[test]
    fn four_elements_is_even() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 1.0,
                    weight: 0.49,
                },
                TestData {
                    value: 2.0,
                    weight: 0.01,
                },
                TestData {
                    value: 3.0,
                    weight: 0.25,
                },
                TestData {
                    value: 1000.0,
                    weight: 0.25,
                },
            ]),
            2.5
        );
    }

    #[test]
    fn five_elements_is_pivot_value() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 2.0,
                    weight: 0.5
                },
                TestData {
                    value: 1.0,
                    weight: 0.5
                },
                TestData {
                    value: 3.0,
                    weight: 1.0
                },
                TestData {
                    value: 10.0,
                    weight: 0.8
                },
                TestData {
                    value: 8.0,
                    weight: 0.2
                }
            ]),
            3.0
        );
    }

    #[test]
    fn custom_input_impl_trait() {
        struct CustomTestData(f64, f64);

        impl Data for CustomTestData {
            fn get_value(&self) -> f64 {
                self.0
            }

            fn get_weight(&self) -> f64 {
                self.1
            }
        }

        assert_eq!(
            weighted_median(&mut [CustomTestData(1.0, 1.0), CustomTestData(2.0, 1.0)]),
            1.5
        );
    }

    mod sorted {
        use crate::{tests::TestData, WeightedMedian};

        #[test]
        fn with_lower_and_higher_delta() {
            assert_eq!(
                WeightedMedian::new(
                    &mut [
                        TestData {
                            value: 1.0,
                            weight: 1.0
                        },
                        TestData {
                            value: 2.0,
                            weight: 2.0
                        }
                    ],
                    1.0,
                    0.0
                )
                .calculate_sorted(),
                1.5
            );

            assert_eq!(
                WeightedMedian::new(
                    &mut [
                        TestData {
                            value: 1.0,
                            weight: 2.0
                        },
                        TestData {
                            value: 2.0,
                            weight: 1.0
                        }
                    ],
                    0.0,
                    1.0
                )
                .calculate_sorted(),
                1.5
            )
        }
    }
}
