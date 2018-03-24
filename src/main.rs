use std::num::ParseIntError;

#[derive(Debug)]
enum Either<E, R> {
    Right(R),
    Left(E),
}

trait EitherMonad<T, E, R> {
    fn map(self, f: Box<Fn(R) -> T>) -> Either<E, T>;
}

impl<T, E, R> EitherMonad<T, E, R> for Either<E, R> {
    fn map(self, f: Box<Fn(R) -> T>) -> Either<E, T> {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(e) => Either::Left(e),
        }
    }
}

fn main() {
    let _101 = f("100").map(Box::new(|n| n + 1));
    println!("{:?}", _101);
}

fn f(s: &str) -> Either<ParseIntError, u64> {
    match s.parse() {
        Ok(n) => {
            Either::Right(n)
        },
        Err(e) => {
            Either::Left(e)
        },
    }
}
