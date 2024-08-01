
#[macro_export]
macro_rules! assert_equal {
    ($left:expr, $right:expr, $err:expr) => {
        if $left != $right {
            return Err($err.into());
        }
    };
}