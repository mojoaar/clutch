import { invoke } from "@tauri-apps/api/core";

export interface StartupConfig {
  startOnBoot: boolean;
  startMinimized: boolean;
  closeToTray: boolean;
}

export async function getStartupConfig(): Promise<StartupConfig> {
  const startOnBoot = await invoke<string>("get_setting", {
    key: "start_on_boot",
  }).catch(() => "false");

  const startMinimized = await invoke<string>("get_setting", {
    key: "start_minimized",
  }).catch(() => "false");

  const closeToTray = await invoke<string>("get_setting", {
    key: "close_to_tray",
  }).catch(() => "true");

  return {
    startOnBoot: startOnBoot === "true",
    startMinimized: startMinimized === "true",
    closeToTray: closeToTray === "true",
  };
}

export async function setStartupConfig(config: StartupConfig): Promise<void> {
  await invoke("set_setting", {
    key: "start_on_boot",
    value: String(config.startOnBoot),
  });
  await invoke("set_setting", {
    key: "start_minimized",
    value: String(config.startMinimized),
  });
  await invoke("set_setting", {
    key: "close_to_tray",
    value: String(config.closeToTray),
  });
}
