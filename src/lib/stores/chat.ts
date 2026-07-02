import { writable, derived } from 'svelte/store';

export interface Message {
  id: string;
  sessionId: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  createdAt: string;
  tokensUsed?: number;
  isDeleted?: boolean;
  editedAt?: string;
  provider?: string;
  model?: string;
}

export interface Session {
  id: string;
  title: string;
  provider: string;
  model: string;
  systemPrompt?: string;
  createdAt: string;
  updatedAt: string;
  isArchived?: boolean;
  isPinned?: boolean;
  tokenCount?: number;
}

export type StreamingStatus = 'idle' | 'streaming' | 'interrupted' | 'complete';

interface ChatState {
  sessions: Session[];
  activeSessionId: string | null;
  messages: Message[];
  streamingStatus: StreamingStatus;
  streamingTokens: number;
  selectedSessionIds: Set<string>;
  selectMode: boolean;
}

const initialState: ChatState = {
  sessions: [],
  activeSessionId: null,
  messages: [],
  streamingStatus: 'idle',
  streamingTokens: 0,
  selectedSessionIds: new Set(),
  selectMode: false,
};

function createChatStore() {
  const { subscribe, update } = writable<ChatState>(initialState);

  return {
    subscribe,

    setSessions(sessions: Session[]) {
      update((s) => ({ ...s, sessions }));
    },

    addSession(session: Session) {
      update((s) => ({ ...s, sessions: [...s.sessions, session] }));
    },

    removeSession(sessionId: string) {
      update((s) => ({
        ...s,
        sessions: s.sessions.filter((ses) => ses.id !== sessionId),
        activeSessionId: s.activeSessionId === sessionId ? null : s.activeSessionId,
        selectedSessionIds: new Set(
          [...s.selectedSessionIds].filter((id) => id !== sessionId),
        ),
      }));
    },

    updateSession(sessionId: string, partial: Partial<Session>) {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          ses.id === sessionId ? { ...ses, ...partial } : ses,
        ),
      }));
    },

    setActiveSession(sessionId: string) {
      update((s) => ({ ...s, activeSessionId: sessionId }));
    },

    pinSession(sessionId: string) {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          ses.id === sessionId
            ? { ...ses, isPinned: true }
            : ses,
        ),
      }));
    },

    unpinSession(sessionId: string) {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          ses.id === sessionId
            ? { ...ses, isPinned: false }
            : ses,
        ),
      }));
    },

    archiveSession(sessionId: string) {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          ses.id === sessionId ? { ...ses, isArchived: true } : ses,
        ),
        activeSessionId: s.activeSessionId === sessionId ? null : s.activeSessionId,
        selectedSessionIds: new Set(
          [...s.selectedSessionIds].filter((id) => id !== sessionId),
        ),
      }));
    },

    unarchiveSession(sessionId: string) {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          ses.id === sessionId ? { ...ses, isArchived: false } : ses,
        ),
      }));
    },

    bulkArchive() {
      update((s) => ({
        ...s,
        sessions: s.sessions.map((ses) =>
          s.selectedSessionIds.has(ses.id) ? { ...ses, isArchived: true } : ses,
        ),
        activeSessionId:
          s.activeSessionId && s.selectedSessionIds.has(s.activeSessionId)
            ? null
            : s.activeSessionId,
        selectedSessionIds: new Set(),
        selectMode: false,
      }));
    },

    bulkDelete() {
      update((s) => ({
        ...s,
        sessions: s.sessions.filter((ses) => !s.selectedSessionIds.has(ses.id)),
        activeSessionId:
          s.activeSessionId && s.selectedSessionIds.has(s.activeSessionId)
            ? null
            : s.activeSessionId,
        selectedSessionIds: new Set(),
        selectMode: false,
      }));
    },

    toggleSelectMode() {
      update((s) => ({
        ...s,
        selectMode: !s.selectMode,
        selectedSessionIds: new Set(),
      }));
    },

    toggleSessionSelected(sessionId: string) {
      update((s) => {
        const next = new Set(s.selectedSessionIds);
        if (next.has(sessionId)) {
          next.delete(sessionId);
        } else {
          next.add(sessionId);
        }
        return { ...s, selectedSessionIds: next };
      });
    },

    selectAllSessions() {
      update((s) => ({
        ...s,
        selectedSessionIds: new Set(s.sessions.map((ses) => ses.id)),
      }));
    },

    deselectAllSessions() {
      update((s) => ({ ...s, selectedSessionIds: new Set() }));
    },

    setMessages(messages: Message[]) {
      update((s) => ({ ...s, messages }));
    },

    addMessage(message: Message) {
      update((s) => ({ ...s, messages: [...s.messages, message] }));
    },

    updateMessageContent(messageId: string, content: string) {
      update((s) => ({
        ...s,
        messages: s.messages.map((m) => (m.id === messageId ? { ...m, content } : m)),
      }));
    },

    appendToMessage(messageId: string, chunk: string) {
      update((s) => ({
        ...s,
        messages: s.messages.map((m) =>
          m.id === messageId ? { ...m, content: m.content + chunk } : m,
        ),
      }));
    },

    markMessageDeleted(messageId: string) {
      update((s) => ({
        ...s,
        messages: s.messages.map((m) =>
          m.id === messageId ? { ...m, isDeleted: true } : m,
        ),
      }));
    },

    markMessageEdited(messageId: string, newContent: string) {
      update((s) => ({
        ...s,
        messages: s.messages.map((m) =>
          m.id === messageId
            ? { ...m, content: newContent, editedAt: new Date().toISOString() }
            : m,
        ),
      }));
    },

    setStreamingStatus(status: StreamingStatus) {
      update((s) => ({ ...s, streamingStatus: status }));
    },

    setStreamingTokens(tokens: number) {
      update((s) => ({ ...s, streamingTokens: tokens }));
    },

    clearMessages() {
      update((s) => ({ ...s, messages: [] }));
    },

    reset() {
      update(() => initialState);
    },
  };
}

export const chatStore = createChatStore();

