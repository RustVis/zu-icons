
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-box.svg)](https://crates.io/crates/dioxus-icons-box)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-box/badge.svg)](https://docs.rs/dioxus-icons-box)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-box.svg)

Ant Design icons for dioxus framework

- [Documentation](https://docs.rs/dioxus-icons-box)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-box = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_box::Icon;
use dioxus_icons_box::solid::BxsAddToQueue;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using box icons" }

        Icon {
            width: "32",
            height: "32",
            icon: BxsAddToQueue
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Box Icons](https://github.com/box-design/box-design-icons)|[MIT License](https://github.com/box-design/box-design-icons/blob/master/LICENSE)|[6.2.3](https://github.com/box-design/box-design-icons/commit/64ae08f55b6b4cf589854e276ceea0951b69432d)
