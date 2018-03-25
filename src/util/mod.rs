#[derive(Debug)]
pub enum Either<E, R> {
    Right(R),
    Left(E),
}

pub trait EitherMonad<T, E, R> {
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

    // TODO test
    fn flat_map(self, f: Box<Fn(R) -> Either<E, T>>) -> Either<E, T> {
        match self {
            Either::Right(r) => f(r),
            Either::Left(e) => Either::Left(e),
        }
    }
}

impl<E, R> Either<E, R> {
    pub fn is_right(&self) -> bool {
        match *self {
            Either::Right(_) => true,
            Either::Left(_) => false,
        }
    }

    pub fn is_left(&self) -> bool {
        match *self {
            Either::Right(_) => false,
            Either::Left(_) => true,
        }
    }

    // TODO test
    pub fn contains(self, f: Box<Fn(R) -> bool>) -> bool {
        match self {
            Either::Right(r) => f(r),
            Either::Left(_) => false,
        }
    }

    pub fn get_or_else(self, f: R) -> R {
        match self {
            Either::Right(r) => r,
            Either::Left(_) => f,
        }
    }

    // TODO test
    pub fn to_option(self) -> Option<R> {
        match self {
            Either::Right(r) => Some(r),
            Either::Left(_) => None,
        }
    }
}