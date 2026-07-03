export type CommandCategory = 'chat' | 'workspace' | 'web' | 'skills' | 'app';
export type CommandType = 'client' | 'ai-aware';

export interface CommandDef {
  id: string;
  label: string;
  description: string;
  category: CommandCategory;
  type: CommandType;
  args?: { name: string; placeholder: string }[];
}

export const COMMANDS: CommandDef[] = [
  {
    id: 'theme',
    label: '/theme',
    description: 'Switch the active theme',
    category: 'app',
    type: 'client',
    args: [{ name: 'name', placeholder: 'clutch|nord|dracula|cyberpunk|catppuccin|github|tokyo-night|monokai' }],
  },
  {
    id: 'model',
    label: '/model',
    description: 'Switch the active model',
    category: 'chat',
    type: 'client',
    args: [{ name: 'name', placeholder: 'model-name' }],
  },
  {
    id: 'provider',
    label: '/provider',
    description: 'Switch the active provider',
    category: 'chat',
    type: 'client',
    args: [{ name: 'id', placeholder: 'deepseek|opencode_go|opencode_zen' }],
  },
  {
    id: 'workspace',
    label: '/workspace',
    description: 'Show the active workspace path',
    category: 'workspace',
    type: 'client',
  },
  {
    id: 'add-workspace',
    label: '/add-workspace',
    description: 'Add a workspace directory',
    category: 'workspace',
    type: 'client',
    args: [{ name: 'path', placeholder: '/path/to/project' }],
  },
  {
    id: 'skills',
    label: '/skills',
    description: 'List installed skills',
    category: 'skills',
    type: 'client',
  },
  {
    id: 'read',
    label: '/read',
    description: 'Read a file and inject contents into context',
    category: 'workspace',
    type: 'ai-aware',
    args: [{ name: 'file', placeholder: 'src/main.rs' }],
  },
  {
    id: 'ls',
    label: '/ls',
    description: 'List files in workspace directory',
    category: 'workspace',
    type: 'ai-aware',
    args: [{ name: 'path', placeholder: 'src/' }],
  },
  {
    id: 'fetch',
    label: '/fetch',
    description: 'Fetch webpage content as context',
    category: 'web',
    type: 'ai-aware',
    args: [{ name: 'url', placeholder: 'https://example.com' }],
  },
  {
    id: 'github',
    label: '/github',
    description: 'Fetch a GitHub repository README',
    category: 'web',
    type: 'ai-aware',
    args: [{ name: 'repo', placeholder: 'owner/repo' }],
  },
  {
    id: 'skill',
    label: '/skill',
    description: 'Load skill instructions for this request',
    category: 'skills',
    type: 'ai-aware',
    args: [{ name: 'name', placeholder: 'golang' }],
  },
  {
    id: 'search-skills',
    label: '/search-skills',
    description: 'Search curated skill catalog',
    category: 'skills',
    type: 'client',
    args: [{ name: 'query', placeholder: 'search terms' }],
  },
];

export function parseCommand(input: string): { command: CommandDef; args: string[]; rest: string } | null {
  const trimmed = input.trim();
  if (!trimmed.startsWith('/')) return null;

  const parts = trimmed.split(/\s+/);
  const cmdText = parts[0].toLowerCase();
  const command = COMMANDS.find((c) => c.label === cmdText);
  if (!command) return null;

  const args = parts.slice(1);
  const rest = '';

  return { command, args, rest };
}

export function filterCommands(query: string, limit?: number): CommandDef[] {
  const lower = query.toLowerCase();
  const matches = COMMANDS.filter(
    (c) =>
      c.label.toLowerCase().includes(lower) ||
      c.id.toLowerCase().includes(lower) ||
      c.description.toLowerCase().includes(lower),
  );
  if (limit) return matches.slice(0, limit);
  return matches;
}
