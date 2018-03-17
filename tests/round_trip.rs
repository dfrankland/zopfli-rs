extern crate zopfli_rs;
extern crate flate2;

use zopfli_rs::bindings::*;
use std::slice;
use std::os::raw::c_uchar;
use std::io::prelude::*;
use flate2::read::GzDecoder;

#[test]
fn round_trip_compression_decompression() {
    unsafe {
        let quotes = include_str!("./pokemon.txt");

        let input = quotes.as_bytes();
        let input_ptr = input.as_ptr();
        let input_size = input.len();

        let mut output: Vec<c_uchar> = Vec::with_capacity(input_size);
        let mut output_ptr = output.as_mut_ptr();
        let mut output_size: usize = 0;

        let zopfli_options = ZopfliOptions {
            verbose: 0,
            verbose_more: 0,
            numiterations: 15,
            blocksplitting: 1,
            blocksplittinglast: 0,
            blocksplittingmax: 15,
        };

        ZopfliCompress(
            &zopfli_options,
            ZopfliFormat_ZOPFLI_FORMAT_GZIP,
            input_ptr,
            input_size,
            &mut output_ptr,
            &mut output_size,
        );

        let compressed_data = slice::from_raw_parts(output_ptr, output_size);

        let mut gzip_decoder = GzDecoder::new(compressed_data);
        let mut output_decoded = String::new();
        gzip_decoder.read_to_string(&mut output_decoded).unwrap();

        assert_eq!(quotes, output_decoded);
    }
}
