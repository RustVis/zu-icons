
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-line-awesome.svg)](https://crates.io/crates/dioxus-icons-line-awesome)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-line-awesome/badge.svg)](https://docs.rs/dioxus-icons-line-awesome)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-line-awesome.svg)

Icon8 [Line awesome icons](https://github.com/icons8/line-awesome) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-line-awesome)

## Example

First add to dependencies:
```toml
[dependencies]
dioxus-icons-line-awesome = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_line_awesome::Icon;
use dioxus_icons_line_awesome::AccessibleIcon;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using line awesome icons" }

        Icon {
            icon: AccessibleIcon
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Line awesome Icons](https://github.com/icons8/line-awesome)|[MIT or Good Boy License](https://github.com/icons8/line-awesome/blob/master/LICENSE.md)|[1.3.1](https://github.com/icons8/line-awesome/commit/78a101217707c9b1c4dcf2a821be75684e36307f)
