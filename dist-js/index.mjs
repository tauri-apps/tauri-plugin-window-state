import { invoke } from '@tauri-apps/api/tauri';

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
var StateFlags;
(function (StateFlags) {
    StateFlags[StateFlags["SIZE"] = 1] = "SIZE";
    StateFlags[StateFlags["POSITION"] = 2] = "POSITION";
    StateFlags[StateFlags["MAXIMIZED"] = 4] = "MAXIMIZED";
    StateFlags[StateFlags["VISIBLE"] = 8] = "VISIBLE";
    StateFlags[StateFlags["DECORATIONS"] = 16] = "DECORATIONS";
    StateFlags[StateFlags["FULLSCREEN"] = 32] = "FULLSCREEN";
    StateFlags[StateFlags["ALL"] = 63] = "ALL";
})(StateFlags || (StateFlags = {}));
/**
 *  Save the state of all open windows to disk.
 */
async function saveWindowState(flags) {
    invoke("plugin:window-state|save_window_state", { flags });
}
/**
 *  Restore the state for the specified window from disk.
 */
async function restoreState(label, flags) {
    invoke("plugin:window-state|restore_state", { label, flags });
}
/**
 *  Restore the state for the current window from disk.
 */
async function restoreStateCurrent(flags) {
    restoreState(window.__TAURI_METADATA__.__currentWindow.label, flags);
}

export { StateFlags, restoreState, restoreStateCurrent, saveWindowState };
//# sourceMappingURL=index.mjs.map
