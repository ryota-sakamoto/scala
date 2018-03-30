extern crate scala;

use scala::util::Either;

#[test]
fn swap_test() {
    let right = create_right().swap();
    assert_eq!(right, Either::Left(0));

    let left = create_left().swap();
    assert_eq!(left, Either::Right("Left".to_string()));
}

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
    let right = create_right().map(|n| n + 1);
    assert_eq!(right.get_or_else(0), 1);

    let right = create_right().map(|n| n + 1).map(|n| n + 1);
    assert_eq!(right.get_or_else(0), 2);
}

#[test]
fn get_or_else_test() {
    let right = create_right();
    assert_eq!(right.get_or_else(1), 0);

    let left = create_left();
    assert_eq!(left.get_or_else(1), 1);
}

#[test]
fn to_option_test() {
    let right = create_right().to_option();
    assert_eq!(right, Some(0u64));

    let left = create_left().to_option();
    assert_eq!(left, None);
}

#[test]
fn eq_test() {
    let right = create_right();
    assert!(right == Either::Right(0));
    assert!(right != Either::Right(1));  

    let left = create_left();
    assert!(left == Either::Left("Left".to_string()));  
    assert!(left != Either::Left("Right".to_string()));
}

fn create_right() -> Either<(), u64> {
    Either::Right(0u64)
}

fn create_left() -> Either<String, u64> {
    Either::Left("Left".to_string())
}