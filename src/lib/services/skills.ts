import { invoke } from "@tauri-apps/api/core";

export interface SkillDetail {
  id: string;
  name: string;
  description: string;
  instructions: string;
  source: string;
  branch: string;
  installed: boolean;
  install_path: string | null;
}

export interface SkillAction {
  action: string;
  path: string;
  content?: string;
  command?: string;
}

export async function getSkillDetail(
  id: string,
  source: string,
  branch?: string,
): Promise<SkillDetail> {
  return invoke<SkillDetail>("get_skill_detail", {
    id,
    source,
    branch: branch || "main",
  });
}

export async function installSkill(
  id: string,
  source: string,
  branch?: string,
): Promise<void> {
  return invoke("install_skill", { id, source, branch: branch || "main" });
}

export async function uninstallSkill(id: string): Promise<void> {
  return invoke("uninstall_skill", { id });
}

export async function listInstalledSkills(): Promise<SkillDetail[]> {
  return invoke<SkillDetail[]>("list_installed_skills");
}

export async function getSkillInstructions(id: string): Promise<string> {
  return invoke<string>("get_skill_instructions", { id });
}

export interface SkillUpdateInfo {
  id: string;
  name: string;
  has_update: boolean;
  current_version: string | null;
  latest_version: string;
}

export async function checkForUpdates(): Promise<SkillUpdateInfo[]> {
  return invoke<SkillUpdateInfo[]>("check_skill_updates");
}

export async function updateSkill(id: string): Promise<void> {
  return invoke("update_skill", { id });
}

export async function executeSkillAction(
  id: string,
  action: SkillAction,
  workspacePath?: string,
): Promise<string> {
  return invoke<string>("execute_skill_action", { id, action, workspacePath });
}
