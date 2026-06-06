
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-feather.svg)](https://crates.io/crates/dioxus-icons-feather)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-feather/badge.svg)](https://docs.rs/dioxus-icons-feather)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-feather.svg)

[Feather icons](https://github.com/feathericons/feather) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-feather)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-feather = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_feather::Icon;
use dioxus_icons_feather::Airplane;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using feather icons" }

        Icon {
            icon: Airplay
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Feather icons](https://github.com/feathericons/feather)|[MIT License](https://github.com/feathericons/feather/blob/main/LICENSE)|[4.29.2](https://github.com/feathericons/feather/tree/v4.29.2)
