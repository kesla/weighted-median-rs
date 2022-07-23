use std::{
    cmp::Ordering,
    slice::{Iter, IterMut},
};

#[derive(Debug, PartialEq)]
pub struct Data {
    pub value: f64,
    pub weight: f64,
}

pub fn weighted_median(input: &mut [Data]) -> f64 {
    let n = input.len();

    if n == 1 {
        return input[0].value;
    }

    if n == 2 {
        return (input[0].value + input[1].value) / 2.0;
    }

    let (lower, pivot, higher) = input.select_nth_unstable_by(input.len() / 2, |a, b| {
        if a.value > b.value {
            return Ordering::Greater;
        }
        if b.value > a.value {
            return Ordering::Less;
        }

        return Ordering::Equal;
    });

    let lower_weight_sum = lower.into_iter().fold(0.0, |accum, item| accum + item.weight);
    let higher_weight_sum = higher.into_iter().fold(0.0, |accum, item| accum + item.weight);
    let weight_sum = lower_weight_sum + pivot.weight + higher_weight_sum;

    if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
        return pivot.value
    }

    return -1.0;
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
    fn two_elements() {
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
                    value: 2.0,
                    weight: 1.0
                },
            ]),
            2.0
        )
    }
}
