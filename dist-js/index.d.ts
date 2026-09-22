import { type WindowLabel } from '@tauri-apps/api/window';
/**
 * Flags controlling which parts of a window's state are saved and restored.
 * Combine multiple flags with the bitwise OR operator (`|`).
 */
export declare enum StateFlags {
    /** Save and restore the window size. */
    SIZE = 1,
    /** Save and restore the window position. */
    POSITION = 2,
    /** Save and restore whether the window is maximized. */
    MAXIMIZED = 4,
    /** Save and restore whether the window is visible. */
    VISIBLE = 8,
    /** Save and restore whether the window has decorations. */
    DECORATIONS = 16,
    /** Save and restore whether the window is fullscreen. */
    FULLSCREEN = 32,
    /** Save and restore every flag above. */
    ALL = 63
}
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
declare function saveWindowState(flags?: StateFlags): Promise<void>;
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
declare function restoreState(label: WindowLabel, flags?: StateFlags): Promise<void>;
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
declare function restoreStateCurrent(flags?: StateFlags): Promise<void>;
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
declare function filename(): Promise<string>;
export { restoreState, restoreStateCurrent, saveWindowState, filename };
