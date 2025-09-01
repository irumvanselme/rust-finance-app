#[cfg(test)]
mod test_amount {
    use crate::app::typing::amount::{Amount, AmountError, _MAX_AMOUNT, _MIN_AMOUNT};

    #[test]
    fn test_negative_amount() {
        // GIVEN a negative amount
        let given_amount = _MIN_AMOUNT - 10f32;

        // WHEN creating an amount
        let amount = Amount::new(given_amount);

        // THEN it should result into a Min Amount error.
        assert!(amount.is_err());
        assert_eq!(amount.err().unwrap(), AmountError::MinValue(given_amount))
    }

    #[test]
    fn test_more_than_maximum_amount() {
        // GIVE an amount, more than the maximum amount.
        let given_amount = _MAX_AMOUNT + 1f32;

        // WHEN creating an amount
        let amount = Amount::new(given_amount);

        // THEN it should result into a Max Amount error.
        assert!(amount.is_err());
        assert_eq!(amount.err().unwrap(), AmountError::MaxValue(given_amount))
    }

    #[test]
    fn test_error_messages() {
        let given_amount = _MIN_AMOUNT - 10f32;
        let value = match Amount::new(given_amount) {
            Ok(value) => value,
            Err(error) => {
                assert_eq!(
                    error.to_string(),
                    format!(
                        "invalid amount value: {}, must be greater than the minimum value {}",
                        given_amount, _MIN_AMOUNT
                    )
                );
                Amount::new(10f32).unwrap()
            }
        };

        print!("{:?}", value);
    }
}
