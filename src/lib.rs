// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use tauri::{
  plugin::{Plugin, Result as PluginResult},
  AppHandle, PhysicalPosition, PhysicalSize, Position, RunEvent, Runtime, Size, Window,
  WindowEvent,
};

use std::{
  collections::HashMap,
  fs::{create_dir_all, File},
  io::Write,
  sync::{Arc, Mutex},
};

const STATE_FILENAME: &str = ".window-state";

#[derive(Debug, Default, Deserialize, Serialize)]
struct WindowMetadata {
  width: u32,
  height: u32,
  x: i32,
  y: i32,
  maximized: bool,
}

#[derive(Default)]
pub struct WindowState {
  cache: Arc<Mutex<HashMap<String, WindowMetadata>>>,
}

impl<R: Runtime> Plugin<R> for WindowState {
  fn name(&self) -> &'static str {
    "window-state"
  }

  fn initialize(&mut self, app: &AppHandle<R>, _config: serde_json::Value) -> PluginResult<()> {
    if let Some(app_dir) = app.path_resolver().app_dir() {
      let state_path = app_dir.join(STATE_FILENAME);
      if state_path.exists() {
        self.cache = Arc::new(Mutex::new(
          tauri::api::file::read_binary(state_path)
            .and_then(|state| bincode::deserialize(&state).map_err(Into::into))
            .unwrap_or_default(),
        ));
      }
    }
    Ok(())
  }

  fn created(&mut self, window: Window<R>) {
    {
      let mut c = self.cache.lock().unwrap();
      if let Some(state) = c.get(window.label()) {
        window
          .set_position(Position::Physical(PhysicalPosition {
            x: state.x,
            y: state.y,
          }))
          .unwrap();
        window
          .set_size(Size::Physical(PhysicalSize {
            width: state.width,
            height: state.height,
          }))
          .unwrap();
        if state.maximized {
          window.maximize().unwrap();
        }
      } else {
        let PhysicalSize { width, height } = window.inner_size().unwrap();
        let PhysicalPosition { x, y } = window.outer_position().unwrap();
        let maximized = window.is_maximized().unwrap_or(false);
        c.insert(
          window.label().into(),
          WindowMetadata {
            width,
            height,
            x,
            y,
            maximized,
          },
        );
      }
    }

    let cache = self.cache.clone();
    let label = window.label().to_string();
    let window_clone = window.clone();
    window.on_window_event(move |e| match e {
      WindowEvent::Moved(position) => {
        let mut c = cache.lock().unwrap();
        let state = c.get_mut(&label).unwrap();

        let is_maximized = window_clone.is_maximized().unwrap_or(false);
        state.maximized = is_maximized;

        if let Some(monitor) = window_clone.current_monitor().unwrap() {
          let monitor_position = monitor.position();
          // save only window positions that are inside the current monitor
          if position.x > monitor_position.x && position.y > monitor_position.y && !is_maximized {
            state.x = position.x;
            state.y = position.y;
          };
        };
      }
      WindowEvent::Resized(size) => {
        let mut c = cache.lock().unwrap();
        let state = c.get_mut(&label).unwrap();

        let is_maximized = window_clone.is_maximized().unwrap_or(false);
        state.maximized = is_maximized;

        // It doesn't make sense to save a window with 0 height or width
        if size.width > 0 && size.height > 0 && !is_maximized {
          state.width = size.width;
          state.height = size.height;
        }
      }
      _ => {}
    });

    window.show().unwrap();
    window.set_focus().unwrap();
  }

  fn on_event(&mut self, app: &AppHandle<R>, event: &RunEvent) {
    if let RunEvent::Exit = event {
      if let Some(app_dir) = app.path_resolver().app_dir() {
        let state_path = app_dir.join(STATE_FILENAME);
        let state = self.cache.lock().unwrap();
        let _ = create_dir_all(&app_dir)
          .map_err(tauri::api::Error::Io)
          .and_then(|_| File::create(state_path).map_err(Into::into))
          .and_then(|mut f| {
            f.write_all(&bincode::serialize(&*state).map_err(tauri::api::Error::Bincode)?)
              .map_err(Into::into)
          });
      }
    }
  }
}
