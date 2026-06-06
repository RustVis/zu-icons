
# About

[![Latest version](https://img.shields.io/crates/v/dioxus-icons-game.svg)](https://crates.io/crates/dioxus-icons-game)
![Build status](https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/dioxus-icons-game/badge.svg)](https://docs.rs/dioxus-icons-game)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.88+-green.svg)
![License](https://img.shields.io/crates/l/dioxus-icons-game.svg)

[Game icons](https://github.com/game-icons/icons) for dioxus framework

- [Document](https://docs.rs/dioxus-icons-game)

## Example

First add to dependencies:

```toml
[dependencies]
dioxus-icons-game = { "0.2" }
```

Then use svg icon component container and icon paths

```rust
use dioxus::prelude::*;
use dioxus_icons_game::Icon;
use dioxus_icons_game::logos::BxlAdobe;

#[comonent]
fn page() -> Element {
    rsx!{
        h1 { "Using game icons" }

        Icon {
            icon: BxlAdobe 
        }
    }
}

```

## License

This project is licensed under the Apache-2.0 license.


## Icon license

Icon Library|License|Version
---|---|---
[Game Icons](https://github.com/game-icons/icons)|[Creative Commons 3.0 License](https://github.com/game-icons/icons/blob/master/license.txt)|[master branch](https://github.com/game-icons/icons/commit/82d948812bfe3f269ef8f731dcdb07b08160edc4)
