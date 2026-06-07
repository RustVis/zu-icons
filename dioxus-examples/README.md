# Dioxus Icons Demo

A demo application showcasing icon packs for the [Dioxus](https://dioxuslabs.com/learn/0.7) framework.

## Supported Icon Packs

| Page | Icon Crate | Description |
|------|-----------|-------------|
| Ant Icons | `dioxus-icons-ant` | Ant Design icons (filled, outlined, twotone) |
| Box Icons | `dioxus-icons-box` | Box Icons |
| Bootstrap Icons | `dioxus-icons-bs` | Bootstrap icons |
| Circum Icons | `dioxus-icons-circum` | Circum icons |
| Dev Icons | `dioxus-icons-dev` | Devicons |
| FA Icons | `dioxus-icons-fa` | Font Awesome icons |
| Feather Icons | `dioxus-icons-feather` | Feather icons |
| Game Icons | `dioxus-icons-game` | Game icons |
| Grommet Icons | `dioxus-icons-grommet` | Grommet icons |
| Hero Icons | `dioxus-icons-hero` | Heroicons |
| Ionic Icons | `dioxus-icons-ionic` | Ionicons |
| Line Awesome Icons | `dioxus-icons-line-awesome` | Line Awesome icons |
| Lucide Icons | `dioxus-icons-lucide` | Lucide icons |
| Material Icons | `dioxus-icons-md` | Material Design icons |
| Octicon Icons | `dioxus-icons-oct` | GitHub Octicons |
| Phosphor Icons | `dioxus-icons-phosphor` | Phosphor icons |
| Radix Icons | `dioxus-icons-radix` | Radix UI icons |
| Remix Icons | `dioxus-icons-remix` | Remix icons |
| Simple Icons | `dioxus-icons-simple` | Simple Icons (brand/technology SVGs) |
| Tabler Icons | `dioxus-icons-tabler` | Tabler icons (outline, filled) |
| VSC Icons | `dioxus-icons-vsc` | VS Code codicons |

## Project Structure

```
project/
├─ assets/          # Static assets (favicon, CSS, header)
├─ src/
│  ├─ main.rs       # Entry point
│  ├─ app.rs        # Router, layout, navbar
│  ├─ pages/
│  │  ├─ mod.rs
│  │  ├─ home_page.rs
│  │  ├─ ant_page.rs
│  │  ├─ bootstrap_page.rs
│  │  ├─ box_page.rs
│  │  ├─ circum_page.rs
│  │  ├─ dev_page.rs
│  │  ├─ fa_page.rs
│  │  ├─ feather_page.rs
│  │  ├─ game_page.rs
│  │  ├─ grommet_page.rs
│  │  ├─ hero_page.rs
│  │  ├─ ionic_page.rs
│  │  ├─ line_awesome_page.rs
│  │  ├─ lucide_page.rs
│  │  ├─ material_page.rs
│  │  ├─ octicon_page.rs
│  │  ├─ phosphor_page.rs
│  │  ├─ radix_page.rs
│  │  ├─ remix_page.rs
│  │  ├─ simple_page.rs
│  │  ├─ tabler_page.rs
│  │  └─ vsc_page.rs
├─ Cargo.toml
├─ Dioxus.toml
└─ tailwind.css
```

## Getting Started

### Prerequisites

Install the Dioxus CLI:

```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

### Serve

Run the following command to start the development server:

```bash
dx serve
```

To run for a different platform:

```bash
dx serve --platform desktop
```

### Tailwind CSS

As of Dioxus 0.7, Tailwind is processed automatically. A `tailwind.css` file in the project root is detected automatically by `dx serve`.

To customize, edit `tailwind.css` or configure `Dioxus.toml`:

```toml
[application]
tailwind_input = "tailwind.css"
tailwind_output = "assets/out.css"
```
