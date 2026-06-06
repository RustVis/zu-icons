
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-ionic.svg)](https://crates.io/crates/dioxus-icons-ionic)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-ionic/badge.svg)](https://docs.rs/dioxus-icons-ionic)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-ionic.svg)

[Ionic icons](https://github.com/ionic-team/ionicons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-ionic)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-ionic = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_ionic::Icon;
use dioxus_icons_ionic::Accessibility;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using ionic icons" }

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
[Ionic Icons](https://github.com/ionic-team/ionicons)|[MIT License](https://github.com/ionic-team/ionicons/blob/main/LICENSE)|[8.0.13](https://github.com/ionic-team/ionicons/tree/v8.0.13)
