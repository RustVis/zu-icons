
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-grommet.svg)](https://crates.io/crates/dioxus-icons-grommet)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-grommet/badge.svg)](https://docs.rs/dioxus-icons-grommet)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-grommet.svg)

[Grommet icons](https://github.com/grommet/grommet-icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-grommet)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-grommet = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_grommet::Icon;
use dioxus_icons_grommet::Accessibility;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using grommet icons" }

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
[Grommet Icons](https://github.com/grommet/grommet-icons)|[Apache-2.0 License](https://github.com/grommet/grommet-icons/blob/master/LICENSE)|[4.14.0](https://github.com/grommet/grommet-icons/tree/v4.14.0)
