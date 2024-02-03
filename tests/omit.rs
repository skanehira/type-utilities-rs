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
        #[omit(NewS, [a, b])]
        struct S {
            a: i32,
            b: &str,
        }
        _ = NewS {};
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
    struct Pair(i32);
    _ = NewPair(1);
});

test!(omit_tuple_struct_that_has_no_fields, {
    #[omit(NewFoo4)]
    struct Pair();
    _ = NewFoo4();
});

test!(omit_unit_struct, {
    #[omit(NewS)]
    struct Unit;
    _ = NewS;
});

test!(omit_generic_struct, {
    #[omit(NewS)]
    struct S<'a, T> {
        a: &'a T,
    }
    let x = (1, 2, 3);
    _ = NewS { a: &x };
});
