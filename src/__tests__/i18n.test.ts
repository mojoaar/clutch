import { describe, it, expect } from "vitest";
import en from "$lib/i18n/en/index";
import da from "$lib/i18n/da/index";
import de from "$lib/i18n/de/index";
import pl from "$lib/i18n/pl/index";
import fr from "$lib/i18n/fr/index";

const LOCALES = { en, da, de, pl, fr };

function getLeafPaths(obj: any, prefix = ""): string[] {
  const keys: string[] = [];
  for (const [k, v] of Object.entries(obj)) {
    const path = prefix ? `${prefix}.${k}` : k;
    if (v !== null && typeof v === "object") {
      keys.push(...getLeafPaths(v, path));
    } else {
      keys.push(path);
    }
  }
  return keys;
}

function getTopLevelKeys(obj: any): string[] {
  return Object.keys(obj).sort();
}

const REQUIRED_NAMESPACES = [
  "chat",
  "errors",
  "providers",
  "startup",
  "shortcuts",
  "workspaces",
  "skills",
  "theme",
  "exportSettings",
  "settingsTabs",
  "settingsSections",
  "profile",
  "permissions",
  "shortcutActions",
  "welcome",
  "networkStatus",
  "modelCache",
  "dropZone",
  "contextBar",
  "exportFormats",
  "errorBoundary",
  "toast",
  "aria",
  "about",
  "updates",
];

const CRITICAL_KEYS = [
  "aria.chatMessages",
  "settingsTabs.updates",
  "settingsTabs.skills",
  "networkStatus.messagesQueued",
  "networkStatus.retryQueue",
  "networkStatus.clearQueue",
  "networkStatus.clearQueueConfirm",
  "updates.autoCheck",
  "updates.checkNow",
  "skills.instructions",
  "skills.installedSuccess",
  "skills.uninstallConfirm",
];

describe("i18n locale coverage", () => {
  // --- EN-specific checks ---

  it("en locale has all required namespaces", () => {
    for (const ns of REQUIRED_NAMESPACES) {
      expect((en as any)[ns], `Missing namespace: ${ns}`).toBeDefined();
    }
  });

  it("en locale has all critical keys", () => {
    const allKeys = getLeafPaths(en);
    for (const key of CRITICAL_KEYS) {
      expect(allKeys.includes(key), `Missing critical key: ${key}`).toBe(true);
    }
  });

  it("en locale has no empty string values", () => {
    const allKeys = getLeafPaths(en);
    for (const key of allKeys) {
      const value = key.split(".").reduce((o: any, k) => o?.[k], en);
      expect(value, `Empty value for key: ${key}`).toBeTruthy();
    }
  });

  it("all en leaf values are non-empty strings", () => {
    function checkStrings(obj: any, path: string): void {
      for (const [k, v] of Object.entries(obj)) {
        const fullPath = `${path}.${k}`;
        if (typeof v === "string") {
          expect(v.length, `Empty string at ${fullPath}`).toBeGreaterThan(0);
        } else if (v !== null && typeof v === "object") {
          checkStrings(v, fullPath);
        }
      }
    }
    checkStrings(en, "en");
  });

  // --- Cross-locale namespace parity ---

  it("all locales have the same top-level namespaces as en", () => {
    const enNamespaces = getTopLevelKeys(en);
    for (const [code, locale] of Object.entries(LOCALES)) {
      const ns = getTopLevelKeys(locale as any);
      const missing = enNamespaces.filter((k) => !ns.includes(k));
      expect(missing, `${code}: missing namespaces`).toEqual([]);
      const extra = ns.filter((k) => !enNamespaces.includes(k));
      if (extra.length > 0) {
        console.warn(
          `${code}: has extra namespaces not in en: ${extra.join(", ")}`,
        );
      }
    }
  });

  // --- Cross-locale deep key parity ---

  it("all locales have the same leaf keys as en", () => {
    const enLeaves = getLeafPaths(en);
    for (const [code, locale] of Object.entries(LOCALES)) {
      if (code === "en") continue;
      const leaves = getLeafPaths(locale as any);
      const missing = enLeaves.filter((k) => !leaves.includes(k));
      expect(missing, `${code}: missing keys`).toEqual([]);
      const extra = leaves.filter((k) => !enLeaves.includes(k));
      if (extra.length > 0) {
        console.warn(`${code}: has extra keys not in en: ${extra.join(", ")}`);
      }
    }
  });

  // --- All locales have no empty strings ---

  it("no locale has empty string values", () => {
    for (const [code, locale] of Object.entries(LOCALES)) {
      const leaves = getLeafPaths(locale as any);
      for (const key of leaves) {
        const value = key.split(".").reduce((o: any, k) => o?.[k], locale);
        expect(value, `${code}: empty value for ${key}`).toBeTruthy();
      }
    }
  });
});
