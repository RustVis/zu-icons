
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-circum.svg)](https://crates.io/crates/dioxus-icons-circum)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-circum/badge.svg)](https://docs.rs/dioxus-icons-circum)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-circum.svg)

[Circum icons](https://github.com/Klarr-Agency/Circum-icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-circum)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-circum = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_circum::Icon;
use dioxus_icons_circum::AirportSign1,

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using circum icons" }

        Icon {
            width: "32",
            height: "32",
            icon: AirportSign1
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Circum Icons](https://github.com/Klarr-Agency/Circum-icons)|[MPL-2.0 License](https://github.com/Klarr-Agency/Circum-Icons/blob/main/LICENSE)|[2.0.2](https://github.com/Klarr-Agency/Circum-Icons/commit/cec1364b5199f55e946a9a8360385a958b98cc60)
