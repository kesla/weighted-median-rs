#[inline]
pub fn is_sorted<T: crate::Data>(data: &mut [T]) -> bool {
    let mut iter = data.into_iter().peekable();
    let mut forward_sorted = true;
    let mut backward_sorted = true;

    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            forward_sorted = forward_sorted && current.get_value() < next.get_value();
            backward_sorted = backward_sorted && current.get_value() > next.get_value();

            if !backward_sorted && !forward_sorted {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod test {

    use super::is_sorted;
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
    fn data_is_forward_sorted() {
        let mut input = [
            TestData { value: 1.0 },
            TestData { value: 2.0 },
            TestData { value: 3.0 },
        ];

        assert_eq!(is_sorted(&mut input), true);
    }

    #[test]
    fn data_is_backward_sorted() {
        let mut input = [
            TestData { value: 3.0 },
            TestData { value: 2.0 },
            TestData { value: 1.0 },
        ];

        assert_eq!(is_sorted(&mut input), true);
    }

    #[test]
    fn data_is_not_sorted() {
        let mut input = [
            TestData { value: 1.0 },
            TestData { value: 3.0 },
            TestData { value: 2.0 },
        ];

        assert_eq!(is_sorted(&mut input), false);
    }
}
