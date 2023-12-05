// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

use std::fmt::Error;
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        // TODO version using `match guards`
        // match value {
        //     value if value == 0 => Err(CreationError::Zero),
        //     value if value < 0 => Err(CreationError::Negative),
        //     value => Ok(PositiveNonzeroInteger(value as u64))
        // }

        // TODO version using `std::cmp`
        match value.cmp(&0) {
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Greater => Ok(PositiveNonzeroInteger(value as u64)),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
