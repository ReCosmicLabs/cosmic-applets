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

- **`click_last_window`** (new key, `bool`, default `false`). With several windows of one app, a click on
  its icon raises the window that was focused last (or minimizes it if it already has focus) instead of
  opening the window list; the list is still there on hover.

- **`title_badge`** (new key, `bool`, default `false`). Draws an unread counter as a red dot with a
  white number on the bottom-right corner of the icon, ringed by the panel background. The number is
  the larger of two sources: the "(N)" prefix that Discord, WhatsApp and similar apps put in their
  window title, and the `count` of the `com.canonical.Unity.LauncherEntry.Update` D-Bus signal
  (what Electron's `setBadgeCount` emits on Linux when libunity is installed; the same signal KDE's
  task manager reads). The applet also owns the bus name `com.canonical.Unity`, because libunity only
  publishes the counter when that name has an owner (Electron asks `unity_inspector_get_unity_running`
  first); plasmashell does the same. The item keeps its width, so neighbours never shift.
- **`badge_ignored`** (new key, `Vec<String>` of desktop ids, default empty). Apps listed here never get
  the badge, from either source (the dotfiles list `google-chrome`, whose tab titles and download count
  are noise).

Files touched: `cosmic-app-list/cosmic-app-list-config/src/lib.rs`, `cosmic-app-list/src/app.rs`,
`cosmic-app-list/src/launcher_entry.rs`.

Build: `cargo build --release -p cosmic-app-list`; the binary is `target/release/cosmic-app-list`.
The dotfiles install it to `~/.local/bin`, ahead of the distro package.
