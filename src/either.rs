/// Either is a custom enum type, which holds *either* type A, or B.
/// This can be useful for functions which want to return two different types.
///
/// Basic usage:
/// ```
/// use inxane_utils::Either;
///
/// // An example function that returns an i32 when n is smaller than 50,
/// // but returns a String if it wasn't.
/// fn test(n: i32) -> Either::<i32, String> {
///     if n < 50 {
///         Either::<i32, String>::Left(5)
///     } else {
///         Either::<i32, String>::Right(String::from("Hello!"))
///     }
/// }
///
/// // Example of checking what was returned in other code:
///
/// let value = test(45);
/// // Print "It was an i32!" if the return value was an i32.
/// // Print "It was a String!" if the return value was a String.
/// println!("{}", match value {
///     Either::Left(v) => "It was an i32!",
///     Either::Right(v) => "It was a String!"
/// });
/// ```
pub enum Either<A, B> {
    Left(A),
    Right(B)
}

#[cfg(test)]
mod either_tests {
    use super::*;
    #[test]
    fn basic_usage() {
        let test = Either::<i32, String>::Left(6);
        assert!(match test {
            Either::Left(_) => true,
            Either::Right(_) => false,
        });
        
        let test2 = Either::<i32, String>::Right(String::from("Hello World!"));
        assert!(match test2 {
            Either::Left(_) => false,
            Either::Right(_) => true,
        });
    }
}
