
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-remix.svg)](https://crates.io/crates/dioxus-icons-remix)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-remix/badge.svg)](https://docs.rs/dioxus-icons-remix)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-remix.svg)

[Remix icons](https://github.com/Remix-Design/RemixIcon) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-remix)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-remix = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_remix::Icon;
use dioxus_icons_remix::logos::AlibabaCloudFill;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using remix icons" }

        Icon {
            icon: AlibabaCloudFill
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Remix Icons](https://github.com/Remix-Design/RemixIcon)|[MIT License](https://github.com/Remix-Design/RemixIcon/blob/master/License)|[4.9.1](https://github.com/Remix-Design/RemixIcon/tree/v4.9.1)
