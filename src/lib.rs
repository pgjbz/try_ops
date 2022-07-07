//! Macro thats simulate try catch but use 'ops'
//! To use this macro you should give a try block that
//! return a Result with Box<dyn std::error:Error>;
//! The ops block should be return the same type of try block if success
//! ret_type is necessary because compile cannot define Ok type
#[macro_export]
macro_rules! catch {
    ($ret_type:ty => try $block:block $(ops $name:ident: $ty: ty $ops_block:block)+) => {
        {
            #[inline(always)]
            fn handled() -> Result<$ret_type, Box<dyn std::error::Error>> $block
            match handled() {
                Ok(val) => val,
                $(Err($name) if $name.is::<$ty>() => {
                    let $name = $name.downcast_ref::<$ty>().unwrap();
                    $ops_block
                },)*
                e => panic!("{:?}", e)
            }
        }
    };
    (try $block:block $(ops $name:ident: $ty: ty $ops_block:block)+) => {crate::catch!(() => try $block $(ops $name: $ty $ops_block )+ ) }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::fmt;
    use std::{error::Error, fmt::Display, io};

    #[derive(Debug)]
    struct ErrorTests;

    impl Display for ErrorTests {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "work")
        }
    }

    impl Error for ErrorTests {}

    #[test]
    fn ops_correct_type() {
        catch!(() => try {
            aux(true)?;
            assert!(false);
            Ok(())
        } ops e: io::Error {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        } ops _e: ErrorTests {
        });
    }

    #[test]
    fn ops_correct_type_without_inform_return() {
        catch!(try {
            aux(true)?;
            assert!(false);
            Ok(())
        } ops e: io::Error {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        } ops _e: ErrorTests {
        });
    }



    #[test]
    fn success() {
        let result = catch!(() => try {
            aux(false)?;
            Ok(())
        } ops e: io::Error {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        } ops _e: ErrorTests {
            assert!(false);
        });
        assert_eq!((), result)
    }

    #[test]
    fn success_without_inform_return() {
        let result = catch!(try {
            aux(false)?;
            Ok(())
        } ops e: io::Error {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        } ops _e: ErrorTests {
            assert!(false);
        });
        assert_eq!((), result)
    }

    fn aux(is_error: bool) -> Result<(), Box<dyn Error>> {
        if is_error {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "oh nooo")));
        }
        Ok(())
    }
}
