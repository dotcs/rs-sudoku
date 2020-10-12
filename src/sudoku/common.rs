use itertools::Itertools;

/// Test if a vector only contains unique digits, but ignore values that are
/// equal to zero.
pub fn has_only_unique_digits(digits: &Vec<u8>) -> bool {
    // Get all non-zero values (unfilled values)
    let nonzero_values: Vec<&u8> = digits.into_iter().filter(|v| **v != 0).collect();

    // If not all non-zero values in the parcel are unique, the parcel is not valid
    let unique_values: Vec<&u8> = nonzero_values.clone().into_iter().unique().collect();

    // The parcel is valid if both, the nonzero and the unique values have the same
    // dimension
    nonzero_values.len() == unique_values.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_check_for_unique_numbers() {
        let non_unique = vec![1, 1, 2];
        let unique = vec![1, 2, 3];
        assert!(!has_only_unique_digits(&non_unique));
        assert!(has_only_unique_digits(&unique));
    }

    #[test]
    fn it_should_ignore_zeros() {
        let unique = vec![0, 0, 1, 2, 3];
        assert!(has_only_unique_digits(&unique));
    }
}
