
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-fa.svg)](https://crates.io/crates/dioxus-icons-fa)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-fa/badge.svg)](https://docs.rs/dioxus-icons-fa)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-fa.svg)

[Font Awesome icons](https://github.com/FortAwesome/Font-Awesome) for Dioxus

- [Document](https://docs.rs/dioxus-icons-fa)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-fa = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_fa::Icon;
use dioxus_icons_fa::regular::AddressBook;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using font awesome design icons" }

        Icon {
            width: "32",
            height: "32",
            icon: AddressBook
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Font Awesome Icons](https://github.com/FortAwesome/Font-Awesome)|[CC BY 4.0 && SIL OFL 1.1](https://github.com/FortAwesome/Font-Awesome/blob/7.x/LICENSE.txt)|[7.2.0](https://github.com/FortAwesome/Font-Awesome/tree/7.2.0)
