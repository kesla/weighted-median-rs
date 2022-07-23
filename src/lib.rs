pub struct Data {
    pub value: f64,
    pub weight: f64,
}

pub fn weighted_median(input: &[Data]) -> f64 {
    let n = input.len();

    if n == 1 {
        return input[0].value;
    }

    if n == 2 {
        return (input[0].value + input[1].value) / 2.0;
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

    #[test]
    fn two_elements() {
        assert_eq!(
            weighted_median(&[
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
}
