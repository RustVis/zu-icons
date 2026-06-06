
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-lucide.svg)](https://crates.io/crates/dioxus-icons-lucide)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-lucide/badge.svg)](https://docs.rs/dioxus-icons-lucide)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-lucide.svg)

[Lucide icons](https://github.com/lucide-icons/lucide) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-lucide)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-lucide = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_lucide::Icon;
use dioxus_icons_lucide::AArrowDown;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using lucide icons" }

        Icon {
            icon: AArrowDown
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Lucide Icons](https://github.com/lucide-icons/lucide)|[ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE)|[1.17.0](https://github.com/lucide-icons/lucide/tree/1.17.0)
