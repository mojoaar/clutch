import { invoke } from "@tauri-apps/api/core";

export async function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke("set_setting", { key, value });
}

export async function getAllSettings(): Promise<Record<string, string>> {
  return invoke<Record<string, string>>("get_all_settings");
}

export interface TestConnectionResult {
  ok: boolean;
  message: string;
}

export async function testConnection(
  provider: string,
  apiKey: string,
): Promise<TestConnectionResult> {
  return invoke<TestConnectionResult>("test_connection", { provider, apiKey });
}
