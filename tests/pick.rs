#![allow(dead_code)]
mod utils;
use type_utilities_rs::pick;

test!(pick_partial_field, {
    {
        #[pick(NewS, [a])]
        struct S {
            a: i32,
            b: &'static str,
        }
        _ = S { a: 10, b: "hello" };
        _ = NewS { a: 1 };
    }
});

test!(pick_all_files, {
    {
        #[pick(NewS, [a,b])]
        struct S {
            a: i32,
            b: &'static str,
        }
        _ = NewS { a: 1, b: "foo" };
    }
});

test!(pick_non_existent_field, {
    #[pick(NewS, [c])]
    struct S {
        a: i32,
        b: &'static str,
    }
    _ = NewS {};
});

test!(pick_unit_struct, {
    #[pick(NewUnit)]
    struct Unit;
    _ = NewUnit;
});
