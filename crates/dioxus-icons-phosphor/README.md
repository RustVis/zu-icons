
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-phosphor.svg)](https://crates.io/crates/dioxus-icons-bs)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-phosphor/badge.svg)](https://docs.rs/dioxus-icons-bs)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-phosphor.svg)

[Phosphor icons](https://github.com/phosphor-icons/core) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-phosphor)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-phosphor = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_phosphor::Icon;
use dioxus_icons_phosphor::Acorn;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using phosphor icons" }

        Icon {
            icon: Acorn
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Phosphor Icons](https://github.com/phosphor-icons/core)|[MIT License](https://github.com/phosphor-icons/core/blob/main/LICENSE)|[main branch](https://github.com/phosphor-icons/core)
