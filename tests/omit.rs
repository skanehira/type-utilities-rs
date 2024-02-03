#![allow(dead_code)]
mod utils;
use types_rs::omit;

test!(keep_visible, {
    mod x {
        use super::*;
        #[omit(NewS, [b])]
        pub struct S {
            pub a: i32,
            pub b: &str,
        }
    }
    _ = x::NewS { a: 1 };
});

test!(omit_partial_field, {
    {
        #[omit(NewS, [b])]
        struct S {
            a: i32,
            b: &str,
        }
        _ = NewS { a: 1 };
    }
});

test!(omit_all_files, {
    #[omit(NewS, [a, b])]
    struct S {
        a: i32,
        b: &str,
    }
    _ = NewS {};
});

test!(skip_non_existent_field, {
    #[omit(NewS, [c])]
    struct S<'a> {
        a: i32,
        b: &'a str,
    }
    _ = NewS { a: 1, b: "2" };
});

test!(omit_tuple_struct, {
    #[omit(NewPair)]
    struct Pair(i32, i64);
    _ = NewPair(1, 10_i64);
});

test!(omit_empty_tuple, {
    #[omit(EmtpyPair)]
    struct Pair();
    _ = EmtpyPair();
});

test!(omit_unit_struct, {
    #[omit(NewUnit)]
    struct Unit;
    _ = NewUnit;
});

test!(omit_generic_struct, {
    #[omit(NewS)]
    struct S<'a, T> {
        a: &'a T,
    }
    let x = (1, 2, 3);
    _ = NewS { a: &x };
});
