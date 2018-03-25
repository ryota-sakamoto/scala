#[derive(Debug)]
pub enum Either<E, R> {
    Right(R),
    Left(E),
}

trait EitherMonad<T, E, R> {
    fn map(self, f: Box<Fn(R) -> T>) -> Either<E, T>;
    fn flat_map(self, f: Box<Fn(R) -> Either<E, T>>) -> Either<E, T>;
}

impl<T, E, R> EitherMonad<T, E, R> for Either<E, R> {
    fn map(self, f: Box<Fn(R) -> T>) -> Either<E, T> {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(e) => Either::Left(e),
        }
    }

    // TODO
    fn flat_map(self, f: Box<Fn(R) -> Either<E, T>>) -> Either<E, T> {
        match self {
            Either::Right(r) => f(r),
            Either::Left(e) => Either::Left(e),
        }
    }
}

impl<E, R> Either<E, R> {
    fn is_right(&self) -> bool {
        match *self {
            Either::Right(_) => true,
            Either::Left(_) => false,
        }
    }

    fn is_left(&self) -> bool {
        match *self {
            Either::Right(_) => false,
            Either::Left(_) => true,
        }
    }

    fn contains(self, f: Box<Fn(R) -> bool>) -> bool {
        match self {
            Either::Right(r) => f(r),
            Either::Left(_) => false,
        }
    }

    fn to_option(self) -> Option<R> {
        match self {
            Either::Right(r) => Some(r),
            Either::Left(_) => None,
        }
    }
}