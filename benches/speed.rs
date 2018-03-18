#![feature(test)]

extern crate test;
extern crate rand;
extern crate zopfli;
extern crate zopfli_rs;
#[macro_use] extern crate lazy_static;

use test::Bencher;
use rand::Rng;

lazy_static! {
    static ref DATA_1: Vec<u8> = get_rand_data(1);
    static ref DATA_10: Vec<u8> = get_rand_data(10);
    static ref DATA_100: Vec<u8> = get_rand_data(100);
    static ref DATA_1000: Vec<u8> = get_rand_data(1000);
    static ref DATA_10000: Vec<u8> = get_rand_data(10000);
    static ref DATA_100000: Vec<u8> = get_rand_data(100_000);
}

fn get_rand_data(len: usize) -> Vec<u8> {
    rand::thread_rng()
        .gen_iter::<u8>()
        .take(len)
        .collect()
}

fn zopfli_gzip(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0; data.len()];

    zopfli::compress(
        &zopfli::Options::default(),
        &zopfli::Format::Gzip,
        data,
        &mut out,
    ).unwrap();

    out
}

fn zopfli_rs_gzip(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0; data.len()];

    zopfli_rs::compress(
        &zopfli_rs::Options::default(),
        &zopfli_rs::Format::Gzip,
        data,
        &mut out,
    ).unwrap();

    out
}

#[bench]
fn bench_zopfli_1(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_1));
}

#[bench]
fn bench_zopfli_10(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_10));
}

#[bench]
fn bench_zopfli_100(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_100));
}

#[bench]
fn bench_zopfli_1000(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_1000));
}

#[bench]
fn bench_zopfli_10000(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_10000));
}

#[bench]
fn bench_zopfli_100000(b: &mut Bencher) {
    b.iter(|| zopfli_gzip(&DATA_100000));
}

#[bench]
fn bench_zopfli_rs_1(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_1));
}

#[bench]
fn bench_zopfli_rs_10(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_10));
}

#[bench]
fn bench_zopfli_rs_100(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_100));
}

#[bench]
fn bench_zopfli_rs_1000(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_1000));
}

#[bench]
fn bench_zopfli_rs_10000(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_10000));
}

#[bench]
fn bench_zopfli_rs_100000(b: &mut Bencher) {
    b.iter(|| zopfli_rs_gzip(&DATA_100000));
}
