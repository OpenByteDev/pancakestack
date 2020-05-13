use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::*;
use std::str;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("text-long", |b| {
        b.iter(|| {
            let mut file = File::open("examples/long.pancake").unwrap();
            let mut program = String::new();
            file.read_to_string(&mut program).unwrap();
            let mut output_buf = Vec::new();
            pancakestack::run_program_str(&program, empty(), &mut output_buf).unwrap();
            let output = str::from_utf8(&output_buf).unwrap();
            assert_eq!(output, "\x04");
        })
    });

    c.bench_function("read-long", |b| {
        b.iter(|| {
            let file = File::open("examples/long.pancake").unwrap();
            let mut output_buf = Vec::new();
            pancakestack::run_program_from_read(file, empty(), &mut output_buf).unwrap();
            let output = str::from_utf8(&output_buf).unwrap();
            assert_eq!(output, "\x04");
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
