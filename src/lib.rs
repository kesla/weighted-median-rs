use is_sorted::IsSorted;

// value, weight
type Data = (f64, f64);

fn weight_sum(input: &mut [Data], lower_weight_delta: f64, higher_weight_delta: f64) -> f64 {
    input
        .into_iter()
        .fold(lower_weight_delta + higher_weight_delta, |accum, item| {
            accum + item.1
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
            current_weight = current_weight + row.1;

            if current_weight / sum == 0.5 {
                break (row.0 + iterator.next().unwrap().0) / 2.0;
            } else if current_weight / sum > 0.5 {
                break row.0;
            }
        }
    }

    fn calculate_not_sorted(self) -> f64 {
        let pivot_index = self.data.len() / 2;
        let (lower, pivot, higher) = self
            .data
            .select_nth_unstable_by(pivot_index, |a, b| a.0.partial_cmp(&b.0).unwrap());

        let lower_weight_sum = weight_sum(lower, self.lower_weight_delta, 0.0);
        let higher_weight_sum = weight_sum(higher, 0.0, self.higher_weight_delta);
        let weight_sum = lower_weight_sum + pivot.1 + higher_weight_sum;

        if lower_weight_sum / weight_sum < 0.5 && higher_weight_sum / weight_sum < 0.5 {
            pivot.0
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
            self.data[0].0
        } else if n == 2 {
            if self.lower_weight_delta + self.data[0].1 == self.data[1].1 + self.higher_weight_delta
            {
                (self.data[0].0 + self.data[1].0) / 2.0
            } else if self.data[0].1 > self.data[1].1 {
                self.data[0].0
            } else {
                self.data[1].0
            }
        } else if IsSorted::is_sorted_by_key(&mut self.data.into_iter(), |data| data.0) {
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
    use crate::weighted_median;

    #[test]
    fn one_element() {
        assert_eq!(weighted_median(&mut [(7.0, 9.0)]), 7.0);
    }

    #[test]
    fn two_elements_different_weight() {
        assert_eq!(weighted_median(&mut [(7.0, 1.0), (8.0, 2.0)]), 8.0);
        assert_eq!(weighted_median(&mut [(8.0, 2.0), (7.0, 1.0),]), 8.0);
    }

    #[test]
    fn two_elements_same_weight() {
        assert_eq!(weighted_median(&mut [(7.0, 1.0), (8.0, 1.0)]), 7.5)
    }

    #[test]
    fn three_elements_is_first_element() {
        assert_eq!(
            weighted_median(&mut [(2.0, 1.0), (3.0, 1.0), (1.0, 1.0),]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_middle_element() {
        assert_eq!(
            weighted_median(&mut [(3.0, 1.0), (2.0, 1.0), (1.0, 1.0),]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_last_element() {
        assert_eq!(
            weighted_median(&mut [(3.0, 1.0), (1.0, 1.0), (2.0, 1.0),]),
            2.0
        )
    }

    #[test]
    fn three_elements_is_smallest_element() {
        assert_eq!(
            weighted_median(&mut [(3.0, 1.0), (2.0, 1.0), (1.0, 5.0),]),
            1.0
        )
    }

    #[test]
    fn three_elements_is_biggest_element() {
        assert_eq!(
            weighted_median(&mut [(3.0, 5.0), (2.0, 1.0), (1.0, 1.0),]),
            3.0
        )
    }

    #[test]
    fn three_elements_is_even() {
        assert_eq!(
            weighted_median(&mut [(3.0, 2.0,), (2.0, 1.0,), (1.0, 1.0,),]),
            2.5
        );
        assert_eq!(
            weighted_median(&mut [(1.0, 1.0,), (2.0, 1.0,), (3.0, 2.0,),]),
            2.5
        );
    }

    #[test]
    fn four_elements_is_even() {
        assert_eq!(
            weighted_median(&mut [(1.0, 0.49,), (2.0, 0.01,), (3.0, 0.25,), (1000.0, 0.25),]),
            2.5
        );
    }

    #[test]
    fn five_elements_is_pivot_value() {
        assert_eq!(
            weighted_median(&mut [(2.0, 0.5), (1.0, 0.5), (3.0, 1.0), (10.0, 0.8), (8.0, 0.2)]),
            3.0
        );
    }

    mod sorted {
        use crate::{WeightedMedian};

        #[test]
        fn with_lower_and_higher_delta() {
            assert_eq!(
                WeightedMedian::new(&mut [(1.0, 1.0), (2.0, 2.0)], 1.0, 0.0).calculate_sorted(),
                1.5
            );

            assert_eq!(
                WeightedMedian::new(&mut [(1.0, 2.0), (2.0, 1.0)], 0.0, 1.0).calculate_sorted(),
                1.5
            )
        }
    }
}
