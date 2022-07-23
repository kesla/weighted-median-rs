pub struct Data {
    pub value: f64,
    pub weight: f64,
}

pub fn weighted_median(input: &[Data]) -> f64 {
    if input.len() == 1 {
        return input[0].value;
    }

    return -1.0;
}

#[cfg(test)]
mod tests {
    use crate::{weighted_median, Data};

    #[test]
    fn one_element() {
        assert_eq!(
            weighted_median(&[Data {
                value: 7.0,
                weight: 9.0
            }]),
            7.0
        );
    }
}
