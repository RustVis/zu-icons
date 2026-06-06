# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-dev.svg)](https://crates.io/crates/dioxus-icons-dev)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-dev/badge.svg)](https://docs.rs/dioxus-icons-dev)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-dev.svg)

[Dev icons](https://github.com/vorillaz/devicons) for dioxus framework.
A collection of icons for developers.

- [Document](https://docs.rs/dioxus-icons-dev)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-dev = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_dev::Icon;
use dioxus_icons_dev::Adonisjs

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using dev icons" }

        Icon {
            width: "32",
            height: "32",
            icon: Adonisjs
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Dev Icons](https://github.com/vorillaz/devicons)|[MIT License](https://github.com/vorillaz/devicons/blob/main/LICENSE)|[1.1.0](https://github.com/vorillaz/devicons/tree/v1.1.0)
