use crate::Data;

#[inline]
pub fn partition<T: Data>(data: &mut [T], partition_index: usize) -> (usize, &mut [T]) {
    data.swap(partition_index, 0);
    let pivot_value = data[0].get_value();

    let mut len = data.len();
    let mut i = 1;

    while i < len {
        if data[i].get_value() == pivot_value {
            len -= 1;
            data[0].set_weight(data[0].get_weight() + data[i].get_weight());
            data.swap(i, len);
        } else {
            i += 1;
        }
    }

    let new_data = match len == data.len() {
        true => data,
        false => &mut data[0..len],
    };

    let pivot_index = partition_without_duplicates(new_data, pivot_value) - 1;

    new_data.swap(0, pivot_index);

    (pivot_index, new_data)
}

#[inline]
fn partition_without_duplicates<T: Data>(data: &mut [T], pivot_value: f64) -> usize {
    let mut pivot_index = 0;
    let mut end_index = data.len();

    'main: while pivot_index < end_index {
        if data[pivot_index].get_value() > pivot_value {
            loop {
                end_index -= 1;
                if data[end_index].get_value() < pivot_value {
                    data.swap(pivot_index, end_index);
                    break;
                }

                if pivot_index == end_index {
                    break 'main;
                }
            }
        }

        pivot_index += 1;
    }

    pivot_index
}

#[cfg(test)]
mod tests {
    use super::partition;
    use crate::Data;

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
    fn partition_unchanged() {
        let mut input = [
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
                weight: 1.0,
            },
        ];

        let (pivot_index, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 1, "pivot_index is 1");
        assert_eq!(input[0].get_value(), 1.0, "first value is 1.0");
        assert_eq!(input[1].get_value(), 2.0, "second value is 2.0");
        assert_eq!(input[2].get_value(), 3.0, "third value is 3.0");
    }

    #[test]
    fn partition_changed1() {
        let mut input = [
            TestData {
                value: 3.0,
                weight: 1.0,
            },
            TestData {
                value: 2.0,
                weight: 1.0,
            },
            TestData {
                value: 1.0,
                weight: 1.0,
            },
        ];

        let (pivot_index, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 1);

        assert_eq!(input[0].get_value(), 1.0);
        assert_eq!(input[1].get_value(), 2.0);
        assert_eq!(input[2].get_value(), 3.0);
    }

    #[test]
    fn partition_changed2() {
        let mut input = [
            TestData {
                value: 3.0,
                weight: 1.0,
            },
            TestData {
                value: 1.0,
                weight: 1.0,
            },
            TestData {
                value: 2.0,
                weight: 1.0,
            },
        ];

        let (pivot_index, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 0);

        assert_eq!(input[0].get_value(), 1.0);
    }

    #[test]
    fn partition_changed3() {
        let mut input = [
            TestData {
                value: 1.0,
                weight: 1.0,
            },
            TestData {
                value: 3.0,
                weight: 1.0,
            },
            TestData {
                value: 2.0,
                weight: 1.0,
            },
        ];

        let (pivot_index, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 2);
        assert_eq!(input[2].get_value(), 3.0);
    }

    #[test]
    fn duplicated_values1() {
        let mut input = [
            TestData {
                value: 1.0,
                weight: 1.0,
            },
            TestData {
                value: 1.0,
                weight: 0.5,
            },
        ];
        let (_, result) = partition(&mut input, 1);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_value(), 1.0);
        assert_eq!(result[0].get_weight(), 1.5);
    }

    #[test]
    fn duplicated_values2() {
        let mut input = [
            TestData {
                value: 1.0,
                weight: 1.0,
            },
            TestData {
                value: 1.0,
                weight: 0.5,
            },
            TestData {
                value: 2.0,
                weight: 1.0,
            },
        ];
        let (_, result) = partition(&mut input, 1);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_value(), 1.0);
        assert_eq!(result[0].get_weight(), 1.5);
        assert_eq!(result[1].get_value(), 2.0);
        assert_eq!(result[1].get_weight(), 1.0);
    }
}
