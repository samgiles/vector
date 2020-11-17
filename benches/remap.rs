use criterion::{criterion_group, criterion_main};
use remap::prelude::*;

criterion_group!(functions, upcase, downcase, parse_json);
criterion_main!(functions);

bench_function! {
    upcase => vector::remap::Upcase;

    literal_value {
        args: func_args![value: "foo"],
        want: Ok("FOO")
    }
}

bench_function! {
    downcase => vector::remap::Downcase;

    literal_value {
        args: func_args![value: "FOO"],
        want: Ok("foo")
    }
}

bench_function! {
    parse_json => vector::remap::ParseJson;

    literal_value {
        args: func_args![value: r#"{"key": "value"}"#],
        want: Ok(map!["key": "value"]),
    }

    invalid_json_with_default {
        args: func_args![
            value: r#"{"key": INVALID}"#,
            default: r#"{"key": "default"}"#,
        ],
        want: Ok(map!["key": "default"]),
    }
}
