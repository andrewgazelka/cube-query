#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use cube_query_lib::Args;

#[bench]
fn bench_pow(b: &mut Bencher) {
    let args = Args {
        chip: "stm32h743zitx".to_string(),
        filter: Some("eth".to_string()),
    };

    b.iter(|| {
        // Inner closure, the actual test
        let res = cube_query_lib::run(&args);
        black_box(res).unwrap();
    });
}
