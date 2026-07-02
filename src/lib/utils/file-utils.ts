export function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.min(
    Math.floor(Math.log(bytes) / Math.log(1024)),
    units.length - 1,
  );
  return `${(bytes / 1024 ** i).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
}

export function getFileType(file: File): string {
  const ext = file.name.split(".").pop()?.toLowerCase() ?? "";
  const mimeMap: Record<string, string> = {
    js: "javascript",
    jsx: "javascript",
    ts: "typescript",
    tsx: "typescript",
    py: "python",
    rb: "ruby",
    rs: "rust",
    go: "go",
    java: "java",
    c: "c",
    cpp: "c++",
    cs: "c#",
    swift: "swift",
    kt: "kotlin",
    vue: "vue",
    svelte: "svelte",
    html: "html",
    css: "css",
    scss: "scss",
    json: "json",
    yaml: "yaml",
    yml: "yaml",
    xml: "xml",
    md: "markdown",
    txt: "text",
    sql: "sql",
    sh: "shell",
    bash: "shell",
    zsh: "shell",
    dockerfile: "dockerfile",
    toml: "toml",
    lock: "text",
    gitignore: "text",
    env: "text",
  };
  return mimeMap[ext] || mimeMap[file.type] || "text";
}

export function isAllowedFileType(file: File): boolean {
  const imageTypes = [
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "image/svg+xml",
  ];
  const textTypes = [
    "text/",
    "application/json",
    "application/xml",
    "application/javascript",
  ];
  const codeTypes = [
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".py",
    ".rs",
    ".go",
    ".rb",
    ".java",
    ".c",
    ".cpp",
    ".cs",
    ".swift",
    ".kt",
    ".vue",
    ".svelte",
    ".html",
    ".css",
    ".scss",
    ".json",
    ".yaml",
    ".yml",
    ".xml",
    ".md",
    ".txt",
    ".sql",
    ".sh",
    ".toml",
    ".lock",
    ".gitignore",
    ".env",
    ".dockerignore",
    ".editorconfig",
  ];

  if (imageTypes.some((t) => file.type.startsWith(t.replace("*", ""))))
    return true;
  if (textTypes.some((t) => file.type.startsWith(t.replace("*", ""))))
    return true;
  if (codeTypes.some((ext) => file.name.endsWith(ext))) return true;

  return false;
}

export function filterFiles(files: FileList | File[]): File[] {
  return Array.from(files).filter(
    (f) => f.size <= 10 * 1024 * 1024 && isAllowedFileType(f),
  );
}
