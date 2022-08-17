
#[inline]
pub fn partition<T: crate::Data>(data: &mut [T]) -> usize {
    data.swap(data.len() / 2, 0);

    let mut pivot_index = 0;
    let pivot_value = data[pivot_index].get_value();

    let mut iter = data.into_iter();
    'main: while let Some(front) = iter.next() {
        if front.get_value() > pivot_value {
            loop {
                match iter.next_back() {
                    Some(back) => {
                        if back.get_value() < pivot_value {
                            std::mem::swap(front, back);
                            break;
                        }
                    }
                    None => {
                        break 'main;
                    }
                }
            }
        }
        pivot_index += 1;
    }

    pivot_index -= 1;
    data.swap(0, pivot_index);

    pivot_index
}

#[cfg(test)]
mod tests {
    use super::partition;
    use crate::Data;

    struct TestData {
        value: f64,
    }

    impl Data for TestData {
        fn get_value(&self) -> f64 {
            self.value
        }

        fn get_weight(&self) -> f64 {
            panic!("Not implemented")
        }
    }

    #[test]
    fn partition_unchanged() {
        let mut input = [
            TestData { value: 1.0 },
            TestData { value: 2.0 },
            TestData { value: 3.0 },
        ];

        let pivot_index = partition(&mut input);

        assert_eq!(pivot_index, 1, "pivot_index is 1");
        assert_eq!(input[0].value, 1.0, "first value is 1.0");
        assert_eq!(input[1].value, 2.0, "second value is 2.0");
        assert_eq!(input[2].value, 3.0, "third value is 3.0");
    }

    #[test]
    fn partition_changed() {
        let mut input = [
            TestData { value: 3.0 },
            TestData { value: 2.0 },
            TestData { value: 1.0 },
        ];

        let pivot_index = partition(&mut input);

        assert_eq!(pivot_index, 1);

        assert_eq!(input[0].value, 1.0);
        assert_eq!(input[1].value, 2.0);
        assert_eq!(input[2].value, 3.0);
    }

    #[test]
    fn partition_changed2() {
        let mut input = [
            TestData { value: 3.0 },
            TestData { value: 1.0 },
            TestData { value: 2.0 },
        ];

        let pivot_index = partition(&mut input);

        assert_eq!(pivot_index, 0);

        assert_eq!(input[0].value, 1.0);
        // assert_eq!(input[1].value, 2.0);
        // assert_eq!(input[2].value, 3.0);
    }

    #[test]
    fn partition_changed3() {
        let mut input = [
            TestData { value: 1.0 },
            TestData { value: 3.0 },
            TestData { value: 2.0 },
        ];

        let pivot_index = partition(&mut input);

        assert_eq!(pivot_index, 2);

        // assert_eq!(input[0].value, 1.0);
        // assert_eq!(input[1].value, 2.0);
        assert_eq!(input[2].value, 3.0);
    }
}
