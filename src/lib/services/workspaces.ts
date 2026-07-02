import { invoke } from '@tauri-apps/api/core';

export interface WorkspaceEntry {
  path: string;
  name: string;
  project_type: string | null;
}

export interface DirEntry {
  name: string;
  is_dir: boolean;
  size: number;
}

export async function listWorkspaces(): Promise<WorkspaceEntry[]> {
  return invoke<WorkspaceEntry[]>('list_workspaces');
}

export async function addWorkspace(path: string): Promise<WorkspaceEntry[]> {
  return invoke<WorkspaceEntry[]>('add_workspace', { path });
}

export async function removeWorkspace(path: string): Promise<WorkspaceEntry[]> {
  return invoke<WorkspaceEntry[]>('remove_workspace', { path });
}

export async function setActiveWorkspace(path: string | null): Promise<void> {
  return invoke('set_active_workspace', { path });
}

export async function getActiveWorkspace(): Promise<string | null> {
  return invoke<string | null>('get_active_workspace');
}

export async function detectWorkspaces(): Promise<WorkspaceEntry[]> {
  return invoke<WorkspaceEntry[]>('detect_workspaces');
}

export async function readWorkspaceFile(
  workspacePath: string,
  filePath: string,
): Promise<string> {
  return invoke<string>('read_workspace_file', { workspacePath, filePath });
}

export async function writeWorkspaceFile(
  workspacePath: string,
  filePath: string,
  content: string,
): Promise<void> {
  return invoke('write_workspace_file', { workspacePath, filePath, content });
}

export async function listWorkspaceDir(
  workspacePath: string,
  dirPath: string,
): Promise<DirEntry[]> {
  return invoke<DirEntry[]>('list_workspace_dir', { workspacePath, dirPath });
}

export async function createWorkspaceDir(
  workspacePath: string,
  dirPath: string,
): Promise<void> {
  return invoke('create_workspace_dir', { workspacePath, dirPath });
}

export async function deleteWorkspaceDir(
  workspacePath: string,
  dirPath: string,
): Promise<void> {
  return invoke('delete_workspace_dir', { workspacePath, dirPath });
}
