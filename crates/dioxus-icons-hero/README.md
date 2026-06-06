
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-hero.svg)](https://crates.io/crates/dioxus-icons-hero)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-hero/badge.svg)](https://docs.rs/dioxus-icons-hero)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-hero.svg)

[Tailwind hero icons] for dioxus framework

- [Document](https://docs.rs/dioxus-icons-hero)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-hero = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_hero::Icon;
use dioxus_icons_hero::solid::AcademicCap;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using hero icons" }

        Icon {
            icon: AcademicCap
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Hero Icons](https://github.com/tailwindlabs/heroicons)|[MIT License](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE)|[2.2.0](https://github.com/tailwindlabs/heroicons/tree/v2.2.0)
