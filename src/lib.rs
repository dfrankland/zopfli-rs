mod bindings;

use std::io::{self, Write};
use std::slice;
use bindings::{
    ZopfliOptions,
    ZopfliCompress,
    ZopfliFormat_ZOPFLI_FORMAT_DEFLATE,
    ZopfliFormat_ZOPFLI_FORMAT_GZIP,
    ZopfliFormat_ZOPFLI_FORMAT_ZLIB,
};

/// Options used to tweak optimization versus speed of compression.
#[derive(Copy, Clone, Debug)]
pub struct Options {
    /// Prints statisitcs on compression to the console. Default: `false`.
    pub verbose: bool,

    /// Prints other statisitcs on block splitting et cetra to the console. Default: `false`.
    pub verbose_more: bool,

    /// Maximum amount of times to rerun forward and backward pass to optimize LZ77 compression
    /// cost. Good values: `10`, `15` for small files, `5` for files over several MB in size or it
    /// will be too slow. Default: `15`.
    pub iterations: i32,

    /// If `true`, splits the data in multiple deflate blocks with optimal choice
    /// for the block boundaries. Block splitting gives better compression. Default: `true`.
    pub block_splitting: bool,

    /// Maximum amount of blocks to split into (`0` for unlimited, but this can give extreme
    /// results that hurt compression on some files). Default: `15`.
    pub block_splitting_max: i32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            verbose: false,
            verbose_more: false,
            iterations: 15,
            block_splitting: true,
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
            blocksplitting: self.block_splitting as i32,
            blocksplittinglast: 0,
            blocksplittingmax: self.block_splitting_max,
        }
    }
}

/// Output format (`deflate`, `gzip`, `zlib`).
#[derive(Clone, Debug)]
pub enum Format {
    /// `deflate` format <http://www.ietf.org/rfc/rfc1951.txt>
    Deflate,

    /// `gzip` format <http://www.ietf.org/rfc/rfc1952.txt>
    Gzip,

    /// `zlib` format <http://www.ietf.org/rfc/rfc1950.txt>
    Zlib,
}

/// Compresses according to the given output format and appends the result to the output.
///
/// # Arguments
///
/// * `options` - An `Options` `struct`.
///
/// * `format` - A `Format` `enum`.
///
/// * `input` - A `slice` of data to compress and write to `ouput`.
///
/// * `output` - Any type which implements `Write` that the resulting compressed data from `input`
///   will be written to.
///
pub fn compress<W: Write>(options: &Options, format: &Format, input: &[u8], mut output: W) -> io::Result<()> {
    let input_ptr = input.as_ptr();
    let input_size = input.len();

    let mut out: Vec<u8> = Vec::with_capacity(input_size);
    let mut out_ptr = out.as_mut_ptr();
    let mut out_size: usize = 0;

    unsafe {
        ZopfliCompress(
            &options.to_zopfli_options(),
            match *format {
                Format::Deflate => ZopfliFormat_ZOPFLI_FORMAT_DEFLATE,
                Format::Gzip => ZopfliFormat_ZOPFLI_FORMAT_GZIP,
                Format::Zlib => ZopfliFormat_ZOPFLI_FORMAT_ZLIB,
            },
            input_ptr,
            input_size,
            &mut out_ptr,
            &mut out_size,
        );

        output.by_ref().write_all(slice::from_raw_parts(out_ptr, out_size))
    }
}
