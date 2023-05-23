declare global {
    interface Window {
        __TAURI_INVOKE__: <T>(cmd: string, args?: unknown) => Promise<T>;
    }
}
interface WindowDef {
    label: string;
}
declare global {
    interface Window {
        __TAURI_METADATA__: {
            __windows: WindowDef[];
            __currentWindow: WindowDef;
        };
    }
}
export declare enum StateFlags {
    SIZE = 1,
    POSITION = 2,
    MAXIMIZED = 4,
    VISIBLE = 8,
    DECORATIONS = 16,
    FULLSCREEN = 32,
    ALL = 63
}
/**
 *  Save the state of all open windows to disk.
 */
declare function saveWindowState(flags: StateFlags): Promise<void>;
/**
 *  Restore the state for the specified window from disk.
 */
declare function restoreState(label: string, flags: StateFlags): Promise<void>;
/**
 *  Restore the state for the current window from disk.
 */
declare function restoreStateCurrent(flags: StateFlags): Promise<void>;
export { restoreState, restoreStateCurrent, saveWindowState };
