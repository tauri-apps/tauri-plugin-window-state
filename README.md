# Tauri Plugin Window State
![Test](https://github.com/tauri-apps/tauri-plugin-window-state/workflows/Test/badge.svg)

This plugin provides a Tauri Plugin that saves the window position and size and restores it when the app is reopened.

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the vanilla demo](examples/vanilla/src-tauri/src/main.rs).
Please note, below in the dependencies you can also lock to a revision/tag in the `Cargo.toml`.

`src-tauri/Cargo.toml`
```yaml
[dependencies.tauri-plugin-window-state]
git = "https://github.com/tauri-apps/tauri-plugin-window-state"
tag = "v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:
```rust
fn main() {
    tauri::AppBuilder::new()
        .plugin(tauri_plugin_window_state::WindowState::default())
        .build()
        .run();
}
```

To prevent flashes when the window is updated, the window `visible` property must be set to `false`.
The plugin is responsible for showing it after restoring its state.

# License
MIT / Apache-2.0
