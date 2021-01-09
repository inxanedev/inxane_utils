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
