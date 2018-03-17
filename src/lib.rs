mod bindings;

use std::slice;
use bindings::{
    ZopfliOptions,
    ZopfliCompress,
    ZopfliFormat_ZOPFLI_FORMAT_DEFLATE,
    ZopfliFormat_ZOPFLI_FORMAT_GZIP,
    ZopfliFormat_ZOPFLI_FORMAT_ZLIB,
};

#[derive(Clone, Debug)]
pub struct Options {
    pub verbose: bool,
    pub verbose_more: bool,
    pub iterations: i32,
    pub block_splitting: i32,
    pub block_splitting_max: i32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            verbose: false,
            verbose_more: false,
            iterations: 15,
            block_splitting: 1,
            block_splitting_max: 15,
        }
    }
}

trait ToZopfliOptions {
    fn to_zopfli_options(self: &Self) -> ZopfliOptions;
}

impl ToZopfliOptions for Options {
    fn to_zopfli_options(self: &Self) -> ZopfliOptions {
        ZopfliOptions {
            verbose: self.verbose as i32,
            verbose_more: self.verbose_more as i32,
            numiterations: self.iterations,
            blocksplitting: self.block_splitting,
            blocksplittinglast: 0,
            blocksplittingmax: self.block_splitting_max,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Format {
    Deflate,
    Gzip,
    Zlib,
}

pub fn compress(options: Options, format: Format, input: &[u8]) -> &[u8] {
    let input_ptr = input.as_ptr();
    let input_size = input.len();

    let mut output: Vec<u8> = Vec::with_capacity(input_size);
    let mut output_ptr = output.as_mut_ptr();
    let mut output_size: usize = 0;

    unsafe {
        ZopfliCompress(
            &options.to_zopfli_options(),
            match format {
                Format::Deflate => ZopfliFormat_ZOPFLI_FORMAT_DEFLATE,
                Format::Gzip => ZopfliFormat_ZOPFLI_FORMAT_GZIP,
                Format::Zlib => ZopfliFormat_ZOPFLI_FORMAT_ZLIB,
            },
            input_ptr,
            input_size,
            &mut output_ptr,
            &mut output_size,
        );

        slice::from_raw_parts(output_ptr, output_size)
    }
}
