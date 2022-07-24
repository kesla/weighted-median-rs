#[derive(Debug, PartialEq)]
pub struct Data {
    pub value: f64,
    pub weight: f64,
}

fn weight_sum(input: &mut [Data]) -> f64 {
    return input
        .into_iter()
        .fold(0.0, |accum, item| accum + item.weight);
}

pub fn weighted_median(input: &mut [Data]) -> f64 {
    let n = input.len();

    if n == 1 {
        return input[0].value;
    }

    if n == 2 {
        if input[0].weight == input[1].weight {
            return (input[0].value + input[1].value) / 2.0;
        } else if input[0].weight > input[1].weight {
            return input[0].value;
        } else {
            return input[1].value;
        }
    }

    let pivot_index = input.len() / 2;
    let (lower, pivot, higher) =
        input.select_nth_unstable_by(pivot_index, |a, b| a.value.partial_cmp(&b.value).unwrap());

    let lower_weight_sum = weight_sum(lower);
    let higher_weight_sum = weight_sum(higher);
    let weight_sum = lower_weight_sum + pivot.weight + higher_weight_sum;

    if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
        return pivot.value;
    }

    if lower_weight_sum / weight_sum >= 0.5 {
        input[pivot_index].weight = input[pivot_index].weight + higher_weight_sum;
        weighted_median(&mut input[..pivot_index + 1])
    } else {
        input[pivot_index].weight = input[pivot_index].weight + lower_weight_sum;
        weighted_median(&mut input[pivot_index..])
    }
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
        )
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
        let input = &mut [
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
        ];
        let result = weighted_median(input);

        assert_eq!(result, 2.5);
    }

    #[test]
    fn four_elements_is_even() {
        let input = &mut [
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
        ];
        let result = weighted_median(input);

        assert_eq!(result, 2.5);
    }
}
