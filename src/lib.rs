mod partition;
use partition::partition;
mod is_sorted;
use is_sorted::is_sorted;

pub trait Data {
    fn get_value(&self) -> f64;
    fn get_weight(&self) -> f64;
}

#[inline]
fn weight_sum<T: Data>(input: &mut [T]) -> f64 {
    input
        .into_iter()
        .fold(0.0, |accum, item| accum + item.get_weight())
}

pub fn calculate<T: Data>(
    data: &mut [T],
    lower_weight_delta: f64,
    higher_weight_delta: f64,
    previous_data_is_sorted: bool,
) -> f64 {
    let data_is_sorted = previous_data_is_sorted || is_sorted(data);
    match data.len() {
        1 => data[0].get_value(),
        2 => {
            let lower = lower_weight_delta + data[0].get_weight();
            let higher = data[1].get_weight() + higher_weight_delta;
            if lower == higher {
                (data[0].get_value() + data[1].get_value()) / 2.0
            } else if lower > higher {
                data[0].get_value()
            } else {
                data[1].get_value()
            }
        }
        _ => {
            let pivot_index = match data_is_sorted {
                true => data.len() / 2,
                false => partition(data),
            };

            let lower_weight_sum = lower_weight_delta + weight_sum(&mut data[..pivot_index]);
            let higher_weight_sum = higher_weight_delta + weight_sum(&mut data[pivot_index + 1..]);
            let weight_sum = lower_weight_sum + data[pivot_index].get_weight() + higher_weight_sum;

            if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
                data[pivot_index].get_value()
            } else if lower_weight_sum / weight_sum >= 0.5 {
                let (next_data, _) = data.split_at_mut(pivot_index + 1);
                calculate(
                    next_data,
                    lower_weight_delta,
                    higher_weight_sum,
                    data_is_sorted,
                )
            } else {
                let next_data = &mut data[pivot_index..];
                calculate(
                    next_data,
                    lower_weight_sum,
                    higher_weight_delta,
                    data_is_sorted,
                )
            }
        }
    }
}

#[inline]
pub fn weighted_median<T: Data>(data: &mut [T]) -> f64 {
    calculate(data, 0.0, 0.0, false)
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
}
