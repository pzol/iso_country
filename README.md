# IsoCountry

Country codes,  their numbers and names.

The crate supports `no_std` targets with `alloc` when its default `std` feature is disabled. The optional `serde` feature works in both configurations.

## What is ISO 3166-1?

> ISO 3166-1 is part of the ISO 3166 standard published by the International
> Organization for Standardization (ISO), and defines codes for the names of
> countries, dependent territories, and special areas of geographical
> interest.
>
> - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)


## Benchmarks

```
test parse      ... bench:          44 ns/iter (+/- 13)
test parse_name ... bench:          50 ns/iter (+/- 16)
```
