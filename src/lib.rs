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

fn weighted_median_sorted(
    input: &mut [Data],
    lower_weight_delta: f64,
    higher_weight_delta: f64,
) -> f64 {
    let sum: f64 = weight_sum(input, lower_weight_delta, higher_weight_delta);
    let mut current_weight = lower_weight_delta + input[0].weight;
    let mut i = 0;
    loop {
        if current_weight / sum == 0.5 {
            return (input[i].value + input[i + 1].value) / 2.0;
        }

        if current_weight / sum > 0.5 {
            return input[i].value;
        }

        i = i + 1;
        current_weight = current_weight + input[i].weight;
    }
}

fn is_sorted(input: &mut [Data]) -> bool {
    let mut prev_value = input[0].value;

    input[1..].into_iter().all(|data| {
        if data.value > prev_value {
            return false;
        }

        prev_value = data.value;
        return true;
    })
}

fn _weighted_median(input: &mut [Data], lower_weight_delta: f64, higher_weight_delta: f64) -> f64 {
    let n = input.len();

    if n == 1 {
        return input[0].value;
    }

    if n == 2 {
        if lower_weight_delta + input[0].weight == input[1].weight + higher_weight_delta {
            return (input[0].value + input[1].value) / 2.0;
        } else if input[0].weight > input[1].weight {
            return input[0].value;
        } else {
            return input[1].value;
        }
    }

    if is_sorted(input) {
        return weighted_median_sorted(input, lower_weight_delta, higher_weight_delta);
    }

    let pivot_index = input.len() / 2;
    let (lower, pivot, higher) =
        input.select_nth_unstable_by(pivot_index, |a, b| a.value.partial_cmp(&b.value).unwrap());

    let lower_weight_sum = weight_sum(lower, lower_weight_delta, 0.0);
    let higher_weight_sum = weight_sum(higher, 0.0, higher_weight_delta);
    let weight_sum = lower_weight_sum + pivot.weight + higher_weight_sum;

    if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
        return pivot.value;
    }

    if lower_weight_sum / weight_sum >= 0.5 {
        return _weighted_median(
            &mut input[..pivot_index + 1],
            lower_weight_delta,
            higher_weight_delta + higher_weight_sum,
        );
    } else {
        return _weighted_median(
            &mut input[pivot_index..],
            lower_weight_delta + lower_weight_sum,
            higher_weight_delta,
        );
    }
}

pub fn weighted_median(input: &mut [Data]) -> f64 {
    _weighted_median(input, 0.0, 0.0)
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
        use crate::{weighted_median_sorted, Data};

        #[test]
        fn with_lower_and_higher_delta() {
            assert_eq!(
                weighted_median_sorted(
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
                ),
                1.5
            );

            assert_eq!(
                weighted_median_sorted(
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
                ),
                1.5
            )
        }
    }
}
