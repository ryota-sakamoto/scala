extern crate scala;

use scala::util::{Either, EitherMonad};

#[test]
fn is_right_test() {
    let right = create_right();
    assert!(right.is_right());
    assert!(!right.is_left());
}

#[test]
fn is_left_test() {
    let left = create_left();
    assert!(!left.is_right());
    assert!(left.is_left());
}

#[test]
fn map_test() {
    let right = create_right().map(Box::new(|n| n + 1));
    assert_eq!(right.get_or_else(0), 1);

    let right = create_right().map(Box::new(|n| n + 1)).map(Box::new(|n| n + 1));
    assert_eq!(right.get_or_else(0), 2);
}

#[test]
fn get_or_else_test() {
    let right = create_right();
    assert_eq!(right.get_or_else(1), 0);

    let left = create_left();
    assert_eq!(left.get_or_else(1), 1);
}

fn create_right() -> Either<(), u64> {
    Either::Right(0u64)
}

fn create_left() -> Either<String, u64> {
    Either::Left("Left".to_string())
}