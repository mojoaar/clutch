import { describe, it, expect, vi, afterEach } from "vitest";
import { timeAgo, isToday, isYesterday } from "$lib/utils/time-utils";

describe("timeAgo", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it('returns "just now" for less than 60 seconds ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-30T12:00:00Z"));
    expect(timeAgo("2026-06-30T11:59:30Z")).toBe("just now");
  });

  it("returns minutes for less than an hour", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-30T12:00:00Z"));
    expect(timeAgo("2026-06-30T11:50:00Z")).toBe("10m ago");
  });

  it("returns hours for less than a day", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-30T12:00:00Z"));
    expect(timeAgo("2026-06-30T09:00:00Z")).toBe("3h ago");
  });

  it("returns days for less than a week", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-30T12:00:00Z"));
    expect(timeAgo("2026-06-28T12:00:00Z")).toBe("2d ago");
  });

  it("returns formatted date for older dates", () => {
    const oldDate = "2024-01-15T00:00:00Z";
    const result = timeAgo(oldDate);
    expect(result).not.toBe("just now");
    expect(result).not.toContain("ago");
  });

  it("accepts Date objects", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-30T12:00:00Z"));
    expect(timeAgo(new Date("2026-06-30T11:59:50Z"))).toBe("just now");
  });
});

describe("isToday", () => {
  it("returns true for today", () => {
    const today = new Date();
    expect(isToday(today)).toBe(true);
  });

  it("returns true for today as string", () => {
    const today = new Date().toISOString();
    expect(isToday(today)).toBe(true);
  });

  it("returns false for yesterday", () => {
    const yesterday = new Date();
    yesterday.setDate(yesterday.getDate() - 1);
    expect(isToday(yesterday)).toBe(false);
  });
});

describe("isYesterday", () => {
  it("returns true for yesterday", () => {
    const yesterday = new Date();
    yesterday.setDate(yesterday.getDate() - 1);
    expect(isYesterday(yesterday)).toBe(true);
  });

  it("returns false for today", () => {
    expect(isYesterday(new Date())).toBe(false);
  });

  it("returns false for two days ago", () => {
    const twoDaysAgo = new Date();
    twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
    expect(isYesterday(twoDaysAgo)).toBe(false);
  });
});
