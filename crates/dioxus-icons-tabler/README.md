
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-tabler.svg)](https://crates.io/crates/dioxus-icons-tabler)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-tabler/badge.svg)](https://docs.rs/dioxus-icons-tabler)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-tabler.svg)

[Tabler icons](https://github.com/tabler/tabler-icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-tabler)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-tabler = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_tabler::Icon;
use dioxus_icons_tabler::AB;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using tabler icons" }

        Icon {
            icon: AB
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Tabler Icons](https://github.com/tabler/tabler-icons)|[MIT License](https://github.com/tabler/tabler-icons/blob/main/LICENSE)|[3.44.0](https://github.com/tabler/tabler-icons/tree/v3.44.0)
