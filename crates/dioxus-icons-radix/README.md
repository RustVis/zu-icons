
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-radix.svg)](https://crates.io/crates/dioxus-icons-radix)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-radix/badge.svg)](https://docs.rs/dioxus-icons-radix)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-radix.svg)

[Radix icons](https://github.com/radix-ui/icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-radix)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-radix = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_radix::Icon;
use dioxus_icons_radix::Accessibility;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using radix icons" }

        Icon {
            icon: Accessibility
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Radix Icons](https://github.com/radix-ui/icons)|[MIT License](https://github.com/radix-ui/icons/blob/main/LICENSE)|[main branch](https://github.com/radix-ui/icons)
