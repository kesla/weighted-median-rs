mod partition;
use partition::partition;

pub trait Data {
    fn get_value(&self) -> f64;
    fn get_weight(&self) -> f64;
    fn set_weight(&mut self, new_weight: f64);
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
) -> Option<f64> {
    match data.len() {
        0 => None,
        1 => Some(data[0].get_value()),
        2 => {
            let lower = lower_weight_delta + data[0].get_weight();
            let higher = data[1].get_weight() + higher_weight_delta;
            if lower == higher {
                Some((data[0].get_value() + data[1].get_value()) / 2.0)
            } else if lower > higher {
                Some(data[0].get_value())
            } else {
                Some(data[1].get_value())
            }
        }
        _ => {
            let (pivot_index, new_data) = partition(data, data.len() / 2);

            let lower_weight_sum = lower_weight_delta + weight_sum(&mut new_data[..pivot_index]);
            let higher_weight_sum =
                higher_weight_delta + weight_sum(&mut new_data[pivot_index + 1..]);
            let weight_sum =
                lower_weight_sum + new_data[pivot_index].get_weight() + higher_weight_sum;

            if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
                Some(new_data[pivot_index].get_value())
            } else if lower_weight_sum / weight_sum >= 0.5 {
                let next_data = &mut new_data[..pivot_index + 1];
                calculate(next_data, lower_weight_delta, higher_weight_sum)
            } else {
                let next_data = &mut new_data[pivot_index..];
                calculate(next_data, lower_weight_sum, higher_weight_delta)
            }
        }
    }
}

#[inline]
pub fn weighted_median<T: Data>(data: &mut [T]) -> Option<f64> {
    calculate(data, 0.0, 0.0)
}

#[cfg(test)]
mod tests {
    use crate::{weighted_median, Data};

    struct TestData {
        value: f64,
        weight: f64,
    }

    impl Data for TestData {
        fn get_value(&self) -> f64 {
            self.value
        }

        fn get_weight(&self) -> f64 {
            self.weight
        }

        fn set_weight(&mut self, new_weight: f64) {
            self.weight = new_weight;
        }
    }

    #[test]
    fn empty_slice() {
        assert_eq!(
            weighted_median::<TestData>(&mut []),
            None
        )
    }

    #[test]
    fn one_element() {
        assert_eq!(
            weighted_median(&mut [TestData {
                value: 7.0,
                weight: 9.0
            }]),
            Some(7.0)
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
            Some(8.0)
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
            Some(8.0)
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
            Some(7.5)
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
            Some(2.0)
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
            Some(2.0)
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
            Some(2.0)
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
            Some(1.0)
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
            Some(3.0)
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
            Some(2.5)
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
            Some(2.5)
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
            Some(2.5)
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
            Some(3.0)
        );
    }

    #[test]
    fn duplicated_values() {
        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
                TestData {
                    value: 1.0,
                    weight: 1.0
                },
                TestData {
                    value: 2.0,
                    weight: 2.0
                }
            ]),
            Some(1.5)
        );

        assert_eq!(
            weighted_median(&mut [
                TestData {
                    value: 1.0,
                    weight: 2.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                },
                TestData {
                    value: 2.0,
                    weight: 1.0
                }
            ]),
            Some(1.5)
        );
    }
}
