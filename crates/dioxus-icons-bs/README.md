
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-bs.svg)](https://crates.io/crates/dioxus-icons-bs)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-bs/badge.svg)](https://docs.rs/dioxus-icons-bs)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-bs.svg)

[Bootstrap icons](https://github.com/twbs/icons) for dioxus

- [Document](https://docs.rs/dioxus-icons-bs)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-bs = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_bs::Icon;
use dioxus_icons_bs::Airplane;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using bs icons" }

        Icon {
            icon: Airplane
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Bootstrap Icons](https://github.com/twbs/icons)|[MIT License](https://github.com/twbs/icons/blob/main/LICENSE)|[1.13.1](https://github.com/twbs/icons/tree/v1.13.1)
