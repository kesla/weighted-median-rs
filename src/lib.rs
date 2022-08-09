use is_sorted::IsSorted;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Data {
    pub value: f64,
    pub weight: f64,
}

fn weight_sum(input: &mut [Data], lower_weight_delta: f64, higher_weight_delta: f64) -> f64 {
    input
        .into_iter()
        .fold(lower_weight_delta + higher_weight_delta, |accum, item| {
            accum + item.weight
        })
}

struct WeightedMedian<'slice> {
    data: &'slice mut [Data],
    lower_weight_delta: f64,
    higher_weight_delta: f64,
}

impl<'slice> WeightedMedian<'slice> {
    fn new(data: &'slice mut [Data], lower_weight_delta: f64, higher_weight_delta: f64) -> Self {
        Self {
            data,
            lower_weight_delta,
            higher_weight_delta,
        }
    }

    fn calculate_sorted(self) -> f64 {
        let sum: f64 = weight_sum(self.data, self.lower_weight_delta, self.higher_weight_delta);
        let mut current_weight = self.lower_weight_delta;
        let mut iterator = self.data.iter();

        loop {
            let row = iterator.next().unwrap();
            current_weight = current_weight + row.weight;

            if current_weight / sum == 0.5 {
                break (row.value + iterator.next().unwrap().value) / 2.0;
            } else if current_weight / sum > 0.5 {
                break row.value;
            }
        }
    }

    fn calculate_not_sorted(self) -> f64 {
        let pivot_index = self.data.len() / 2;
        let (lower, pivot, higher) = self
            .data
            .select_nth_unstable_by(pivot_index, |a, b| a.value.partial_cmp(&b.value).unwrap());

        let lower_weight_sum = weight_sum(lower, self.lower_weight_delta, 0.0);
        let higher_weight_sum = weight_sum(higher, 0.0, self.higher_weight_delta);
        let weight_sum = lower_weight_sum + pivot.weight + higher_weight_sum;

        if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
            pivot.value
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
            self.data[0].value
        } else if n == 2 {
            if self.lower_weight_delta + self.data[0].weight
                == self.data[1].weight + self.higher_weight_delta
            {
                (self.data[0].value + self.data[1].value) / 2.0
            } else if self.data[0].weight > self.data[1].weight {
                self.data[0].value
            } else {
                self.data[1].value
            }
        } else if IsSorted::is_sorted_by_key(&mut self.data.into_iter(), |data| data.value) {
            self.calculate_sorted()
        } else {
            self.calculate_not_sorted()
        }
    }
}

pub fn weighted_median(input: &mut [Data]) -> f64 {
    WeightedMedian::new(input, 0.0, 0.0).calculate()
}

#[cfg(test)]
mod tests {
    use crate::{weighted_median, Data};

    #[test]
    fn one_element() {
        assert_eq!(
            weighted_median(&mut [Data {
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
                Data {
                    value: 7.0,
                    weight: 1.0
                },
                Data {
                    value: 8.0,
                    weight: 2.0
                }
            ]),
            8.0
        );
        assert_eq!(
            weighted_median(&mut [
                Data {
                    value: 8.0,
                    weight: 2.0
                },
                Data {
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
                Data {
                    value: 7.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 2.0,
                    weight: 1.0
                },
                Data {
                    value: 3.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 3.0,
                    weight: 1.0
                },
                Data {
                    value: 2.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 3.0,
                    weight: 1.0
                },
                Data {
                    value: 1.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 3.0,
                    weight: 1.0
                },
                Data {
                    value: 2.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 3.0,
                    weight: 5.0
                },
                Data {
                    value: 2.0,
                    weight: 1.0
                },
                Data {
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
                Data {
                    value: 3.0,
                    weight: 2.0,
                },
                Data {
                    value: 2.0,
                    weight: 1.0,
                },
                Data {
                    value: 1.0,
                    weight: 1.0,
                },
            ]),
            2.5
        );
        assert_eq!(
            weighted_median(&mut [
                Data {
                    value: 1.0,
                    weight: 1.0,
                },
                Data {
                    value: 2.0,
                    weight: 1.0,
                },
                Data {
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
                Data {
                    value: 1.0,
                    weight: 0.49,
                },
                Data {
                    value: 2.0,
                    weight: 0.01,
                },
                Data {
                    value: 3.0,
                    weight: 0.25,
                },
                Data {
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
                Data {
                    value: 2.0,
                    weight: 0.5
                },
                Data {
                    value: 1.0,
                    weight: 0.5
                },
                Data {
                    value: 3.0,
                    weight: 1.0
                },
                Data {
                    value: 10.0,
                    weight: 0.8
                },
                Data {
                    value: 8.0,
                    weight: 0.2
                }
            ]),
            3.0
        );
    }

    mod sorted {
        use crate::{Data, WeightedMedian};

        #[test]
        fn with_lower_and_higher_delta() {
            assert_eq!(
                WeightedMedian::new(
                    &mut [
                        Data {
                            value: 1.0,
                            weight: 1.0
                        },
                        Data {
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
                        Data {
                            value: 1.0,
                            weight: 2.0
                        },
                        Data {
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
