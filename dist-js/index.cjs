'use strict';

var core = require('@tauri-apps/api/core');
var window = require('@tauri-apps/api/window');

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * Save and restore window positions and sizes.
 *
 * @module
 */
/**
 * Flags controlling which parts of a window's state are saved and restored.
 * Combine multiple flags with the bitwise OR operator (`|`).
 */
exports.StateFlags = void 0;
(function (StateFlags) {
    /** Save and restore the window size. */
    StateFlags[StateFlags["SIZE"] = 1] = "SIZE";
    /** Save and restore the window position. */
    StateFlags[StateFlags["POSITION"] = 2] = "POSITION";
    /** Save and restore whether the window is maximized. */
    StateFlags[StateFlags["MAXIMIZED"] = 4] = "MAXIMIZED";
    /** Save and restore whether the window is visible. */
    StateFlags[StateFlags["VISIBLE"] = 8] = "VISIBLE";
    /** Save and restore whether the window has decorations. */
    StateFlags[StateFlags["DECORATIONS"] = 16] = "DECORATIONS";
    /** Save and restore whether the window is fullscreen. */
    StateFlags[StateFlags["FULLSCREEN"] = 32] = "FULLSCREEN";
    /** Save and restore every flag above. */
    StateFlags[StateFlags["ALL"] = 63] = "ALL";
})(exports.StateFlags || (exports.StateFlags = {}));
/**
 *  Save the state of all open windows to disk.
 *
 * @example
 * ```typescript
 * import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';
 *
 * await saveWindowState(StateFlags.ALL);
 * ```
 *
 * @param flags Which parts of the state to save. Defaults to the flags passed to the plugin `Builder` (all flags if none were set).
 * @since 2.0.0
 */
async function saveWindowState(flags) {
    await core.invoke('plugin:window-state|save_window_state', { flags });
}
/**
 *  Restore the state for the specified window from disk.
 *
 * @example
 * ```typescript
 * import { restoreState, StateFlags } from '@tauri-apps/plugin-window-state';
 *
 * await restoreState('main', StateFlags.ALL);
 * ```
 *
 * @param label The label of the window to restore.
 * @param flags Which parts of the state to restore. Defaults to the flags passed to the plugin `Builder` (all flags if none were set).
 * @since 2.0.0
 */
async function restoreState(label, flags) {
    await core.invoke('plugin:window-state|restore_state', { label, flags });
}
/**
 *  Restore the state for the current window from disk.
 *
 * @example
 * ```typescript
 * import { restoreStateCurrent, StateFlags } from '@tauri-apps/plugin-window-state';
 *
 * await restoreStateCurrent(StateFlags.ALL);
 * ```
 *
 * @param flags Which parts of the state to restore. Defaults to the flags passed to the plugin `Builder` (all flags if none were set).
 * @since 2.0.0
 */
async function restoreStateCurrent(flags) {
    await restoreState(window.getCurrentWindow().label, flags);
}
/**
 *  Get the name of the file used to store window state.
 *
 * @example
 * ```typescript
 * import { filename } from '@tauri-apps/plugin-window-state';
 *
 * const name = await filename();
 * ```
 *
 * @returns A promise resolving to the name of the file used to store window state.
 * @since 2.0.0
 */
async function filename() {
    return await core.invoke('plugin:window-state|filename');
}

exports.filename = filename;
exports.restoreState = restoreState;
exports.restoreStateCurrent = restoreStateCurrent;
exports.saveWindowState = saveWindowState;
