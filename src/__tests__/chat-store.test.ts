import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { chatStore, type Session, type Message } from "$lib/stores/chat";

function makeSession(overrides: Partial<Session> = {}): Session {
  return {
    id: "s1",
    title: "Test Session",
    provider: "deepseek",
    model: "deepseek-chat",
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-01T00:00:00Z",
    ...overrides,
  };
}

function makeMessage(overrides: Partial<Message> = {}): Message {
  return {
    id: "m1",
    sessionId: "s1",
    role: "user",
    content: "Hello",
    createdAt: "2026-01-01T00:00:00Z",
    ...overrides,
  };
}

describe("chatStore", () => {
  beforeEach(() => {
    chatStore.reset();
  });

  describe("session management", () => {
    it("addSession adds to sessions array", () => {
      const session = makeSession();
      chatStore.addSession(session);
      expect(get(chatStore).sessions).toEqual([session]);
    });

    it("removeSession removes and clears activeSessionId if it was the removed session", () => {
      const session = makeSession({ id: "s1" });
      chatStore.addSession(session);
      chatStore.setActiveSession("s1");
      chatStore.removeSession("s1");
      const state = get(chatStore);
      expect(state.sessions).toEqual([]);
      expect(state.activeSessionId).toBeNull();
    });

    it("removeSession removes but keeps activeSessionId if a different session is active", () => {
      const s1 = makeSession({ id: "s1" });
      const s2 = makeSession({ id: "s2" });
      chatStore.addSession(s1);
      chatStore.addSession(s2);
      chatStore.setActiveSession("s2");
      chatStore.removeSession("s1");
      const state = get(chatStore);
      expect(state.sessions).toHaveLength(1);
      expect(state.sessions[0].id).toBe("s2");
      expect(state.activeSessionId).toBe("s2");
    });

    it("setActiveSession updates activeSessionId", () => {
      chatStore.setActiveSession("s1");
      expect(get(chatStore).activeSessionId).toBe("s1");
    });

    it("pinSession sets isPinned=true", () => {
      const session = makeSession({ id: "s1", isPinned: false });
      chatStore.addSession(session);
      chatStore.pinSession("s1");
      expect(get(chatStore).sessions[0].isPinned).toBe(true);
    });

    it("unpinSession sets isPinned=false", () => {
      const session = makeSession({ id: "s1", isPinned: true });
      chatStore.addSession(session);
      chatStore.unpinSession("s1");
      expect(get(chatStore).sessions[0].isPinned).toBe(false);
    });

    it("archiveSession sets isArchived=true", () => {
      const session = makeSession({ id: "s1", isArchived: false });
      chatStore.addSession(session);
      chatStore.archiveSession("s1");
      expect(get(chatStore).sessions[0].isArchived).toBe(true);
    });

    it("archiveSession clears activeSessionId if it was the archived session", () => {
      const session = makeSession({ id: "s1" });
      chatStore.addSession(session);
      chatStore.setActiveSession("s1");
      chatStore.archiveSession("s1");
      expect(get(chatStore).activeSessionId).toBeNull();
    });

    it("unarchiveSession sets isArchived=false", () => {
      const session = makeSession({ id: "s1", isArchived: true });
      chatStore.addSession(session);
      chatStore.unarchiveSession("s1");
      expect(get(chatStore).sessions[0].isArchived).toBe(false);
    });

    it("updateSession merges partial updates", () => {
      const session = makeSession({ id: "s1", title: "Old Title" });
      chatStore.addSession(session);
      chatStore.updateSession("s1", { title: "New Title" });
      const state = get(chatStore);
      expect(state.sessions[0].title).toBe("New Title");
      expect(state.sessions[0].provider).toBe("deepseek");
    });

    it("setSessions replaces entire sessions array", () => {
      chatStore.addSession(makeSession({ id: "s1" }));
      const newSessions = [
        makeSession({ id: "s2" }),
        makeSession({ id: "s3" }),
      ];
      chatStore.setSessions(newSessions);
      expect(get(chatStore).sessions).toEqual(newSessions);
    });
  });

  describe("bulk operations", () => {
    it("bulkArchive archives selected sessions and clears selection", () => {
      const s1 = makeSession({ id: "s1", isArchived: false });
      const s2 = makeSession({ id: "s2", isArchived: false });
      const s3 = makeSession({ id: "s3", isArchived: false });
      chatStore.addSession(s1);
      chatStore.addSession(s2);
      chatStore.addSession(s3);
      chatStore.toggleSessionSelected("s1");
      chatStore.toggleSessionSelected("s3");
      chatStore.bulkArchive();
      const state = get(chatStore);
      expect(state.sessions.find((s) => s.id === "s1")?.isArchived).toBe(true);
      expect(state.sessions.find((s) => s.id === "s2")?.isArchived).toBe(false);
      expect(state.sessions.find((s) => s.id === "s3")?.isArchived).toBe(true);
      expect(state.selectedSessionIds.size).toBe(0);
      expect(state.selectMode).toBe(false);
    });

    it("bulkArchive clears activeSessionId if active session was archived", () => {
      const s1 = makeSession({ id: "s1" });
      chatStore.addSession(s1);
      chatStore.setActiveSession("s1");
      chatStore.toggleSessionSelected("s1");
      chatStore.bulkArchive();
      expect(get(chatStore).activeSessionId).toBeNull();
    });

    it("bulkArchive keeps activeSessionId if active session was not archived", () => {
      const s1 = makeSession({ id: "s1" });
      const s2 = makeSession({ id: "s2" });
      chatStore.addSession(s1);
      chatStore.addSession(s2);
      chatStore.setActiveSession("s2");
      chatStore.toggleSessionSelected("s1");
      chatStore.bulkArchive();
      expect(get(chatStore).activeSessionId).toBe("s2");
    });

    it("bulkDelete removes selected sessions from array", () => {
      const s1 = makeSession({ id: "s1" });
      const s2 = makeSession({ id: "s2" });
      chatStore.addSession(s1);
      chatStore.addSession(s2);
      chatStore.toggleSessionSelected("s2");
      chatStore.bulkDelete();
      const state = get(chatStore);
      expect(state.sessions).toHaveLength(1);
      expect(state.sessions[0].id).toBe("s1");
      expect(state.selectedSessionIds.size).toBe(0);
      expect(state.selectMode).toBe(false);
    });

    it("bulkDelete clears activeSessionId if active session was deleted", () => {
      const s1 = makeSession({ id: "s1" });
      chatStore.addSession(s1);
      chatStore.setActiveSession("s1");
      chatStore.toggleSessionSelected("s1");
      chatStore.bulkDelete();
      expect(get(chatStore).activeSessionId).toBeNull();
    });
  });

  describe("selection management", () => {
    it("toggleSelectMode toggles selectMode and clears selection", () => {
      chatStore.toggleSessionSelected("s1");
      expect(get(chatStore).selectMode).toBe(false);
      chatStore.toggleSelectMode();
      expect(get(chatStore).selectMode).toBe(true);
      expect(get(chatStore).selectedSessionIds.size).toBe(0);
      chatStore.toggleSelectMode();
      expect(get(chatStore).selectMode).toBe(false);
    });

    it("toggleSessionSelected adds id to selectedSessionIds", () => {
      chatStore.toggleSessionSelected("s1");
      expect(get(chatStore).selectedSessionIds.has("s1")).toBe(true);
    });

    it("toggleSessionSelected removes id if already selected", () => {
      chatStore.toggleSessionSelected("s1");
      chatStore.toggleSessionSelected("s1");
      expect(get(chatStore).selectedSessionIds.has("s1")).toBe(false);
    });

    it("selectAllSessions selects all sessions", () => {
      chatStore.addSession(makeSession({ id: "s1" }));
      chatStore.addSession(makeSession({ id: "s2" }));
      chatStore.addSession(makeSession({ id: "s3" }));
      chatStore.selectAllSessions();
      const selected = get(chatStore).selectedSessionIds;
      expect(selected.size).toBe(3);
      expect(selected.has("s1")).toBe(true);
      expect(selected.has("s2")).toBe(true);
      expect(selected.has("s3")).toBe(true);
    });

    it("deselectAllSessions clears selection", () => {
      chatStore.toggleSessionSelected("s1");
      chatStore.toggleSessionSelected("s2");
      chatStore.deselectAllSessions();
      expect(get(chatStore).selectedSessionIds.size).toBe(0);
    });
  });

  describe("message management", () => {
    it("setMessages replaces messages array", () => {
      const messages = [makeMessage(), makeMessage({ id: "m2" })];
      chatStore.setMessages(messages);
      expect(get(chatStore).messages).toEqual(messages);
    });

    it("addMessage appends to messages", () => {
      const m1 = makeMessage({ id: "m1" });
      const m2 = makeMessage({ id: "m2" });
      chatStore.addMessage(m1);
      chatStore.addMessage(m2);
      expect(get(chatStore).messages).toEqual([m1, m2]);
    });

    it("appendToMessage concatenates content to existing message", () => {
      const msg = makeMessage({ id: "m1", content: "Hello" });
      chatStore.addMessage(msg);
      chatStore.appendToMessage("m1", " World");
      expect(get(chatStore).messages[0].content).toBe("Hello World");
      chatStore.appendToMessage("m1", "!");
      expect(get(chatStore).messages[0].content).toBe("Hello World!");
    });

    it("updateMessageContent directly replaces content", () => {
      const msg = makeMessage({ id: "m1", content: "Original" });
      chatStore.addMessage(msg);
      chatStore.updateMessageContent("m1", "Replaced");
      expect(get(chatStore).messages[0].content).toBe("Replaced");
    });

    it("markMessageDeleted sets isDeleted=true", () => {
      const msg = makeMessage({ id: "m1", isDeleted: false });
      chatStore.addMessage(msg);
      chatStore.markMessageDeleted("m1");
      expect(get(chatStore).messages[0].isDeleted).toBe(true);
    });

    it("clearMessages empties messages array", () => {
      chatStore.addMessage(makeMessage({ id: "m1" }));
      chatStore.addMessage(makeMessage({ id: "m2" }));
      chatStore.clearMessages();
      expect(get(chatStore).messages).toEqual([]);
    });
  });

  describe("streaming", () => {
    it("setStreamingStatus updates streamingStatus", () => {
      chatStore.setStreamingStatus("streaming");
      expect(get(chatStore).streamingStatus).toBe("streaming");
      chatStore.setStreamingStatus("complete");
      expect(get(chatStore).streamingStatus).toBe("complete");
    });

    it("setStreamingTokens updates token count", () => {
      chatStore.setStreamingTokens(42);
      expect(get(chatStore).streamingTokens).toBe(42);
    });
  });

  describe("reset", () => {
    it("reset returns to initialState shape", () => {
      chatStore.addSession(makeSession({ id: "s1" }));
      chatStore.setActiveSession("s1");
      chatStore.addMessage(makeMessage({ id: "m1" }));
      chatStore.setStreamingStatus("streaming");
      chatStore.setStreamingTokens(100);
      chatStore.toggleSelectMode();

      chatStore.reset();
      const state = get(chatStore);
      expect(state.sessions).toEqual([]);
      expect(state.activeSessionId).toBeNull();
      expect(state.messages).toEqual([]);
      expect(state.streamingStatus).toBe("idle");
      expect(state.streamingTokens).toBe(0);
      expect(state.selectedSessionIds).toBeInstanceOf(Set);
      expect(state.selectedSessionIds.size).toBe(0);
      expect(state.selectMode).toBe(false);
    });
  });

  describe("markMessageEdited", () => {
    it("updates content and sets editedAt", () => {
      const before = Date.now();
      const msg = makeMessage({ id: "m1", content: "Old" });
      chatStore.addMessage(msg);
      chatStore.markMessageEdited("m1", "New");
      const updated = get(chatStore).messages[0];
      expect(updated.content).toBe("New");
      expect(updated.editedAt).toBeDefined();
      expect(new Date(updated.editedAt!).getTime()).toBeGreaterThanOrEqual(
        before,
      );
    });
  });

  describe("removeSession also clears selectedSessionIds", () => {
    it("removes the session from selection set", () => {
      chatStore.addSession(makeSession({ id: "s1" }));
      chatStore.addSession(makeSession({ id: "s2" }));
      chatStore.toggleSessionSelected("s1");
      chatStore.toggleSessionSelected("s2");
      chatStore.removeSession("s1");
      const selected = get(chatStore).selectedSessionIds;
      expect(selected.has("s1")).toBe(false);
      expect(selected.has("s2")).toBe(true);
    });
  });

  describe("archiveSession also clears selectedSessionIds", () => {
    it("removes archived session from selection", () => {
      chatStore.addSession(makeSession({ id: "s1" }));
      chatStore.toggleSessionSelected("s1");
      chatStore.archiveSession("s1");
      expect(get(chatStore).selectedSessionIds.has("s1")).toBe(false);
    });
  });
});
