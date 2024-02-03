#[macro_export]
macro_rules! test {
    ($test_name: ident, $test_fn: expr) => {
        #[test]
        fn $test_name() {
            $test_fn;
        }
    };
}
