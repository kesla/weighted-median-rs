use num_traits::{Float, Num};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Data<V, W> {
    pub value: V,
    pub weight: W,
}

fn weight_sum<T, W, WF>(input: &mut [T], weightFn: WF) -> W
where
    W: Num + PartialOrd + Debug + Copy,
    WF: Fn(&mut T) -> W,
{
    input
        .into_iter()
        .fold(W::zero(), |accum, item| accum + weightFn(item))
}

fn weighted_median_sorted<T, V, W, WF, VF>(input: &mut [T], weightFn: WF, valueFn: VF) -> V
where
    V: Float,
    W: Num + PartialOrd + Debug + Copy,
    WF: Fn(&mut T) -> W + Copy,
    VF: Fn(&mut T) -> V,
{
    let sum: W = weight_sum(input, weightFn);
    let mut current_weight = weightFn(&mut input[0]);
    let mut i = 0;
    loop {
        if current_weight + current_weight == sum {
            return (valueFn(&mut input[i]) + valueFn(&mut input[i + 1])) / V::from(2).unwrap();
        }

        if current_weight + current_weight > sum {
            return valueFn(&mut input[i]);
        }

        i = i + 1;
        current_weight = current_weight + weightFn(&mut input[i]);
    }
}

fn is_sorted<V, W>(input: &mut [Data<V, W>]) -> bool
where
    V: Float,
{
    let mut prev_value = input[0].value;

    input.into_iter().all(|data| {
        if data.value > prev_value {
            return false;
        }

        prev_value = data.value;
        return true;
    })
}

pub fn weighted_median<V, W>(input: &mut [Data<V, W>]) -> V
where
    V: Float,
    W: Num + PartialOrd + Debug + Copy,
{
    let n = input.len();

    if n == 1 {
        return input[0].value;
    }

    if n == 2 {
        if input[0].weight == input[1].weight {
            return (input[0].value + input[1].value) / V::from(2).unwrap();
        } else if input[0].weight > input[1].weight {
            return input[0].value;
        } else {
            return input[1].value;
        }
    }

    if is_sorted(input) {
        return weighted_median_sorted(input, |item| item.weight, |item| item.value);
    }

    let pivot_index = input.len() / 2;
    let (lower, pivot, higher) =
        input.select_nth_unstable_by(pivot_index, |a, b| a.value.partial_cmp(&b.value).unwrap());

    let lower_weight_sum = weight_sum(lower, |item| item.weight);
    let higher_weight_sum = weight_sum(higher, |item| item.weight);
    let weight_sum = lower_weight_sum + pivot.weight + higher_weight_sum;

    // write it like this to avoid comparison w 0.5 (like lower_weight_um / weight_sum < 0.5)
    if lower_weight_sum + lower_weight_sum < weight_sum
        && higher_weight_sum + higher_weight_sum < weight_sum
    {
        return pivot.value;
    }

    // write it like this to avoid comparison w 0.5 (lower_weight_sum / weight_sum >= 0.5)
    if lower_weight_sum + lower_weight_sum >= weight_sum {
        input[pivot_index].weight = input[pivot_index].weight + higher_weight_sum;
        return weighted_median(&mut input[..pivot_index + 1]);
    } else {
        input[pivot_index].weight = input[pivot_index].weight + lower_weight_sum;
        return weighted_median(&mut input[pivot_index..]);
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
    fn weight_is_integer_and_value_is_f32() {
        assert_eq!(
            weighted_median(&mut [Data {
                value: 1.0__f32,
                weight: 1
            }]),
            1.0
        );
    }
}
