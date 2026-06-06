
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-ant.svg)](https://crates.io/crates/dioxus-icons-ant)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-ant/badge.svg)](https://docs.rs/dioxus-icons-ant)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-ant.svg)

Ant Design icons for dioxus framework

- [Documentation](https://docs.rs/dioxus-icons-ant)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-ant = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_ant::Icon;
use dioxus_icons_ant::filled::AlipayCircle;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using ant design icons" }

        Icon {
            width: "32",
            height: "32",
            icon: AlipayCircle
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Ant Design Icons](https://github.com/ant-design/ant-design-icons)|[MIT License](https://github.com/ant-design/ant-design-icons/blob/master/LICENSE)|[6.2.3](https://github.com/ant-design/ant-design-icons/commit/64ae08f55b6b4cf589854e276ceea0951b69432d)
