use std::cmp::PartialEq;

#[derive(Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

// TODO
#[macro_export]
macro_rules! e_for {
    ($val:tt <- $function:tt $arg:tt $($val_o:tt <- $f:tt $a:tt)*) => (
        {
            $function$arg
            $(
                .flat_map(|n| $f(n))
            )*
        }
    );
}

impl<A, B> Either<A, B> {
    pub fn swap(self) -> Either<B, A> {
        match self {
            Either::Left(a) => Either::Right(a),
            Either::Right(b) => Either::Left(b),
        }
    }

    pub fn for_each<F, U>(self, f: F)
    where
        F: Fn(B) -> U,
    {
        match self {
            Either::Right(b) => {
                f(b);
            }
            _ => {}
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            &Either::Left(_) => false,
            &Either::Right(_) => true,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            &Either::Left(_) => true,
            &Either::Right(_) => false,
        }
    }

    pub fn contains<F>(&self, f: F) -> bool
    where 
        F: Fn(&B) -> bool
    {
        match self {
            &Either::Right(ref b) => f(b),
            _ => false,
        }
    }

    pub fn get_or_else(self, f: B) -> B {
        match self {
            Either::Right(b) => b,
            _ => f,
        }
    }

    pub fn map<B1, F>(self, f: F) -> Either<A, B1>
    where
        F: Fn(B) -> B1,
    {
        match self {
            Either::Left(a) => Either::Left(a),
            Either::Right(b) => Either::Right(f(b)),
        }
    }

    // TODO test
    pub fn flat_map<A1, B1, F>(self, f: F) -> Either<A1, B1>
    where
        A1: From<A>,
        F: Fn(B) -> Either<A1, B1>,
    {
        match self {
            Either::Right(r) => f(r),
            Either::Left(l) => Either::Left(From::from(l)),
        }
    }

    pub fn to_option(self) -> Option<B> {
        match self {
            Either::Right(b) => Some(b),
            _ => None,
        }
    }
}

impl<A, B> PartialEq for Either<A, B>
where
    A: PartialEq,
    B: PartialEq,
{
    fn eq(&self, other: &Either<A, B>) -> bool {
        let mut result = false;
        if self.is_right() == other.is_right() {
            match (self, other) {
                (&Either::Left(ref a), &Either::Left(ref a1)) => {
                    result = a == a1;
                }
                (&Either::Right(ref b), &Either::Right(ref b1)) => {
                    result = b == b1;
                }
                _ => {
                    result = false;
                }
            }
        }
        result
    }
}

/*
* Either::from(Result<B, A>) -> Either<A, B>
* (Result<B, A>).into() -> Either<A, B>
*/
impl<A, B> From<Result<B, A>> for Either<A, B> {
    fn from(from: Result<B, A>) -> Self {
        match from {
            Ok(a) => Either::Right(a),
            Err(b) => Either::Left(b),
        }
    }
}
