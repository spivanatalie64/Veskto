import { invoke } from "@tauri-apps/api/core";
import { listen, emit } from "@tauri-apps/api/event";

export interface VesktopNativeApp {
    relaunch(): Promise<void>;
    getVersion(): Promise<string>;
    setBadgeCount(count: number): Promise<void>;
    supportsWindowsTransparency(): Promise<boolean>;
    getEnableHardwareAcceleration(): Promise<boolean>;
    isOutdated(): Promise<boolean>;
    openUpdater(): Promise<void>;
    getRendererCss(): Promise<string>;
    onRendererCssUpdate(callback: (css: string) => void): void;
}

export interface VesktopNativeSettings {
    get(key: string): Promise<any>;
    set(key: string, value: any): Promise<void>;
}

export interface VesktopNativeWin {
    focus(): Promise<void>;
    close(): Promise<void>;
    minimize(): Promise<void>;
    maximize(): Promise<void>;
    flashFrame(): Promise<void>;
    setDevtoolsCallbacks(
        onOpen: (() => void) | null,
        onClose: (() => void) | null
    ): void;
}

export interface VesktopNativeAutostart {
    isEnabled(): Promise<boolean>;
    enable(): Promise<void>;
    disable(): Promise<void>;
}

export interface VesktopNativeCommands {
    onCommand(
        callback: (data: { message: string; nonce: string; data: any }) => void
    ): void;
    respond(nonce: string, success: boolean, result: any): void;
}

export interface VesktopNativeClipboard {
    copyImage(dataUrl: string): Promise<void>;
}

export interface VesktopNativeTray {
    setIcon(path: string): Promise<void>;
    setTooltip(tooltip: string): Promise<void>;
}

export interface VesktopNativeVencord {
    getScript(): Promise<string>;
}

export interface VesktopNativeSteamDeck {
    isGameMode(): Promise<boolean>;
    applyFixes(): Promise<void>;
}

export interface VesktopNative {
    app: VesktopNativeApp;
    settings: VesktopNativeSettings;
    win: VesktopNativeWin;
    autostart: VesktopNativeAutostart;
    commands: VesktopNativeCommands;
    clipboard: VesktopNativeClipboard;
    tray: VesktopNativeTray;
    vencord: VesktopNativeVencord;
    steamDeck: VesktopNativeSteamDeck;
}

const commandHandlers = new Map<
    string,
    (data: any) => Promise<any>
>();

export const VesktopNative: VesktopNative = {
    app: {
        async relaunch() {
            await invoke("relaunch");
        },
        async getVersion() {
            return invoke("get_version");
        },
        async setBadgeCount(_count: number) {
            // TODO: implement via Tauri notification plugin
        },
        async supportsWindowsTransparency() {
            return false;
        },
        async getEnableHardwareAcceleration() {
            return true;
        },
        async isOutdated() {
            return false;
        },
        async openUpdater() {},
        async getRendererCss() {
            return "";
        },
        onRendererCssUpdate(_callback: (css: string) => void) {},
    },

    settings: {
        async get(key: string) {
            return invoke("get_setting", { key });
        },
        async set(key: string, value: any) {
            return invoke("set_setting", { key, value });
        },
    },

    win: {
        async focus() {
            await invoke("show_window");
        },
        async close() {
            await invoke("close_window");
        },
        async minimize() {
            await invoke("minimize_window");
        },
        async maximize() {
            await invoke("maximize_window");
        },
        async flashFrame() {
            await invoke("flash_window");
        },
        setDevtoolsCallbacks(_onOpen, _onClose) {},
    },

    autostart: {
        async isEnabled() {
            return invoke("is_autostart_enabled");
        },
        async enable() {
            return invoke("enable_autostart");
        },
        async disable() {
            return invoke("disable_autostart");
        },
    },

    commands: {
        onCommand(callback) {
            listen("vesktop-command", (event: any) => {
                callback(event.payload);
            });
        },
        respond(nonce: string, success: boolean, result: any) {
            emit("vesktop-command-response", { nonce, success, result });
        },
    },

    clipboard: {
        async copyImage(_dataUrl: string) {
            // TODO: implement via clipboard manager plugin
        },
    },

    tray: {
        async setIcon(path: string) {
            return invoke("set_tray_icon", { iconPath: path });
        },
        async setTooltip(tooltip: string) {
            return invoke("set_tray_tooltip", { tooltip });
        },
    },

    vencord: {
        async getScript() {
            return invoke("get_vencord_script");
        },
    },

    steamDeck: {
        async isGameMode() {
            return invoke("is_steam_deck_game_mode");
        },
        async applyFixes() {
            return invoke("apply_steam_deck_fixes");
        },
    },
};

declare global {
    interface Window {
        VesktopNative: VesktopNative;
    }
}

if (typeof window !== "undefined") {
    window.VesktopNative = VesktopNative;
}
