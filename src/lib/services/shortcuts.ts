import { invoke } from "@tauri-apps/api/core";

export interface ShortcutConfig {
  popup: string;
  main: string;
}

export async function getShortcuts(): Promise<ShortcutConfig> {
  const popup = await invoke<string>("get_setting", {
    key: "shortcut_popup",
  }).catch(() => "CmdOrCtrl+Shift+P");

  const main = await invoke<string>("get_setting", {
    key: "shortcut_main",
  }).catch(() => "CmdOrCtrl+Shift+M");

  return { popup, main };
}

export async function setShortcuts(config: ShortcutConfig): Promise<void> {
  await invoke("set_setting", { key: "shortcut_popup", value: config.popup });
  await invoke("set_setting", { key: "shortcut_main", value: config.main });
}
