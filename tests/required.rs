#![allow(dead_code)]
mod utils;
use type_utilities_rs::required;

test!(require_optional_field, {
    {
        #[required(NewS)]
        struct S<'a> {
            a: Option<i32>,
            b: Option<&'a str>,
            c: f32,
        }
        _ = S {
            a: Some(10),
            b: Some("foo"),
            c: 1.5,
        };
        _ = NewS {
            a: 1,
            b: "hello",
            c: 1.1,
        };
    }
});
