# cervine

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/cervine)
[![Crates.io](https://img.shields.io/crates/v/cervine)](https://crates.io/crates/cervine)
[![Docs.rs](https://img.shields.io/badge/Docs.rs-*-black)](https://docs.rs/crates/cervine)

![Rust 1.46.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.46.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/cervine.svg?branch=develop)](https://travis-ci.com/Tamschi/cervine/branches)
![Crates.io - License](https://img.shields.io/crates/l/cervine/0.0.2)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/cervine)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/cervine)](https://github.com/Tamschi/cervine/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/cervine)](https://github.com/Tamschi/cervine/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cervine.svg)](https://web.crev.dev/rust-reviews/crate/cervine/)

A slightly more flexible Cow; to [`AsRef`] as [`alloc::borrow::Cow`] is to [`Borrow`].

The owned and reference types can be chosen independently, which means for example [smartstring]'s [`String`] can be used in the owned variant instead of [`alloc`'s].

[Serde] support is optional via the `"serde"` feature and `no_std`-compatible.  
Note that deserialisation currently always happens by value. This may change in a major version upgrade after [specialization] becomes available.

[`AsRef`]: https://doc.rust-lang.org/stable/core/convert/trait.AsRef.html
[`alloc::borrow::Cow`]: https://doc.rust-lang.org/stable/alloc/borrow/enum.Cow.html
[`Borrow`]: https://doc.rust-lang.org/stable/alloc/borrow/trait.Borrow.html

[smartstring]: https://lib.rs/crates/smartstring
[`String`]: https://docs.rs/smartstring/0.2.3/smartstring/alias/type.String.html
[`alloc`'s]: https://doc.rust-lang.org/stable/alloc/string/struct.String.html

[Serde]: https://lib.rs/crates/serde
[specialization]: https://github.com/rust-lang/rust/issues/31844

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add cervine
```

## Example

```rust
use cervine::Cow;
use core::borrow::Borrow as _;
use rand::prelude::*;

let data = [true, false];
let mut cow = Cow::Borrowed(&data);

if thread_rng().gen() {
  cow = Cow::Owned([false, true]);
}

let array_ref: &[bool; 2] = cow.borrow();
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`cervine` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
