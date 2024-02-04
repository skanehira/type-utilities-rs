#![allow(dead_code)]
mod utils;
use types_rs::partial;

test!(ignore_option, {
    #[partial(NewS)]
    struct S {
        a: Option<i32>,
    }
    _ = NewS { a: Some(20) };
});

test!(wrap_type_path, {
    #[partial(NewS)]
    struct S {
        a: i32,
        b: f32,
        c: String,
    }
    _ = NewS {
        a: Some(20),
        b: Some(std::f32::consts::PI),
        c: Some("foo".into()),
    };
});

test!(wrap_type_array, {
    #[partial(NewS)]
    struct S {
        a: [i32; 3],
    }
    _ = NewS { a: Some([1, 2, 3]) };
});

test!(wrap_type_ptr, {
    #[partial(NewS)]
    struct S {
        a: *const i32,
    }
    _ = NewS {
        a: Some(std::ptr::null()),
    };
});

test!(wrap_type_reference, {
    #[partial(NewS)]
    struct S<'a, T> {
        a: &'a T,
    }
    _ = NewS { a: Some(&10) };
});

test!(wrap_type_trait_object, {
    #[partial(NewS)]
    struct S {
        a: &'static dyn Fn() -> i32,
    }
    _ = NewS { a: Some(&|| 10) };
});

test!(wrap_type_tuple, {
    #[partial(NewS)]
    struct S {
        a: (i32, f32),
    }
    _ = NewS { a: Some((10, std::f32::consts::PI)) };
});
