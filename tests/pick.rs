#![allow(dead_code)]
mod utils;
use types_rs::pick;

test!(omit_partial_field, {
    {
        #[pick(NewS, [a])]
        struct S {
            a: i32,
            b: &str,
        }
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
