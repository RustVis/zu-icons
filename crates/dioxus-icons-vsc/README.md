
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-vsc.svg)](https://crates.io/crates/dioxus-icons-vsc)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-vsc/badge.svg)](https://docs.rs/dioxus-icons-vsc)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-vsc.svg)

[Visual Studio Code icons](https://github.com/microsoft/vscode-codicons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-vsc)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-vsc = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_vsc::Icon;
use dioxus_icons_vsc::Account

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using vsc icons" }

        Icon {
            icon: Account
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[VSC Codicons](https://github.com/microsoft/vscode-codicons)|[MIT && CC-BY-4.0 License](https://github.com/microsoft/vscode-codicons/blob/main/LICENSE)|[0.0.45-12](https://github.com/microsoft/vscode-codicons/tree/v0.0.45-12)
