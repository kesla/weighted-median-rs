use crate::Data;

#[inline]
pub fn partition<T: Data>(data: &mut [T], partition_index: usize) -> (usize, &mut [T], f64) {
    data.swap(partition_index, 0);
    let pivot_value = data[0].get_value();

    let mut len = data.len();
    let mut i = 1;
    let mut pivot_extra_weight = 0.0;

    while i < len {
        if data[i].get_value() == pivot_value {
            len -= 1;
            pivot_extra_weight += data[i].get_weight();
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

    (pivot_index, new_data, pivot_extra_weight)
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
    use crate::tests::TestData;

    use super::partition;

    #[test]
    fn partition_unchanged() {
        let mut input = [TestData::value(1).weight(1), TestData::value(2).weight(1), TestData::value(3).weight(1),];
        let expected = [TestData::value(1).weight(1), TestData::value(2).weight(1), TestData::value(3).weight(1),];

        let (pivot_index, actual, pivot_extra_weight) = partition(&mut input, 1);

        assert_eq!(pivot_index, 1);
        assert_eq!(actual, expected);
        assert_eq!(pivot_extra_weight, 0.0);

        assert_eq!(input, expected);
    }

    #[test]
    fn partition_changed1() {
        let mut input = [TestData::value(3).weight(1), TestData::value(2).weight(1), TestData::value(1).weight(1),];
        let expected = [TestData::value(1).weight(1), TestData::value(2).weight(1), TestData::value(3).weight(1),];

        let (pivot_index, actual, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 1);
        assert_eq!(actual, expected);

        assert_eq!(input, expected);
    }

    #[test]
    fn partition_changed2() {
        let mut input = [TestData::value(3).weight(1), TestData::value(1).weight(1), TestData::value(2).weight(1),];

        let (pivot_index, actual, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 0);

        assert_eq!(actual[0], TestData::value(1).weight(1),);
        assert_eq!(actual.len(), 3);

        assert_eq!(input[0], TestData::value(1).weight(1),);
    }

    #[test]
    fn partition_changed3() {
        let mut input = [TestData::value(1).weight(1), TestData::value(3).weight(1), TestData::value(2).weight(1),];

        let (pivot_index, actual, _) = partition(&mut input, 1);

        assert_eq!(pivot_index, 2);
        assert_eq!(actual[2], TestData::value(3).weight(1),);
        assert_eq!(input[2], TestData::value(3).weight(1),);
    }

    #[test]
    fn duplicated_values1() {
        let mut input = [TestData::value(1).weight(1), TestData::value(1).weight(5)];
        let expected = [TestData::value(1).weight(5)];
        let (_, actual, pivot_extra_weight) = partition(&mut input, 1);

        assert_eq!(actual, expected);
        assert_eq!(pivot_extra_weight, 1.0);
        assert_eq!(input[0], expected[0]);
    }

    #[test]
    fn duplicated_values2() {
        let mut input = [
            TestData::value(1).weight(1),
            TestData::value(1).weight(5),
            TestData::value(2).weight(1),
        ];
        let expected = [TestData::value(1).weight(5), TestData::value(2).weight(1),];
        let (_, actual, pivot_extra_weight) = partition(&mut input, 1);

        assert_eq!(actual, expected);
        assert_eq!(pivot_extra_weight, 1.0);

        assert_eq!(input[..2], expected);
    }
}
