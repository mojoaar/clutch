import { invoke } from "@tauri-apps/api/core";
import type { Session, Message } from "$lib/stores/chat";

export async function listSessions(
  includeArchived = false,
): Promise<Session[]> {
  const sessions = await invoke<any[]>("list_sessions", { includeArchived });
  return sessions.map((s) => ({
    id: s.id,
    title: s.title,
    provider: s.provider,
    model: s.model,
    systemPrompt: s.system_prompt,
    createdAt: s.created_at,
    updatedAt: s.updated_at,
    isArchived: s.is_archived,
    isPinned: s.is_pinned,
    tokenCount: s.token_count,
  }));
}

export async function createSession(
  id: string,
  title: string,
  provider: string,
  model: string,
  systemPrompt?: string,
): Promise<void> {
  return invoke("create_session", { id, title, provider, model, systemPrompt });
}

export async function updateSessionTitle(
  id: string,
  title: string,
): Promise<void> {
  return invoke("update_session_title", { id, title });
}

export async function updateSessionProvider(
  id: string,
  provider: string,
  model: string,
): Promise<void> {
  return invoke("update_session_provider", { id, provider, model });
}

export async function pinSession(id: string): Promise<void> {
  return invoke("pin_session", { id });
}

export async function unpinSession(id: string): Promise<void> {
  return invoke("unpin_session", { id });
}

export async function archiveSession(id: string): Promise<void> {
  return invoke("archive_session", { id });
}

export async function unarchiveSession(id: string): Promise<void> {
  return invoke("unarchive_session", { id });
}

export async function deleteSession(id: string): Promise<void> {
  return invoke("delete_session", { id });
}

export async function listMessages(sessionId: string): Promise<Message[]> {
  const msgs = await invoke<Message[]>("list_messages", { sessionId });
  return msgs.map((m) => ({
    id: m.id,
    sessionId: m.session_id ?? sessionId,
    role: m.role as "user" | "assistant" | "system",
    content: m.content,
    createdAt: m.created_at,
    tokensUsed: m.tokens_used,
    isDeleted: m.is_deleted,
    editedAt: m.edited_at,
  }));
}

export async function createMessage(
  id: string,
  sessionId: string,
  role: string,
  content: string,
  tokensUsed?: number,
): Promise<void> {
  return invoke("create_message", { id, sessionId, role, content, tokensUsed });
}

export async function updateMessage(
  id: string,
  content: string,
): Promise<void> {
  return invoke("update_message", { id, content });
}

export async function deleteMessage(id: string): Promise<void> {
  return invoke("delete_message", { id });
}

export interface UserProfile {
  displayName: string;
  avatarType: string;
  avatarData: string;
  avatarColor: string;
}

export async function getUserProfile(): Promise<UserProfile | null> {
  const raw = await invoke<{
    display_name: string;
    avatar_type: string;
    avatar_data: string;
    avatar_color: string;
  } | null>("get_user_profile");
  if (!raw) return null;
  return {
    displayName: raw.display_name,
    avatarType: raw.avatar_type,
    avatarData: raw.avatar_data,
    avatarColor: raw.avatar_color,
  };
}

export async function updateUserProfile(profile: UserProfile): Promise<void> {
  await invoke("update_user_profile", {
    displayName: profile.displayName,
    avatarType: profile.avatarType,
    avatarData: profile.avatarData,
    avatarColor: profile.avatarColor,
  });
}
