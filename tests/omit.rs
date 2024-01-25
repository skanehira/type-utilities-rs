#![allow(dead_code)]
use types_rs::omit;

#[test]
fn test() {
    // omit partial field
    {
        mod foo {
            use super::*;
            #[omit(NewFoo, [b])]
            pub struct Foo {
                pub a: i32,
                pub b: &str,
            }
        }
        _ = foo::NewFoo { a: 1 };
    }

    // omit all fields
    {
        #[omit(NewFoo2, [a, b])]
        struct Foo {
            a: i32,
            b: &str,
        }
        _ = NewFoo2;
    }

    // skip non-existent field
    {
        #[omit(NewFoo3, [c])]
        struct Foo {
            a: i32,
            b: &'static str,
        }
        _ = NewFoo3 {
            a: 1,
            b: "2",
        };
    }

    // omit unit struct
    {
        #[omit(NewFoo4, [])]
        struct Foo;
        _ = NewFoo4;
    }

    // TODO: support generics
    // omit generic struct
    //{
    //    #[omit(NewFoo5, [])]
    //    struct Foo<T> {
    //        a: T,
    //    }
    //    _ = NewFoo5 { a: 1 };
    //}
}
