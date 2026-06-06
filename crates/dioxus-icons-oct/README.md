
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-oct.svg)](https://crates.io/crates/dioxus-icons-oct)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-oct/badge.svg)](https://docs.rs/dioxus-icons-oct)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-oct.svg)

[Octicons](https://github.com/primer/octicons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-oct)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-oct = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_oct::Icon;
use dioxus_icons_oct::Airplane;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using Octicons icons" }

        Icon {
            icon: Accessibility16
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Octicons Icons](https://github.com/primer/octicons)|[MIT License](https://github.com/primer/octicons/blob/main/LICENSE)|[19.28.0](https://github.com/primer/octicons/tree/v19.28.0)
