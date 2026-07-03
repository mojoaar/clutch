export type CommandCategory = "chat" | "workspace" | "web" | "skills" | "app";
export type CommandType = "client" | "ai-aware";

export interface CommandDef {
  id: string;
  label: string;
  descriptionKey: string;
  category: CommandCategory;
  type: CommandType;
  args?: { name: string; placeholder: string }[];
}

export const COMMANDS: CommandDef[] = [
  {
    id: "theme",
    label: "/theme",
    descriptionKey: "switchTheme",
    category: "app",
    type: "client",
    args: [
      {
        name: "name",
        placeholder:
          "clutch|nord|dracula|cyberpunk|catppuccin|github|tokyo-night|monokai",
      },
    ],
  },
  {
    id: "model",
    label: "/model",
    descriptionKey: "switchModel",
    category: "chat",
    type: "client",
    args: [{ name: "name", placeholder: "model-name" }],
  },
  {
    id: "provider",
    label: "/provider",
    descriptionKey: "switchProvider",
    category: "chat",
    type: "client",
    args: [{ name: "id", placeholder: "deepseek|opencode_go|opencode_zen" }],
  },
  {
    id: "workspace",
    label: "/workspace",
    descriptionKey: "showWorkspace",
    category: "workspace",
    type: "client",
  },
  {
    id: "add-workspace",
    label: "/add-workspace",
    descriptionKey: "addWorkspace",
    category: "workspace",
    type: "client",
    args: [{ name: "path", placeholder: "/path/to/project" }],
  },
  {
    id: "skills",
    label: "/skills",
    descriptionKey: "listSkills",
    category: "skills",
    type: "client",
  },
  {
    id: "read",
    label: "/read",
    descriptionKey: "readFile",
    category: "workspace",
    type: "ai-aware",
    args: [{ name: "file", placeholder: "src/main.rs" }],
  },
  {
    id: "ls",
    label: "/ls",
    descriptionKey: "listDir",
    category: "workspace",
    type: "ai-aware",
    args: [{ name: "path", placeholder: "src/" }],
  },
  {
    id: "fetch",
    label: "/fetch",
    descriptionKey: "fetchUrl",
    category: "web",
    type: "ai-aware",
    args: [{ name: "url", placeholder: "https://example.com" }],
  },
  {
    id: "github",
    label: "/github",
    descriptionKey: "github",
    category: "web",
    type: "ai-aware",
    args: [{ name: "repo", placeholder: "owner/repo" }],
  },
  {
    id: "skill",
    label: "/skill",
    descriptionKey: "loadSkill",
    category: "skills",
    type: "ai-aware",
    args: [{ name: "name", placeholder: "golang" }],
  },
  {
    id: "search-skills",
    label: "/search-skills",
    descriptionKey: "searchSkills",
    category: "skills",
    type: "client",
    args: [{ name: "query", placeholder: "search terms" }],
  },
];

export function parseCommand(
  input: string,
): { command: CommandDef; args: string[]; rest: string } | null {
  const trimmed = input.trim();
  if (!trimmed.startsWith("/")) return null;

  const parts = trimmed.split(/\s+/);
  const cmdText = parts[0].toLowerCase();
  const command = COMMANDS.find((c) => c.label === cmdText);
  if (!command) return null;

  const args = parts.slice(1);
  const rest = "";

  return { command, args, rest };
}

export function filterCommands(query: string, limit?: number): CommandDef[] {
  const lower = query.toLowerCase();
  const matches = COMMANDS.filter(
    (c) =>
      c.label.toLowerCase().includes(lower) ||
      c.id.toLowerCase().includes(lower) ||
      c.descriptionKey.toLowerCase().includes(lower),
  );
  if (limit) return matches.slice(0, limit);
  return matches;
}
