mod partition;
use partition::partition;

pub struct Data {
    pub value: f64,
    pub weight: f64,
}

#[inline]
fn weight_sum(input: &mut [Data]) -> f64 {
    input
        .into_iter()
        .fold(0.0, |accum, item| accum + item.weight)
}

pub fn calculate(data: &mut [Data], lower_weight_delta: f64, higher_weight_delta: f64) -> f64 {
    match data.len() {
        1 => data[0].value,
        2 => {
            let lower = lower_weight_delta + data[0].weight;
            let higher = data[1].weight + higher_weight_delta;
            if lower == higher {
                (data[0].value + data[1].value) / 2.0
            } else if lower > higher {
                data[0].value
            } else {
                data[1].value
            }
        }
        _ => {
            let (pivot_index, new_data) = partition(data, data.len() / 2);

            let lower_weight_sum = lower_weight_delta + weight_sum(&mut new_data[..pivot_index]);
            let higher_weight_sum =
                higher_weight_delta + weight_sum(&mut new_data[pivot_index + 1..]);
            let weight_sum = lower_weight_sum + new_data[pivot_index].weight + higher_weight_sum;

            if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
                new_data[pivot_index].value
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
pub fn weighted_median(data: &mut [Data]) -> f64 {
    calculate(data, 0.0, 0.0)
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

    #[test]
    fn duplicated_values() {
        assert_eq!(
            weighted_median(&mut [
                Data {
                    value: 1.0,
                    weight: 1.0
                },
                Data {
                    value: 1.0,
                    weight: 1.0
                },
                Data {
                    value: 2.0,
                    weight: 2.0
                }
            ]),
            1.5
        );

        assert_eq!(
            weighted_median(&mut [
                Data {
                    value: 1.0,
                    weight: 2.0
                },
                Data {
                    value: 2.0,
                    weight: 1.0
                },
                Data {
                    value: 2.0,
                    weight: 1.0
                }
            ]),
            1.5
        );
    }
}
