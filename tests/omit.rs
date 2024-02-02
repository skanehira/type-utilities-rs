#![allow(dead_code)]
use types_rs::omit;

#[test]
fn in_progress() {}

#[test]
fn should_pass_omit() {
    // omit partial field
    {
        mod x {
            use super::*;
            #[omit(NewS, [b])]
            pub struct S {
                pub a: i32,
                pub b: &str,
            }
        }
        _ = x::NewS { a: 1 };
    }

    // omit all fields
    {
        #[omit(NewS, [a, b])]
        struct S {
            a: i32,
            b: &str,
        }
        _ = NewS {};
    }

    // skip non-existent field
    {
        #[omit(NewS, [c])]
        struct S {
            a: i32,
            b: &'static str,
        }
        _ = NewS { a: 1, b: "2" };
    }

    // omit tuple struct
    {
        #[omit(NewPair)]
        struct Pair(i32);
        _ = NewPair(1);
    }

    // omit tuple struct that has no fields
    {
        #[omit(NewFoo4)]
        struct Pair();
        _ = NewFoo4();
    }

    // omit unit struct
    {
        #[omit(NewS)]
        struct Unit;
        _ = NewS;
    }

    // omit generic struct
    {
        #[omit(NewS)]
        struct S<'a, T> {
            a: &'a T,
        }
        let x = (1, 2, 3);
        _ = NewS { a: &x };
    }
}
