
# About

Ant Design icons for dioxus framework

## Example

First adding to dependencies:
```toml
[dependencies]
dioxus-icons-ant = { "0.2" }
```

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
