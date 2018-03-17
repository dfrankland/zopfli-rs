extern crate zopfli_rs;
extern crate flate2;

use std::io::prelude::*;
use zopfli_rs::{Options, Format, compress};
use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};

#[test]
fn round_trip_deflate() {
    let input = include_str!("./pokemon.txt");
    let mut output = String::new();

    DeflateDecoder::new(
        compress(
            Options::default(),
            Format::Deflate,
            input.as_bytes(),
        )
    ).read_to_string(
        &mut output
    ).unwrap();

    assert_eq!(input, output);
}

#[test]
fn round_trip_gzip() {
    let input = include_str!("./pokemon.txt");
    let mut output = String::new();

    GzDecoder::new(
        compress(
            Options::default(),
            Format::Gzip,
            input.as_bytes(),
        )
    ).read_to_string(
        &mut output
    ).unwrap();

    assert_eq!(input, output);
}

#[test]
fn round_trip_zlib() {
    let input = include_str!("./pokemon.txt");
    let mut output = String::new();

    ZlibDecoder::new(
        compress(
            Options::default(),
            Format::Zlib,
            input.as_bytes(),
        )
    ).read_to_string(
        &mut output
    ).unwrap();

    assert_eq!(input, output);
}
