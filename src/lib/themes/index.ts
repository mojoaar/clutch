export interface ThemeColors {
  "--color-bg": string;
  "--color-surface": string;
  "--color-surface-hover": string;
  "--color-border": string;
  "--color-text": string;
  "--color-text-muted": string;
  "--color-text-dim": string;
  "--color-primary": string;
  "--color-primary-hover": string;
  "--color-accent": string;
  "--color-success": string;
  "--color-warning": string;
  "--color-error": string;
  "--color-code-bg": string;
  "--color-scrollbar": string;
  "--color-shadow": string;
}

export interface ThemeVariant {
  name: string;
  colors: ThemeColors;
}

export interface ThemeDefinition {
  name: string;
  label: string;
  light: ThemeVariant;
  dark: ThemeVariant;
}

export const themes: Record<string, ThemeDefinition> = {
  clutch: {
    name: "clutch",
    label: "Clutch",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#f8f8fc",
        "--color-surface": "#ffffff",
        "--color-surface-hover": "#f0f0f8",
        "--color-border": "#e2e2f0",
        "--color-text": "#1a1a2e",
        "--color-text-muted": "#6b6b8a",
        "--color-text-dim": "#9a9ab8",
        "--color-primary": "#6366f1",
        "--color-primary-hover": "#5558e6",
        "--color-accent": "#f43f5e",
        "--color-success": "#10b981",
        "--color-warning": "#f59e0b",
        "--color-error": "#ef4444",
        "--color-code-bg": "#f0f0f8",
        "--color-scrollbar": "#c4c4d4",
        "--color-shadow": "rgba(0, 0, 0, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#0f0f12",
        "--color-surface": "#1a1a24",
        "--color-surface-hover": "#242438",
        "--color-border": "#2a2a3a",
        "--color-text": "#e4e4ec",
        "--color-text-muted": "#8888a0",
        "--color-text-dim": "#5c5c76",
        "--color-primary": "#6366f1",
        "--color-primary-hover": "#5558e6",
        "--color-accent": "#f43f5e",
        "--color-success": "#10b981",
        "--color-warning": "#f59e0b",
        "--color-error": "#ef4444",
        "--color-code-bg": "#242438",
        "--color-scrollbar": "#3a3a50",
        "--color-shadow": "rgba(0, 0, 0, 0.3)",
      },
    },
  },
  nord: {
    name: "nord",
    label: "Nord",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#eceff4",
        "--color-surface": "#e5e9f0",
        "--color-surface-hover": "#d8dee9",
        "--color-border": "#d8dee9",
        "--color-text": "#2e3440",
        "--color-text-muted": "#4c566a",
        "--color-text-dim": "#81a1c1",
        "--color-primary": "#5e81ac",
        "--color-primary-hover": "#4c6a94",
        "--color-accent": "#bf616a",
        "--color-success": "#a3be8c",
        "--color-warning": "#ebcb8b",
        "--color-error": "#bf616a",
        "--color-code-bg": "#d8dee9",
        "--color-scrollbar": "#c4cad4",
        "--color-shadow": "rgba(46, 52, 64, 0.08)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#2e3440",
        "--color-surface": "#3b4252",
        "--color-surface-hover": "#434c5e",
        "--color-border": "#4c566a",
        "--color-text": "#eceff4",
        "--color-text-muted": "#d8dee9",
        "--color-text-dim": "#81a1c1",
        "--color-primary": "#88c0d0",
        "--color-primary-hover": "#71b4c5",
        "--color-accent": "#bf616a",
        "--color-success": "#a3be8c",
        "--color-warning": "#ebcb8b",
        "--color-error": "#bf616a",
        "--color-code-bg": "#434c5e",
        "--color-scrollbar": "#4c566a",
        "--color-shadow": "rgba(0, 0, 0, 0.2)",
      },
    },
  },
  dracula: {
    name: "dracula",
    label: "Dracula",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#f8f8f2",
        "--color-surface": "#ffffff",
        "--color-surface-hover": "#f0f0ea",
        "--color-border": "#e0e0d8",
        "--color-text": "#282a36",
        "--color-text-muted": "#6272a4",
        "--color-text-dim": "#a0a0b0",
        "--color-primary": "#bd93f9",
        "--color-primary-hover": "#a77bf3",
        "--color-accent": "#ff79c6",
        "--color-success": "#50fa7b",
        "--color-warning": "#f1fa8c",
        "--color-error": "#ff5555",
        "--color-code-bg": "#f0f0ea",
        "--color-scrollbar": "#d4d4cc",
        "--color-shadow": "rgba(40, 42, 54, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#282a36",
        "--color-surface": "#343746",
        "--color-surface-hover": "#3e4158",
        "--color-border": "#44475a",
        "--color-text": "#f8f8f2",
        "--color-text-muted": "#6272a4",
        "--color-text-dim": "#4d4d62",
        "--color-primary": "#bd93f9",
        "--color-primary-hover": "#a77bf3",
        "--color-accent": "#ff79c6",
        "--color-success": "#50fa7b",
        "--color-warning": "#f1fa8c",
        "--color-error": "#ff5555",
        "--color-code-bg": "#1d1f27",
        "--color-scrollbar": "#44475a",
        "--color-shadow": "rgba(0, 0, 0, 0.3)",
      },
    },
  },
  cyberpunk: {
    name: "cyberpunk",
    label: "Cyberpunk",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#f5f5f5",
        "--color-surface": "#ffffff",
        "--color-surface-hover": "#ededf5",
        "--color-border": "#d1d5db",
        "--color-text": "#111827",
        "--color-text-muted": "#6b7280",
        "--color-text-dim": "#9ca3af",
        "--color-primary": "#cc0099",
        "--color-primary-hover": "#a3007a",
        "--color-accent": "#0088cc",
        "--color-success": "#00cc66",
        "--color-warning": "#ffcc00",
        "--color-error": "#ff3333",
        "--color-code-bg": "#ededf5",
        "--color-scrollbar": "#d1d5db",
        "--color-shadow": "rgba(0, 0, 0, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#0a0a0a",
        "--color-surface": "#14141e",
        "--color-surface-hover": "#1e1e2e",
        "--color-border": "#2a2a3e",
        "--color-text": "#e0e0e0",
        "--color-text-muted": "#8888aa",
        "--color-text-dim": "#555570",
        "--color-primary": "#ff00ff",
        "--color-primary-hover": "#dd00dd",
        "--color-accent": "#00ffff",
        "--color-success": "#00ff66",
        "--color-warning": "#ffcc00",
        "--color-error": "#ff3333",
        "--color-code-bg": "#1e1e2e",
        "--color-scrollbar": "#3a3a50",
        "--color-shadow": "rgba(0, 0, 0, 0.4)",
      },
    },
  },
  catppuccin: {
    name: "catppuccin",
    label: "Catppuccin",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#eff1f5",
        "--color-surface": "#e6e9ef",
        "--color-surface-hover": "#dce0e8",
        "--color-border": "#ccd0da",
        "--color-text": "#4c4f69",
        "--color-text-muted": "#6c6f85",
        "--color-text-dim": "#9ca0b0",
        "--color-primary": "#1e66f5",
        "--color-primary-hover": "#1856d2",
        "--color-accent": "#ea76cb",
        "--color-success": "#40a02b",
        "--color-warning": "#df8e1d",
        "--color-error": "#d20f39",
        "--color-code-bg": "#dce0e8",
        "--color-scrollbar": "#bcc0cc",
        "--color-shadow": "rgba(76, 79, 105, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#1e1e2e",
        "--color-surface": "#313244",
        "--color-surface-hover": "#45475a",
        "--color-border": "#45475a",
        "--color-text": "#cdd6f4",
        "--color-text-muted": "#a6adc8",
        "--color-text-dim": "#6c7086",
        "--color-primary": "#89b4fa",
        "--color-primary-hover": "#74a8f7",
        "--color-accent": "#f5c2e7",
        "--color-success": "#a6e3a1",
        "--color-warning": "#f9e2af",
        "--color-error": "#f38ba8",
        "--color-code-bg": "#313244",
        "--color-scrollbar": "#585b70",
        "--color-shadow": "rgba(0, 0, 0, 0.3)",
      },
    },
  },
  github: {
    name: "github",
    label: "GitHub",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#ffffff",
        "--color-surface": "#f6f8fa",
        "--color-surface-hover": "#eaeef2",
        "--color-border": "#d0d7de",
        "--color-text": "#1f2328",
        "--color-text-muted": "#656d76",
        "--color-text-dim": "#8b949e",
        "--color-primary": "#0969da",
        "--color-primary-hover": "#0550ae",
        "--color-accent": "#cf222e",
        "--color-success": "#1a7f37",
        "--color-warning": "#9a6700",
        "--color-error": "#d1242f",
        "--color-code-bg": "#f6f8fa",
        "--color-scrollbar": "#c0c6cb",
        "--color-shadow": "rgba(31, 35, 40, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#22272e",
        "--color-surface": "#2d333b",
        "--color-surface-hover": "#373e47",
        "--color-border": "#444c56",
        "--color-text": "#cdd9e5",
        "--color-text-muted": "#768390",
        "--color-text-dim": "#545d68",
        "--color-primary": "#539bf5",
        "--color-primary-hover": "#4690e8",
        "--color-accent": "#f47067",
        "--color-success": "#57ab5a",
        "--color-warning": "#c69026",
        "--color-error": "#e5534b",
        "--color-code-bg": "#2d333b",
        "--color-scrollbar": "#545d68",
        "--color-shadow": "rgba(0, 0, 0, 0.3)",
      },
    },
  },
  "tokyo-night": {
    name: "tokyo-night",
    label: "Tokyo Night",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#e1e2e7",
        "--color-surface": "#d6d7dd",
        "--color-surface-hover": "#c8cad2",
        "--color-border": "#b8bbc6",
        "--color-text": "#343b58",
        "--color-text-muted": "#545c7e",
        "--color-text-dim": "#9aa0b6",
        "--color-primary": "#34548a",
        "--color-primary-hover": "#2a4573",
        "--color-accent": "#965027",
        "--color-success": "#485e30",
        "--color-warning": "#8f5e15",
        "--color-error": "#8c4351",
        "--color-code-bg": "#c8cad2",
        "--color-scrollbar": "#a8acb8",
        "--color-shadow": "rgba(52, 59, 88, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#24283b",
        "--color-surface": "#1f2335",
        "--color-surface-hover": "#292e42",
        "--color-border": "#3b4261",
        "--color-text": "#c0caf5",
        "--color-text-muted": "#a9b1d6",
        "--color-text-dim": "#565f89",
        "--color-primary": "#7aa2f7",
        "--color-primary-hover": "#628ce8",
        "--color-accent": "#f7768e",
        "--color-success": "#9ece6a",
        "--color-warning": "#e0af68",
        "--color-error": "#db4b4b",
        "--color-code-bg": "#1f2335",
        "--color-scrollbar": "#565f89",
        "--color-shadow": "rgba(0, 0, 0, 0.35)",
      },
    },
  },
  monokai: {
    name: "monokai",
    label: "Monokai",
    light: {
      name: "light",
      colors: {
        "--color-bg": "#fafafa",
        "--color-surface": "#f4f4f4",
        "--color-surface-hover": "#e8e8e8",
        "--color-border": "#d4d4d4",
        "--color-text": "#272822",
        "--color-text-muted": "#5c5c52",
        "--color-text-dim": "#909082",
        "--color-primary": "#66d9ef",
        "--color-primary-hover": "#4dc8e0",
        "--color-accent": "#f92672",
        "--color-success": "#a6e22e",
        "--color-warning": "#e6db74",
        "--color-error": "#f92672",
        "--color-code-bg": "#e8e8e8",
        "--color-scrollbar": "#bcbcbc",
        "--color-shadow": "rgba(39, 40, 34, 0.06)",
      },
    },
    dark: {
      name: "dark",
      colors: {
        "--color-bg": "#2d2a2e",
        "--color-surface": "#221f22",
        "--color-surface-hover": "#3e3b3e",
        "--color-border": "#5b595c",
        "--color-text": "#f8f8f2",
        "--color-text-muted": "#939293",
        "--color-text-dim": "#727072",
        "--color-primary": "#a6e22e",
        "--color-primary-hover": "#8ec521",
        "--color-accent": "#f92672",
        "--color-success": "#a6e22e",
        "--color-warning": "#e6db74",
        "--color-error": "#f92672",
        "--color-code-bg": "#221f22",
        "--color-scrollbar": "#5b595c",
        "--color-shadow": "rgba(0, 0, 0, 0.4)",
      },
    },
  },
};

export type ThemeName = keyof typeof themes;
export type ThemeMode = "light" | "dark" | "system";

export function getEffectiveTheme(
  name: ThemeName,
  mode: ThemeMode,
): ThemeVariant {
  const theme = themes[name];
  if (mode === "system") {
    const prefersDark =
      typeof window !== "undefined" &&
      window.matchMedia("(prefers-color-scheme: dark)").matches;
    return prefersDark ? theme.dark : theme.light;
  }
  return mode === "dark" ? theme.dark : theme.light;
}

export function applyColors(colors: ThemeColors): void {
  const root = document.documentElement;
  for (const [key, value] of Object.entries(colors)) {
    root.style.setProperty(key, value);
  }
}
