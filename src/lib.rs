// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use tauri::{
  plugin::{Builder as PluginBuilder, TauriPlugin},
  Manager, PhysicalPosition, PhysicalSize, Position, RunEvent, Runtime, Size, Window, WindowEvent,
};

use std::{
  collections::{HashMap, HashSet},
  fs::{create_dir_all, File},
  io::Write,
  sync::{Arc, Mutex},
};

pub const STATE_FILENAME: &str = ".window-state";

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
  #[error(transparent)]
  TauriApi(#[from] tauri::api::Error),
  #[error(transparent)]
  Bincode(#[from] Box<bincode::ErrorKind>),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default, Deserialize, Serialize)]
struct WindowMetadata {
  width: u32,
  height: u32,
  x: i32,
  y: i32,
  maximized: bool,
  visible: bool,
  decorated: bool,
  fullscreen: bool,
}

struct WindowStateCache(Arc<Mutex<HashMap<String, WindowMetadata>>>);
pub trait AppHandleExt {
  fn save_window_state(&self) -> Result<()>;
}

impl<R: Runtime> AppHandleExt for tauri::AppHandle<R> {
  fn save_window_state(&self) -> Result<()> {
    if let Some(app_dir) = self.path_resolver().app_dir() {
      let state_path = app_dir.join(STATE_FILENAME);
      let cache = self.state::<WindowStateCache>();
      let state = cache.0.lock().unwrap();
      create_dir_all(&app_dir)
        .map_err(Error::Io)
        .and_then(|_| File::create(state_path).map_err(Into::into))
        .and_then(|mut f| {
          f.write_all(&bincode::serialize(&*state).map_err(Error::Bincode)?)
            .map_err(Into::into)
        })
    } else {
      Ok(())
    }
  }
}

pub trait WindowExt {
  fn restore_state(&self, auto_show: bool) -> tauri::Result<()>;
}

impl<R: Runtime> WindowExt for Window<R> {
  fn restore_state(&self, auto_show: bool) -> tauri::Result<()> {
    let cache = self.state::<WindowStateCache>();
    let mut c = cache.0.lock().unwrap();
    let mut should_show = true;
    if let Some(state) = c.get(self.label()) {
      self.set_decorations(state.decorated)?;
      self.set_position(Position::Physical(PhysicalPosition {
        x: state.x,
        y: state.y,
      }))?;
      self.set_size(Size::Physical(PhysicalSize {
        width: state.width,
        height: state.height,
      }))?;
      if state.maximized {
        self.maximize()?;
      }
      self.set_fullscreen(state.fullscreen)?;

      should_show = state.visible;
    } else {
      let PhysicalSize { width, height } = self.inner_size()?;
      let PhysicalPosition { x, y } = self.outer_position()?;
      let maximized = self.is_maximized().unwrap_or(false);
      let visible = self.is_visible().unwrap_or(true);
      let decorated = self.is_decorated().unwrap_or(true);
      let fullscreen = self.is_fullscreen().unwrap_or(false);
      c.insert(
        self.label().into(),
        WindowMetadata {
          width,
          height,
          x,
          y,
          maximized,
          visible,
          decorated,
          fullscreen,
        },
      );
    }
    if auto_show && should_show {
      self.show()?;
      self.set_focus()?;
    }

    Ok(())
  }
}

pub struct Builder {
  auto_show: bool,
  blacklist: Option<HashSet<String>>,
}

impl Default for Builder {
  fn default() -> Self {
    Builder {
      auto_show: true,
      blacklist: None,
    }
  }
}

impl Builder {
  /// Whether to enable or disable automatically showing the window
  /// 
  /// - `true`: the window will be automatically shown if the last stored state for visibility was `true`
  /// - `false`: the window will not be automatically shown by this plugin
  pub fn with_auto_show(mut self, auto_show: bool) -> Self {
    self.auto_show = auto_show;
    self
  }

  pub fn with_blacklist(mut self, blacklist: &[&str]) -> Self {
    if !blacklist.is_empty() {
      let mut blacklist_set: HashSet<String> = HashSet::with_capacity(blacklist.len());
      for win in blacklist {
        exlude_set.insert(win.to_string());
      }
      self.blacklist = Some(blacklist_set);
    }
    self
  }

  pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
    PluginBuilder::new("window-state")
      .setup(|app| {
        let cache: Arc<Mutex<HashMap<String, WindowMetadata>>> =
          if let Some(app_dir) = app.path_resolver().app_dir() {
            let state_path = app_dir.join(STATE_FILENAME);
            if state_path.exists() {
              Arc::new(Mutex::new(
                tauri::api::file::read_binary(state_path)
                  .map_err(Error::TauriApi)
                  .and_then(|state| bincode::deserialize(&state).map_err(Into::into))
                  .unwrap_or_default(),
              ))
            } else {
              Default::default()
            }
          } else {
            Default::default()
          };
        app.manage(WindowStateCache(cache));
        Ok(())
      })
      .on_webview_ready(move |window| {
        if let Some(blacklist) = &self.blacklist {
          if blacklist.contains(window.label()) {
            return;
          }
        }
        let _ = window.restore_state(self.auto_show);

        let cache = window.state::<WindowStateCache>();
        let cache = cache.0.clone();
        let label = window.label().to_string();
        let window_clone = window.clone();
        window.on_window_event(move |e| match e {
          WindowEvent::Moved(position) => {
            let mut c = cache.lock().unwrap();
            if let Some(state) = c.get_mut(&label) {
              let is_maximized = window_clone.is_maximized().unwrap_or(false);
              state.maximized = is_maximized;

              if let Some(monitor) = window_clone.current_monitor().unwrap() {
                let monitor_position = monitor.position();
                // save only window positions that are inside the current monitor
                if position.x > monitor_position.x
                  && position.y > monitor_position.y
                  && !is_maximized
                {
                  state.x = position.x;
                  state.y = position.y;
                };
              };
            }
          }
          WindowEvent::Resized(size) => {
            let mut c = cache.lock().unwrap();
            if let Some(state) = c.get_mut(&label) {
              let is_maximized = window_clone.is_maximized().unwrap_or(false);
              let is_fullscreen = window_clone.is_fullscreen().unwrap_or(false);
              state.decorated = window_clone.is_decorated().unwrap_or(true);
              state.maximized = is_maximized;
              state.fullscreen = is_fullscreen;

              // It doesn't make sense to save a window with 0 height or width
              if size.width > 0 && size.height > 0 && !is_maximized {
                state.width = size.width;
                state.height = size.height;
              }
            }
          }
          WindowEvent::CloseRequested { .. } => {
            let mut c = cache.lock().unwrap();
            if let Some(state) = c.get_mut(&label) {
              state.visible = window_clone.is_visible().unwrap_or(true);
            }
          }
          _ => {}
        });
      })
      .on_event(|app, event| {
        if let RunEvent::Exit = event {
          let _ = app.save_window_state();
        }
      })
      .build()
  }
}
