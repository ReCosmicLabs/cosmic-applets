# cosmic-applets (fork)

> **This is a fork of [pop-os/cosmic-applets](https://github.com/pop-os/cosmic-applets)**, the panel applets
> of the COSMIC desktop by [System76](https://system76.com). All credit for the applets themselves goes to
> System76 and the upstream contributors. The license is unchanged: **GPL-3.0-only** (see `LICENSE`).

## Changes in this fork

Maintained by [ReCosmicLabs](https://github.com/ReCosmicLabs) for the
[dotfiles](https://github.com/eualexandrerrr/dotfiles) setup. Only `cosmic-app-list` is modified; everything
below is a modification of the original work, as required by section 5 of the GPL.

- **`ignored`** (new key in `com.system76.CosmicAppList/v1`, `Vec<String>`, default empty). Windows whose
  `app_id` is in this list never appear in the task list, even while running. Wayland clients cannot ask to
  be skipped from the taskbar, so this is the compositor-side equivalent of `skipTaskbar`. Match is
  case-insensitive.
- **`show_divider`** (new key, `bool`, default `true`). When `false`, the divider between pinned favorites
  and running windows is not drawn.
- **`hover_popup_delay_ms`** (new key, `Option<u32>`, default `None`). When set, resting the pointer on
  the icon of an app with two or more windows opens its window list after that many milliseconds, and
  the list closes shortly after the pointer leaves both the icon and the list. `None` keeps click-only.

Files touched: `cosmic-app-list/cosmic-app-list-config/src/lib.rs`, `cosmic-app-list/src/app.rs`.

Build: `cargo build --release -p cosmic-app-list`; the binary is `target/release/cosmic-app-list`.
The dotfiles install it to `~/.local/bin`, ahead of the distro package.
