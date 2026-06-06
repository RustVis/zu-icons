
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-simple.svg)](https://crates.io/crates/dioxus-icons-simple)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-simple/badge.svg)](https://docs.rs/dioxus-icons-simple)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-simple.svg)

[Simple icons](https://github.com/simple-icons/simple-icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-simple)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-simple = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_simple::Icon;
use dioxus_icons_simple::Abb;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using simple icons" }

        Icon {
            icon: Abb
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Simple Icons](https://github.com/simple-icons/simple-icons)|[CC0-1.0 License](https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md)|[16.22.0](https://github.com/simple-icons/simple-icons/tree/16.22.0)
