'use strict';

var core = require('@tauri-apps/api/core');
var window = require('@tauri-apps/api/window');

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
exports.StateFlags = void 0;
(function (StateFlags) {
    StateFlags[StateFlags["SIZE"] = 1] = "SIZE";
    StateFlags[StateFlags["POSITION"] = 2] = "POSITION";
    StateFlags[StateFlags["MAXIMIZED"] = 4] = "MAXIMIZED";
    StateFlags[StateFlags["VISIBLE"] = 8] = "VISIBLE";
    StateFlags[StateFlags["DECORATIONS"] = 16] = "DECORATIONS";
    StateFlags[StateFlags["FULLSCREEN"] = 32] = "FULLSCREEN";
    StateFlags[StateFlags["ALL"] = 63] = "ALL";
})(exports.StateFlags || (exports.StateFlags = {}));
/**
 *  Save the state of all open windows to disk.
 */
async function saveWindowState(flags) {
    return core.invoke("plugin:window-state|save_window_state", { flags });
}
/**
 *  Restore the state for the specified window from disk.
 */
async function restoreState(label, flags) {
    return core.invoke("plugin:window-state|restore_state", { label, flags });
}
/**
 *  Restore the state for the current window from disk.
 */
async function restoreStateCurrent(flags) {
    return restoreState(window.getCurrent().label, flags);
}

exports.restoreState = restoreState;
exports.restoreStateCurrent = restoreStateCurrent;
exports.saveWindowState = saveWindowState;
