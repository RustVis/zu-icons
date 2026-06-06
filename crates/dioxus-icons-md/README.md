
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-md.svg)](https://crates.io/crates/dioxus-icons-md)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-md/badge.svg)](https://docs.rs/dioxus-icons-md)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-md.svg)

[Material design icons](https://github.com/google/material-design-icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-md)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-md = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_md::Icon;
use dioxus_icons_md::outlined::Abc;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using material design icons icons" }

        Icon {
            icon: Abc
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Material Design Icons](https://github.com/google/material-design-icons)|[Apache-2.0 License](https://github.com/google/material-design-icons/blob/master/LICENSE)|[master branch](https://github.com/google/material-design-icons)
