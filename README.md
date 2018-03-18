# zopfli-rs

Bindings to `zopfli` for deflate, gzip, and zlib compression.

## Purpose

`zopfli` is already implemented in pure Rust by `@carols10cents`
([`carols10cents/zopfli`][carols10cents/zopfli]), so it might seem duplicative
to create bindings to the original C / C++ library by Google
([`google/zopfli`][google/zopfli]). In order to give better insights into the
performance of the pure Rust library and to give a slightly faster alternative,
`zopfli-rs` was made.

## Benchmarks

`zopfli-rs` just slightly inches ahead of `zopfli` when it comes to gzip
compression. Both `zopfli-rs` and `zopfli` follow a strange performance curve
that seem to be faster for certain sizes of data (in this case ~10KB is the
sweet spot). Changing up the `Options` would more than likely improve these
results, but that would take a bit of time to tweak to perfection.

```
test bench_zopfli_1         ... bench:   5,026,521 ns/iter (+/- 547,023)
test bench_zopfli_10        ... bench:   5,817,841 ns/iter (+/- 585,086)
test bench_zopfli_100       ... bench:  19,293,553 ns/iter (+/- 2,106,929)
test bench_zopfli_1000      ... bench: 290,497,031 ns/iter (+/- 32,570,971)
test bench_zopfli_10000     ... bench:  38,902,158 ns/iter (+/- 6,040,615)
test bench_zopfli_100000    ... bench: 301,934,186 ns/iter (+/- 17,868,991)
test bench_zopfli_rs_1      ... bench:   2,439,766 ns/iter (+/- 644,219)
test bench_zopfli_rs_10     ... bench:   3,299,515 ns/iter (+/- 359,117)
test bench_zopfli_rs_100    ... bench:  14,146,432 ns/iter (+/- 1,505,380)
test bench_zopfli_rs_1000   ... bench: 209,671,685 ns/iter (+/- 9,135,909)
test bench_zopfli_rs_10000  ... bench:  29,229,623 ns/iter (+/- 3,060,932)
test bench_zopfli_rs_100000 ... bench: 289,308,248 ns/iter (+/- 13,868,834)
```

[carols10cents/zopfli]: https://github.com/carols10cents/zopfli
[google/zopfli]: https://github.com/google/zopfli
