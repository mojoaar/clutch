# 🚀 CLUTCH — Complete Project Prompt

## 📋 Project Overview

Build **Clutch**, a cross-platform desktop AI chat application for macOS, Linux, and Windows using **Tauri v2** (stable) + **SvelteKit** + **SQLite**. The app lives in the system tray, supports multiple LLM providers (DeepSeek, OpenCode Go, OpenCode Zen), features a skill system from skills.sh, and provides a rich chat experience with markdown rendering, syntax highlighting, file attachments, and local workspace integration.

**Tagline**: "Grab the conversation"

---

## 🏗️ Tech Stack (Stable Versions Only)

### Backend (Rust + Tauri)

```
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-sql = "2"
tauri-plugin-fs = "2"
tauri-plugin-dialog = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-updater = "2"
tauri-plugin-process = "2"
tauri-plugin-window-state = "2"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.13", features = ["json", "stream"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "tls-rustls-ring-webpki"] }
chrono = "0.4"
url = "2"
scraper = "0.20"
html2md = "0.2"         # HTML → Markdown conversion
sha2 = "0.10"           # For cache keys
tracing = "0.1"         # Structured logging
tracing-subscriber = "0.3"
tracing-appender = "0.2"  # File-based logging with rotation
tiktoken-rs = "0.5"     # Token counting
```

### Frontend (SvelteKit + Lucide)

```
@sveltejs/kit = "^2.5.0"
@sveltejs/adapter-static = "^3.0.0"
svelte = "^5.0.0"
@sveltejs/vite-plugin-svelte = "^6.0.0"
vite = "^7.0.0"
@lucide/svelte = "^1.22.0"
marked = "^18.0.0"
highlight.js = "^11.9.0"
dompurify = "^3.0.6"
uuid = "^14.0.0"
typesafe-i18n = "^5.26.0"
typescript = "^6.0.0"
vitest = "^4.0.0"
@testing-library/svelte = "^5.0.0"
playwright = "^1.61.0"
```

### Database (SQLite via tauri-plugin-sql)

- `sessions`: id (PK), title, provider, model, system_prompt, created_at, updated_at, is_archived, is_pinned, token_count
- `messages`: id (PK), session_id (FK), role, content, created_at, tokens_used, is_deleted, edited_at
- `settings`: key (PK), value, updated_at
- `user_profile`: id (PK), display_name, avatar_type, avatar_data, avatar_color, updated_at
- `model_cache`: provider (PK), models (JSON), last_updated, etag, version
- `search_index`: Virtual FTS5 table mapping to messages.content and sessions.title

---

## 🗂️ Project Structure

```
clutch/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Avatar.svelte
│   │   │   ├── AIAvatar.svelte
│   │   │   ├── ChatMessage.svelte
│   │   │   ├── ChatInput.svelte
│   │   │   ├── MarkdownRenderer.svelte
│   │   │   ├── DropZone.svelte
│   │   │   ├── FilePreview.svelte
│   │   │   ├── Timestamp.svelte
│   │   │   ├── Icon.svelte
│   │   │   ├── ThemeSelector.svelte
│   │   │   ├── ProviderSelector.svelte
│   │   │   ├── ModelSelector.svelte
│   │   │   ├── WorkspaceSelector.svelte
│   │   │   ├── ExportDialog.svelte
│   │   │   └── ShortcutRecorder.svelte
│   │   ├── db/
│   │   │   └── index.ts
│   │   ├── i18n/
│   │   │   ├── en/
│   │   │   │   ├── chat.ts
│   │   │   │   ├── settings.ts
│   │   │   │   ├── errors.ts
│   │   │   │   ├── common.ts
│   │   │   │   ├── providers.ts
│   │   │   │   ├── skills.ts
│   │   │   │   └── workspaces.ts
│   │   │   ├── da/
│   │   │   │   └── (same structure)
│   │   │   └── index.ts
│   │   ├── services/
│   │   │   ├── providers.ts
│   │   │   ├── models.ts
│   │   │   ├── skills.ts
│   │   │   ├── directories.ts
│   │   │   ├── web-fetcher.ts
│   │   │   ├── export.ts
│   │   │   ├── startup.ts
│   │   │   ├── shortcuts.ts
│   │   │   └── cache-scheduler.ts
│   │   ├── stores/
│   │   │   └── theme.ts
│   │   ├── themes/
│   │   │   └── index.ts
│   │   ├── utils/
│   │   │   ├── time-utils.ts
│   │   │   └── file-utils.ts
│   │   └── styles/
│   │       └── highlight-theme.css
│   ├── routes/
│   │   ├── chat/
│   │   │   └── +page.svelte
│   │   ├── settings/
│   │   │   └── +page.svelte
│   │   ├── +layout.svelte
│   │   ├── +layout.js          # ssr: false
│   │   └── +page.svelte
│   ├── app.html
│   └── app.css
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── startup.rs
│   │   ├── shortcuts.rs
│   │   ├── model_cache.rs
│   │   ├── web_fetcher.rs
│   │   ├── export.rs
│   │   └── prompt_builder.rs
│   ├── migrations/
│   │   ├── 001_initial_schema.sql
│   │   ├── 002_user_profile.sql
│   │   ├── 003_theme_settings.sql
│   │   ├── 004_time_settings.sql
│   │   ├── 005_startup_settings.sql
│   │   ├── 006_shortcut_settings.sql
│   │   ├── 007_export_settings.sql
│   │   ├── 008_model_cache.sql
│   │   └── 009_search_index.sql
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   │   ├── icon.svg
│   │   └── (generated png/icns/ico files)
│   ├── Cargo.toml
│   └── tauri.conf.json
├── tests/
│   ├── e2e/
│   └── integration/
├── .github/
│   └── workflows/
│       └── ci.yml
├── package.json
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
├── LICENSE (AGPL-3.0)
└── README.md
```

---

## 🔧 Core Features to Implement

### 1. System Tray & Window Management

- App starts with system tray icon (asteroid icon)
- Click tray icon → show/hide main window
- Right-click → menu: Show, Hide, Quit, Settings
- Popup window on global shortcut (Cmd/Ctrl+Shift+P)
- Main window on global shortcut (Cmd/Ctrl+Shift+M)
- Close button → hide to tray (if setting enabled)
- Cross-platform auto-start (Registry for Windows, LaunchAgents for macOS, autostart for Linux)
- Remember window position and size across sessions (tauri-plugin-window-state)

### 2. Provider Integration

**Provider Configuration (UI in Settings)**:

- Each provider (DeepSeek, OpenCode Go, OpenCode Zen) has:
  - API Key input (password field)
  - Default model dropdown (dynamically loaded from cache)
  - Enable/disable toggle
  - Test connection button

**API Endpoints**:

- DeepSeek: `https://api.deepseek.com/v1/chat/completions`
- OpenCode Go: `https://opencode.ai/zen/go/v1/chat/completions`
- OpenCode Zen: `https://opencode.ai/zen/v1/chat/completions`

**Model Discovery**:

- Cache model lists in SQLite with 24-hour TTL
- Auto-refresh on app start (if cache > 24h)
- Manual refresh button in settings
- Provider API keys stored encrypted in SQLite (settings table)

**Model Categorization** (display in UI):

- OpenCode Go: MiniMax, Kimi, GLM, DeepSeek, Qwen, Mimo
- OpenCode Zen: Claude, Gemini, GPT, DeepSeek, Grok, Free models

### 3. Chat Interface

**Main Chat Window**:

- Message list with streaming responses
- Markdown rendering with syntax highlighting
- Code blocks with copy buttons
- Tables, lists, blockquotes
- User avatar (uploaded image, emoji, initials, or UserCircle icon)
- AI avatar (Bot icon with provider color dot)
- Copy message button (hover)
- Regenerate response button (for assistant messages)
- Timestamps (auto-detected 12h/24h, configurable)

**Chat Input**:

- Text area with auto-resize (max-height: 200px)
- Drag & drop file attachments (click to browse)
- File previews with progress indicators
- Send button (disabled when empty/streaming)
- Attach button (opens file browser)
- Enter to send, Shift+Enter for new line

**Message Metadata**:

- Provider label (e.g., "Clutch · DeepSeek")
- Token usage display (optional)
- Timestamps with tooltip (full date/time)

### 4. Settings Management

**Settings Pages**:

- **General**: Theme selector, Time format (Auto/12h/24h), Timestamp visibility, Language selector, Developer mode toggle
- **Providers**: API keys, default models per provider, test connection
- **Startup**: Run on system startup, Start minimized, Close to tray
- **Shortcuts**: Global hotkeys (Show Main)
- **Workspaces**: Add/remove project folders, select active workspace
- **Export**: Default format, include metadata/timestamps/provider info
- **Models**: View cache status, manual refresh, force refresh all

**Settings Storage**: All settings in SQLite (`settings` table)

### 5. Skills Integration (skills.sh)

**Skill Discovery** (via skills.sh API):

- Search endpoint: `https://skills.sh/api/search?q={query}&limit={limit}`
- Trending skills: Combine popular search terms
- Skill details: Fetch from GitHub (`https://raw.githubusercontent.com/{source}/{branch}/{skillId}/SKILL.md`)

**Skill Installation**:

- Download skill to `~/.clutch/skills/{skillId}/`
- Parse `SKILL.md` for metadata and instructions
- Store installed skills in SQLite

**Skill Execution with Workspace Context**:

- Skills can read/write files in the active workspace
- Execute shell commands in workspace directory
- Parse skill output and display in chat
- Support for `read_file`, `write_file`, `list_dir`, `run_command` actions

**Skill UI**:

- Browse installed skills in a dedicated page or modal
- Search skills.sh for new skills
- Install/Uninstall skills
- Skill execution with workspace context

**Bundled Skills (Anthropic + Superpowers)**:

Clutch ships with 26 pre-installed skills from two sources. All are evaluated for value to Clutch end-users (developers, business users, casual chatters) — not for AI agent development.

_Anthropic Skills_ (`github.com/anthropics/skills`) — 16 bundled, 1 excluded:

| #   | Skill                 | Value | Use Case                                                      |
| --- | --------------------- | ----- | ------------------------------------------------------------- |
| 1   | docx                  | High  | Create, edit, and read Word documents                         |
| 2   | pdf                   | High  | Read, extract, and manipulate PDFs                            |
| 3   | pptx                  | High  | Create and edit PowerPoint presentations                      |
| 4   | xlsx                  | High  | Create, edit, and analyze Excel spreadsheets                  |
| 5   | internal-comms        | High  | Write status updates, memos, newsletters                      |
| 6   | doc-coauthoring       | High  | Co-author documents with AI assistance                        |
| 7   | frontend-design       | Med   | UI/UX design guidance and frontend best practices             |
| 8   | canvas-design         | Med   | Visual design creation and layout guidance                    |
| 9   | brand-guidelines      | Med   | Brand identity and style guide creation                       |
| 10  | mcp-builder           | Med   | Build MCP servers (developer audience)                        |
| 11  | webapp-testing        | Med   | Test and audit web apps via Playwright (developer audience)   |
| 12  | web-artifacts-builder | Med   | Build reusable web components and artifacts                   |
| 13  | skill-creator         | Med   | Create custom skills for extended AI capabilities             |
| 14  | claude-api            | Med   | Claude API integration (relevant if Claude added as provider) |
| 15  | theme-factory         | Low   | Generate theme artifacts and visual styles                    |
| 16  | algorithmic-art       | Low   | Create generative art with p5.js                              |

**Excluded**: `slack-gif-creator` — too Slack-specific; no value for a desktop chat app.

`claude-api` is conditionally relevant: bundled only if Claude is added as a provider (BYO API key). If Claude is not on the roadmap, skip this skill.

_Superpowers Skills_ (`github.com/obra/superpowers`) — 10 bundled, 4 excluded:

| #   | Skill                          | Value | Use Case                                                 |
| --- | ------------------------------ | ----- | -------------------------------------------------------- |
| 1   | brainstorming                  | High  | Guides AI to explore requirements before implementing    |
| 2   | systematic-debugging           | High  | Structured debugging workflow for any "help me fix this" |
| 3   | test-driven-development        | High  | Test-first development guidance for code generation      |
| 4   | verification-before-completion | High  | AI verifies work before claiming done — quality guard    |
| 5   | writing-plans                  | High  | Plan before coding for any multi-step task               |
| 6   | finishing-a-development-branch | Med   | Git branch completion workflow (developer users)         |
| 7   | receiving-code-review          | Med   | Code review response workflow (developer users)          |
| 8   | requesting-code-review         | Med   | Pre-submit validation checklist (developer users)        |
| 9   | using-git-worktrees            | Low   | Git worktree isolation pattern — niche                   |
| 10  | writing-skills                 | Low   | Meta skill for skill creators and contributors           |

**Excluded**: `dispatching-parallel-agents`, `executing-plans`, `subagent-driven-development`, `using-superpowers` — these are agent-internal orchestration patterns not meaningful in a conversational chat context.

**Skill Update Mechanism**:

Skills are bundled as SKILL.md files in `src-tauri/resources/skills/`. On first run, Clutch extracts them to `~/.clutch/skills/{id}/` and stores a version marker (commit SHA for Anthropic, release tag for Superpowers) in the settings table as `skill:{id}:sha`.

Updates are **manual only** — a "Check for updates" button in the Skills page:

1. Anthropic source: `GET /repos/anthropics/skills/commits/main` → compare SHA
2. Superpowers source: `GET /repos/obra/superpowers/releases/latest` → compare tag
3. Show diff: "3 skills have updates" with per-skill changelog
4. User clicks "Update All" or per-skill "Update"
5. Re-download SKILL.md from GitHub raw → update local file + version marker

No auto-check on startup — avoids unnecessary GitHub API calls. 60 unauthenticated requests/hour is plenty for manual use.

**What to build**:

| Layer     | What                                                                |
| --------- | ------------------------------------------------------------------- |
| Resources | 16 + 10 SKILL.md files in `src-tauri/resources/skills/`             |
| Rust      | `check_skill_updates(source)` — GitHub API call to compare versions |
| Rust      | Modify `install_skill` to store version SHA/tag                     |
| Rust      | `update_skill(id)` — re-download + update version marker            |
| Frontend  | Extend `skills.ts` with `checkForUpdates`, `updateSkill`            |
| UI        | "Check for updates" button + diff list in Skills page/modal         |

### 6. Workspace & Local Directories

**Workspace Management**:

- Auto-detect common dev directories (Projects, Code, Dev, workspace)
- User can manually add/remove folders
- One active workspace at a time
- Store workspaces in SQLite (settings table)

**File Operations**:

- Read files in workspace (via Tauri FS)
- Write files in workspace
- List directory contents
- Create/delete directories
- Skills can operate on workspace files

**Workspace Detection**:

- Check for project indicators: `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `.git`, etc.

### 7. Web Fetching

**Secure Fetching** (via Tauri backend):

- Block local/private IPs (security)
- Configurable timeouts (default 30s)
- Size limits (default 10MB)
- Custom headers support
- Follow redirects (max 10)

**Fetch Modes**:

- Raw content (as text)
- HTML → Markdown conversion (using html2md)
- Webpage info (title, description, OG tags)
- Batch fetch multiple URLs

**GitHub Integration**:

- Parse GitHub repo URLs
- Fetch README.md (try main → master → develop)
- Fetch raw files from repositories

### 8. Export & Sharing

**Export Formats**:

- Markdown (.md) — Primary format
- Plain Text (.txt)
- JSON (.json)
- HTML (.html)

**Export Options**:

- Include metadata (provider, model, created/updated, tokens)
- Include timestamps per message
- Include provider info
- Include system prompt (if present)

**Export Flow**:

- User clicks export button in chat header
- Export dialog opens with preview
- User selects options
- Save dialog (native OS) to choose location

### 9. Token Optimization

**Provider-Side Caching**:

- **DeepSeek**: Automatic disk caching (stable prefix → 90% savings)
- **OpenCode (Kimi/GLM)**: Automatic caching, ensure stable tool schemas
- **OpenCode (MiniMax)**: Limited caching (15k cap), avoid for tool-heavy tasks

**Prompt Architecture**:

- Split prompt into **Stable Prefix** (cacheable) and **Dynamic Suffix** (not cached)
- Stable Prefix: System prompt, skill definitions, global context
- Dynamic Suffix: Workspace context, user query, attachments

**Cache Strategy**:

- Sort skills alphabetically (prevent nondeterministic invalidation)
- Keep workspace context minimal (path + project type, not full file listings)
- Cache skills by version hash
- Monitor cache hit rates via API response tokens

### 10. Theming

**Built-in Themes**:

1. Clutch (custom, signature theme)
2. Nord (arctic color palette)
3. Dracula (purple/pink)
4. Cyberpunk (neon colors)

**Theme System**:

- CSS custom properties for theming
- Light/Dark mode per theme
- System preference detection
- Persistent storage in SQLite
- Smooth transitions

**Font**: JetBrains Mono (web font)

---

## 🔐 Security & Permissions

### API Keys

- Stored in SQLite (settings table)
- Each provider has its own API key field
- User sets them in Settings → Providers
- Never stored in plaintext in logs or frontend state

### File System Access

- User selects directories via dialog (no arbitrary access)
- Operations scoped to allowed directories
- Tauri FS plugin with scope configuration
- Security: Block local/private IPs in web fetcher

### Shortcuts

- Only active when app is running
- Configurable to avoid system conflicts
- Reserved keys blocked (Cmd+W, Cmd+Q, etc.)

### Licensing

- **AGPL-3.0** (strong copyleft, open source)
- Include LICENSE file in root
- Add license headers to source files
- Notice in app UI

---

## 🚀 Release & Distribution

### Build Targets

- **Windows**: `.msi` or `.exe` installer
- **macOS**: `.app` bundle and `.dmg` (Universal Binary)
- **Linux**: `.deb`, `.AppImage`, or `.rpm`

### Platform Requirements

- **Windows**: WebView2 runtime (bundled)
- **macOS**: macOS 10.15+ (Catalina)
- **Linux**: `libwebkit2gtk-4.0-dev`, `libgtk-3-dev`

### Icons

- App icon: Asteroid (512x512 base)
- Tray icon: Simplified asteroid (16x16)
- Generate all sizes via `pnpm tauri icon`

---

## 📦 Dependencies Checklist

### Rust Dependencies

```
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-sql = "2"
tauri-plugin-fs = "2"
tauri-plugin-dialog = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-updater = "2"
tauri-plugin-process = "2"
tauri-plugin-window-state = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.13", features = ["json", "stream"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "tls-rustls-ring-webpki"] }
chrono = "0.4"
url = "2"
scraper = "0.20"
html2md = "0.2"
sha2 = "0.10"
tracing = "0.1"
tracing-subscriber = "0.3"
tracing-appender = "0.2"
tiktoken-rs = "0.5"
```

### NPM Dependencies

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-sql": "^2.0.0",
    "@tauri-apps/plugin-fs": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0",
    "@tauri-apps/plugin-global-shortcut": "^2.0.0",
    "@tauri-apps/plugin-updater": "^2.0.0",
    "@tauri-apps/plugin-process": "^2.0.0",
    "@tauri-apps/plugin-window-state": "^2.0.0",
    "@lucide/svelte": "^1.22.0",
    "marked": "^18.0.0",
    "highlight.js": "^11.9.0",
    "dompurify": "^3.0.6",
    "uuid": "^9.0.0",
    "typesafe-i18n": "^5.26.0"
  },
  "devDependencies": {
    "@sveltejs/kit": "^2.5.0",
    "@sveltejs/adapter-static": "^3.0.0",
    "svelte": "^5.0.0",
    "@sveltejs/vite-plugin-svelte": "^6.0.0",
    "vite": "^7.0.0",
    "typescript": "^6.0.0",
    "vitest": "^4.0.0",
    "@testing-library/svelte": "^5.0.0",
    "playwright": "^1.61.0"
  }
}
```

---

## 🎯 Key Design Decisions

1. **PNPM** as package manager
2. **SvelteKit CSR** mode (`ssr: false`, `adapter-static`)
3. **Tauri v2 stable** (not beta)
4. **SQLite** for all local storage
5. **Lucide** for icons
6. **JetBrains Mono** for font
7. **AGPL-3.0** for license
8. **Dark/Light** themes per theme
9. **Natural time display** (auto-detect 12h/24h)
10. **Workspace auto-detection** of common dev directories
11. **Global skills storage** in `~/.clutch/skills/`
12. **Per-provider API keys** stored in SQLite
13. **Model cache refresh**: 24h TTL, background check every 12h
14. **Popup window**: frameless, 420x600, always on top

---

## 🧪 Testing Strategy

**Approach**: Test from day one. Every feature ships with tests.

### Frontend (Vitest + Playwright)

```
vitest (unit + component tests)
@testing-library/svelte (component rendering)
playwright (E2E, multi-window, tray interaction)
```

- **Unit tests**: Stores, utilities, services (pure logic)
- **Component tests**: Every Svelte component with `@testing-library/svelte`
- **E2E tests**: Critical user flows (send message, switch providers, export, settings)
- **Coverage target**: 80%+ lines, 90%+ branches for core services
- **CI**: Vitest runs on every push; Playwright runs on PR to main

### Backend (cargo test + integration)

```
cargo test (unit tests)
cargo test --test integration (integration tests with temp SQLite)
```

- **Unit tests**: Every Tauri command handler, model cache, web fetcher, prompt builder
- **Integration tests**: Database migrations, full API round-trip (mock HTTP server)
- **Coverage**: `cargo-tarpaulin` in CI

### Test Organization

```
src/__tests__/           # Frontend tests co-located or in __tests__
  components/
  stores/
  services/
src-tauri/tests/         # Rust integration tests
  integration/
  fixtures/
tests/                   # Root-level E2E and integration
  e2e/
  integration/
```

---

## ❌ Error Handling

**Principle**: Errors are first-class UI. Never fail silently.

### Error Types

```typescript
type AppError =
  | { kind: "api"; provider: string; status: number; message: string }
  | { kind: "network"; detail: string }
  | { kind: "file_system"; operation: string; path: string; detail: string }
  | { kind: "parse"; source: string; detail: string }
  | { kind: "validation"; field: string; detail: string }
  | { kind: "timeout"; operation: string }
  | { kind: "stream_interrupted"; sessionId: string; partialContent: string };
```

### Error Surface

- **Toast notifications**: Transient errors (timeouts, transient network issues). Auto-dismiss after 8s.
- **Inline banners**: Persistent errors (invalid API key, workspace not found). Require user action.
- **Message-level error**: Failed AI response shown as error bubble with retry button.
- **Stream interruption**: Partial message preserved, "Response interrupted — Retry?" banner with resume button.

### Retry Strategy

- **Exponential backoff**: 1s → 2s → 4s → 8s (max 3 retries) for transient network errors
- **No retry for**: 4xx errors (bad API key, rate limit), validation errors
- **Streaming retry**: Resume from last complete token on connection drop (if provider supports it)

### Error Boundaries (Svelte)

- Top-level error boundary in `+layout.svelte` catches unhandled errors
- Per-component boundaries for chat messages, settings panels
- Fallback UI: "Something went wrong" with reload button

---

## 🔄 Loading & Empty States

### Loading States

| State                 | UI                                    |
| --------------------- | ------------------------------------- |
| App startup           | Branded splash screen (1-2s)          |
| Loading sessions      | Skeleton list (3 rows) in sidebar     |
| Loading messages      | Skeleton chat bubbles (alternating)   |
| Streaming response    | Animated cursor + token count counter |
| Model list loading    | Skeleton dropdown                     |
| Export generating     | Progress spinner with format label    |
| Searching             | Spinner in search bar                 |
| Connecting provider   | Pulsing "Testing connection..."       |
| Web fetch in progress | Progress bar + URL display            |

### Empty States

| State                   | UI                                                     |
| ----------------------- | ------------------------------------------------------ |
| First run (no sessions) | Welcome screen: app logo, tagline, "Start a chat" CTA  |
| No messages in session  | "Send a message to get started" with provider selector |
| No search results       | "No results for '{query}'" with clear search button    |
| No skills installed     | Browse skills CTA linking to skills.sh                 |
| No workspaces added     | "Add a project folder to get started"                  |
| Provider not configured | "Add your DeepSeek API key in Settings" with link      |
| All sessions archived   | "No active conversations" with link to archives        |

---

## 🔐 Tauri v2 Capabilities Config

**Location**: `src-tauri/capabilities/default.json`

```json
{
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-close",
    "core:window:allow-set-focus",
    "core:window:allow-set-always-on-top",
    "sql:default",
    "sql:allow-load",
    "sql:allow-execute",
    "sql:allow-select",
    "fs:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file",
    "fs:allow-exists",
    "fs:allow-mkdir",
    "fs:allow-read-dir",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "dialog:allow-ask",
    "dialog:allow-confirm",
    "dialog:allow-message",
    "global-shortcut:default",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-is-registered",
    "updater:default",
    "updater:allow-check",
    "updater:allow-download-and-install",
    "process:default",
    "process:allow-restart"
  ]
}
```

**Scope restrictions in `tauri.conf.json`**:

- `fs` scope: Only `$APPDATA/**`, `$HOME/.clutch/**`, and user-selected workspace directories
- `shell` scope: Only workspace directories (for skill execution)
- No `shell:allow-open` — use `dialog:allow-open` instead

---

## 🌐 Offline & Network Resilience

### Connectivity Detection

- **Tauri event**: Listen for `tauri://network-status` (use `navigator.onLine` + periodic health check)
- **Status bar indicator**: Green dot (online), yellow (degraded), red (offline)
- **Offline mode**: Full chat UI available, messages queue locally, sync when back online

### Retry Queue

- All outbound API calls enqueued in IndexedDB-backed queue
- Queue processor picks up when connectivity restored
- User-visible: "X messages queued" badge with manual retry option
- Max queue depth: 50 items; oldest evicted on overflow with user warning

### Graceful Degradation

| Feature        | Online                 | Offline               |
| -------------- | ---------------------- | --------------------- |
| Send message   | Normal                 | Queued (badge shown)  |
| Model list     | From cache/settings    | From cache only       |
| Web fetch      | Active                 | Disabled (grayed out) |
| Skills install | Active                 | Disabled              |
| Skills execute | Active (API-dependent) | Local-only skills     |
| Settings       | Full                   | Full (local)          |
| Export         | Full                   | Full (local)          |
| Search         | Full                   | Full (local SQLite)   |

---

## ✏️ Message Management

### Edit Messages

- Only user messages can be edited
- Click edit icon (hover) → inline text area → Save/Cancel
- "Edited" indicator badge on edited messages
- Editing a message regenerates the AI response that follows it
- Store `edited_at` timestamp in messages table

### Delete Messages

- Soft-delete: `is_deleted` flag in messages table
- Deleted messages show "This message was deleted" placeholder
- Delete confirmation dialog (hold Shift to skip)
- Bulk delete: Select multiple → Delete selected

### Pin Conversations

- Pin icon in session sidebar (hover)
- Pinned sessions appear at top, sorted by pin date
- `is_pinned` field in sessions table

### Bulk Actions

- Select mode: Click checkboxes on sessions
- Actions on selection: Archive, Delete, Export

---

## 🔍 Search

### Implementation

- **SQLite FTS5**: Virtual table over `sessions.title` + `messages.content`
- Trigger + index updated on insert/update/delete
- Search results grouped by session with match highlights

### Search UI

- **Trigger**: Cmd/Ctrl+K or click search icon in sidebar
- **Search bar**: Slides in from top of sidebar, auto-focuses
- **Results**: Grouped by session, match snippets highlighted
- **Navigation**: Arrow keys to move through results, Enter to open session
- **Recent searches**: Last 10 searches stored locally

### Search Scope

- Session titles (fuzzy match)
- Message content (full-text)
- Filter by: Provider, Date range, Archived/Active

---

## ♿ Accessibility

**Target**: WCAG 2.1 AA compliance.

### Keyboard Navigation

- **Tab order**: Logical (sidebar → chat → input → actions)
- **Shortcuts**: Full keyboard shortcut map (see Shortcuts section)
- **Focus trapping**: In modals and dialogs
- **Skip link**: "Skip to chat" link at top of page (visually hidden, focusable)

### Screen Reader

- **ARIA labels**: All interactive elements (`aria-label`, `aria-describedby`)
- **Live regions**: `aria-live="polite"` for streaming responses (announce new tokens at intervals)
- **Role annotations**: `role="log"` on chat container, `role="article"` on messages
- **Status announcements**: Connection changes, error states, loading completion

### Visual Accessibility

- **Contrast**: All themes meet 4.5:1 minimum (AA) for text
- **Focus indicators**: Visible focus ring on all interactive elements (2px, primary color)
- **Reduced motion**: Respect `prefers-reduced-motion` — disable transitions/animations
- **Text scaling**: Layout works at 200% zoom without horizontal scroll

---

## 📦 Auto-Update

### Implementation

- **Plugin**: `tauri-plugin-updater` (official Tauri v2 plugin)
- **Update manifest**: Hosted as `latest.json` on GitHub Releases
- **Signature**: Public key embedded in app, manifest + binaries signed

### Update Flow

```
App start → Check for updates (background)
  → Update available? → Silent download
    → User notified: "Update ready. Restart now / Later"
      → Restart → Install → Relaunch
```

### Update Settings

- **Auto-check**: On/off (default on)
- **Auto-download**: On/off (default on)
- **Channel**: Stable / Beta (dropdown)
- **Last check timestamp**: Stored in settings table
- **Check now**: Manual trigger button in Settings → General

### Platform Notes

- **macOS**: `.tar.gz` or `.dmg` updater, requires code signing
- **Windows**: `.msi` or `.nsis` updater
- **Linux**: `.AppImage` updater

---

## 🪵 Logging & Debugging

### Rust Backend (`tracing` crate)

```rust
// Levels: ERROR, WARN, INFO, DEBUG, TRACE
// Outputs: Console (dev), File (production at ~/.clutch/logs/)
tracing_subscriber::fmt()
    .with_max_level(if cfg!(debug_assertions) { Level::DEBUG } else { Level::INFO })
    .with_writer(tracing_appender::rolling::daily("~/.clutch/logs", "clutch.log"))
    .init();
```

- **Log rotation**: Daily, keep 7 days
- **Log format**: Structured JSON (production), human-readable (dev)
- **Categories**: `api`, `db`, `fs`, `skills`, `web_fetcher`, `startup`, `window`

### Frontend Logging

- **Structured logger**: Wraps `console.*` with levels and categories
- **Dev mode**: Full debug output to browser console
- **Production**: Only ERROR and WARN, no console output unless devtools enabled

### In-App DevTools

- **Toggle**: Settings → Developer → Enable DevTools (off by default)
- **Menu bar**: View → Toggle Developer Tools (always available in dev builds)
- **Log viewer**: Built-in log viewer page (accessible when devtools enabled) showing:
  - Real-time log stream
  - Filter by level, category, time range
  - Export logs as file
- **Feature flags**: `--devtools` CLI flag to force-enable

---

## 🗄️ Database Pragmas

Applied on every connection open:

```sql
PRAGMA journal_mode = WAL;           -- Write-Ahead Logging (better concurrency)
PRAGMA foreign_keys = ON;            -- Enforce FK constraints
PRAGMA busy_timeout = 5000;          -- Wait up to 5s on lock
PRAGMA cache_size = -64000;          -- 64MB page cache
PRAGMA synchronous = NORMAL;         -- Balance safety vs performance (WAL already safe)
PRAGMA temp_store = MEMORY;          -- Temp tables in memory
```

### Connection Pool

- **Pool size**: 5 connections (sqlx)
- **Acquire timeout**: 30s
- **Idle timeout**: 10min

### Migration Safety

- Run all migrations in a transaction
- On migration failure: Rollback entire migration batch, log error, alert user
- Migration history tracked in `_migrations` table (managed by sqlx)
- Never modify existing migrations — always add new ones

---

## 🛡️ CSP & Security Headers

### Content Security Policy (tauri.conf.json)

```json
{
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https://api.deepseek.com https://opencode.ai https://skills.sh https://raw.githubusercontent.com; font-src 'self' data:;"
    }
  }
}
```

### IPC Security

- **Isolation pattern**: Frontend invokes Tauri commands (not direct plugin calls)
- **Input validation**: All command arguments validated server-side (Rust)
- **No eval()**: No dynamic code execution in frontend
- **DOMPurify**: All rendered markdown sanitized before DOM insertion
- **File path validation**: All paths verified as within allowed scopes before read/write

### API Key Handling

- Keys never logged (filtered in `tracing` spans)
- Keys never sent to frontend after initial save (only masked view `sk-...****`)
- Keys stored in settings table (encryption optional via OS keychain integration in future)

---

## ⚡ Performance at Scale

### Message List (Virtual Scrolling)

- **Library**: `svelte-virtual-list` or custom implementation
- **Threshold**: Enable virtual scrolling when session has >50 messages
- **Rendering**: Only DOM nodes for visible messages + buffer (3 above, 3 below)
- **Estimated item height**: 120px average, recalculated on render

### Session Sidebar

- **Pagination**: Load 20 sessions initially, "Load more" button
- **Lazy metadata**: Don't load message previews until session is focused

### Streaming Performance

- **Rust-side streaming**: `reqwest` streaming response → `tokio::sync::mpsc` channel → Tauri events
- **Frontend**: Batch DOM updates via `requestAnimationFrame` (not every token)
- **Markdown rendering**: Debounce re-render to 100ms intervals during active streaming
- **Token counting**: Offload to Rust (tiktoken-rs), don't block UI thread

### Memory Management

- **Session cache**: Keep last 3 sessions warm, evict others from memory
- **Image/attachment limits**: Max 10MB per file, max 5 files per message
- **Log cleanup**: Automatic log rotation (daily, 7-day retention)

---

## 🔁 Streaming Error Recovery

### Failure Modes

| Failure              | Behavior                                                      |
| -------------------- | ------------------------------------------------------------- |
| Connection drop      | Preserve partial response, show "Response interrupted" banner |
| Provider error (5xx) | Preserve partial, retry button → resumes from last token      |
| Timeout              | Preserve partial, show elapsed time, retry button             |
| Content filter       | Show provider's refusal message as system message             |
| Rate limit           | Show "Rate limited — retrying in Xs" with countdown           |

### Partial Message Handling

```
Stream token flow:
  onToken(text) → append → accumulate in message store
  onError(err)  → mark message as "interrupted" (not "complete")
                  → show retry banner with provider info
  onComplete()  → mark message as "complete"
```

- **Resume strategy**: On retry, send entire conversation + partial response as context
- **User choice**: "Retry from interruption" (default) or "Retry from scratch"
- **Edge case**: If provider doesn't support resume, retry from scratch automatically

---

## 📏 Context Window Management

### Token Counting

- **Rust backend**: `tiktoken-rs` with per-model encoding
- **Count on**: Every message send, every message receive
- **Stored**: Per-message `tokens_used`, per-session `token_count` (sum)

### Context Bar UI

- **Location**: Below chat input (subtle, expands on hover)
- **Display**: Visual gauge `████████░░` "2,430 / 8,192 tokens (30%)"
- **Color coding**: Green (<50%), Yellow (50-80%), Red (>80%), Flashing red (>95%)

### Auto-Trim Strategy

```
When context exceeds 90% of model limit:
  1. Preserve: System prompt (always)
  2. Preserve: Last 4 messages (most relevant)
  3. Preserve: Pinned messages (user-marked important)
  4. Trim: Oldest non-preserved messages first
  5. Model limit: Derived from model metadata or API response
  6. Notification: "Context window reached — older messages trimmed" banner
```

- **User override**: "Never trim" option (will hit API error instead)
- **Manual trim**: "Trim context" button to force cleanup
- **Trim indicator**: "(3 messages trimmed)" displayed in chat at trim boundary

---

## 🌍 Internationalization (i18n)

### Framework

- **Library**: `typesafe-i18n` (type-safe, tree-shakeable, Svelte-native)
- **Languages**: English (`en`), Danish (`da`), German (`de`), Polish (`pl`), French (`fr`)
- **Fallback**: `en` for any missing key in other languages

### Namespace Structure

All translations are in flat `index.ts` files, one per locale. Namespaces are defined inline as nested objects.

```
src/lib/i18n/
  en/
    index.ts          # English translations (source of truth)
  da/
    index.ts          # Danish translations
  de/
    index.ts          # German translations
  pl/
    index.ts          # Polish translations
  fr/
    index.ts          # French translations
  i18n-svelte.ts      # Auto-generated Svelte adapter (LL store)
  i18n-types.ts       # Auto-generated types (RootTranslation, TranslationFunctions)
  i18n-util.ts        # Auto-generated utilities (locales[], loadedLocales)
  i18n-util.sync.ts   # Auto-generated sync loader
  i18n-util.async.ts  # Auto-generated async loader
```

Each locale file exports a single object implementing the `Translation` type with these namespaces:

| Namespace          | Use                         |
| ------------------ | --------------------------- |
| Root (top-level)   | Common actions, labels      |
| `chat`             | Chat UI strings             |
| `errors`           | Error messages              |
| `providers`        | Provider names, API labels  |
| `startup`          | Startup behavior labels     |
| `shortcuts`        | Keyboard shortcut labels    |
| `workspaces`       | Workspace management        |
| `skills`           | Skills UI                   |
| `theme`            | Theme/time format labels    |
| `exportSettings`   | Export options              |
| `settingsTabs`     | Settings navigation tabs    |
| `settingsSections` | Settings section headings   |
| `profile`          | User profile labels         |
| `permissions`      | Permission checkboxes       |
| `shortcutActions`  | Shortcut action names       |
| `welcome`          | Welcome screen text         |
| `networkStatus`    | Network status labels       |
| `modelCache`       | Model cache management      |
| `dropZone`         | File drop zone hints        |
| `contextBar`       | Context window bar          |
| `exportFormats`    | Export format labels        |
| `errorBoundary`    | Error boundary UI           |
| `searchDialog`     | Search dialog text          |
| `toast`            | Toast notification messages |
| `aria`             | ARIA labels and alt text    |
| `about`            | About section labels        |

### Usage in Components

```svelte
<script lang="ts">
  import LL from '$lib/i18n/i18n-svelte';
</script>

<h1>{$LL.welcome.title()}</h1>
<button>{$LL.save()}</button>
```

### Language Detection

- **First run**: Detect OS language, default to `en` if not in supported list
- **Persistence**: Language preference stored in settings table
- **Selector**: Dropdown in Settings → General, in-app language switcher in sidebar footer

### RTL Readiness

- Layout uses CSS logical properties (`margin-inline-start`, `padding-inline-end`)
- No hardcoded text directions — all via CSS

---

## 📌 Versioning Strategy

### Version Scheme

- **SemVer**: `MAJOR.MINOR.PATCH` (e.g., `1.2.3`)
- **MAJOR**: Breaking changes (API, data format, plugin interface)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes, performance improvements

### Commit Convention

```
feat: add session search via FTS5
fix: handle stream interruption gracefully
feat!: redesign settings as tabbed layout (breaks setting key names)
chore: update tauri to 2.0.1
docs: add testing strategy to AGENTS.md
```

### Source of Truth

- **Single source**: `version` field in `tauri.conf.json`
- **Sync script** (pre-build): Reads `tauri.conf.json` → writes to `package.json` version + `Cargo.toml` version
- **CI check**: Fails build if versions are out of sync

### Changelog

- **Format**: [Keep a Changelog](https://keepachangelog.com) (Added, Changed, Deprecated, Removed, Fixed, Security)
- **Generation**: Automated from conventional commits via `git-cliff` or similar
- **Location**: `CHANGELOG.md` in repo root, also shown in-app on update

---

## 🔧 CI/CD

### GitHub Actions Workflow

**`.github/workflows/ci.yml`** (on push + PR):

```
1. Lint:
   - Frontend: pnpm lint + svelte-check
   - Backend: cargo clippy + cargo fmt --check

2. Test:
   - Frontend: pnpm vitest run
   - Backend: cargo test
   - E2E: pnpm playwright test (PR to main only)

3. Build:
   - Matrix: macOS (x86_64, aarch64), Windows (x86_64), Linux (x86_64)
   - Artifact: Upload to GitHub Releases (tagged pushes only)

4. Release:
   - Trigger: Pushed version tag (v1.2.3)
   - Create GitHub Release with changelog
   - Upload platform binaries
   - Generate update manifest (latest.json)
```

### Version Bump Automation

```bash
# Script: scripts/bump-version.sh
# Reads tauri.conf.json → bumps → syncs → commits → tags
```

---

## 📋 Implementation Order

```
Phase 1 — Foundation
  1.  Project Setup                 Tauri init + SvelteKit + pnpm + deps + adapter-static
  2.  Tauri v2 Capabilities         capabilities/default.json + CSP + tauri.conf.json v2 schema
  3.  Database + Pragmas            Migrations 001-009 + WAL/FK/busy timeout + DB service
  4.  i18n Framework                typesafe-i18n setup, en + da locale files, namespace structure
  5.  Testing Setup                 Vitest + Playwright + cargo test configs, first smoke tests
  6.  Logging & Debugging           tracing (Rust) + frontend logger + devtools toggle

Phase 2 — Core UI
  7.  Theme System                  CSS variables, theme store, 4 themes (Clutch, Nord, Dracula, Cyberpunk)
  8.  Tray & Window                 System tray icon, show/hide, right-click menu, global shortcuts
  9.  Error Handling                Error types, toast system, retry patterns, error boundaries
  10. Loading & Empty States        Skeletons, streaming indicator, empty states, welcome screen
  11. Accessibility                 Keyboard nav, ARIA labels, focus management, screen reader

Phase 3 — Providers & Chat
  12. Provider & Model Cache        Model discovery, 24h TTL cache, provider service
  13. Offline & Network             Connectivity detection, retry queue, graceful degradation
  14. Chat Core                     Message rendering, markdown, syntax highlighting, virtual scrolling
  15. Chat Input                    Text area, attachments, drag & drop, file previews
  16. API Integration               DeepSeek + OpenCode Go + OpenCode Zen streaming
  17. Streaming Error Recovery      Partial message handling, retry/resume, interruption UI
  18. Context Window Management     Token counting (tiktoken-rs), auto-trim, context bar UI

Phase 4 — Management
  19. Message Management            Edit, delete (soft), pin conversations, bulk actions
  20. Search                        FTS5 full-text search, Cmd/Ctrl+K, search UI, filters
  21. Settings                      All settings pages (General, Providers, Startup, Shortcuts, Workspaces, Export, Models)
  22. Workspaces                    Directory management, auto-detection, file operations

Phase 5 — Advanced Features
  23. Skills                        skills.sh integration, install/uninstall, execution with workspace
  24. Web Fetching                  Secure HTTP, HTML→MD, GitHub integration, batch fetch
  25. Export                        Markdown/JSON/HTML/TXT export with options, save dialog

Phase 6 — Distribution
  26. Auto-Update                   tauri-plugin-updater, GitHub Releases manifest, update flow UI
  27. CI/CD                         GitHub Actions: lint → test → build → release matrix
  28. Shortcuts                     Global hotkeys (Cmd/Ctrl+Shift+P, Cmd/Ctrl+Shift+M)
  29. Polish                        Icons (all platforms), AGPL license headers, in-app license notice,
                                    final accessibility audit, performance profiling

Phase 7 — Ongoing
  30. Versioning                    SemVer tagging, conventional commits, changelog automation
```

---

## ⚠️ Gotchas & Pitfalls

### Hard Blockers

| #   | Gotcha                                                                                                                                                                | Mitigation                                                                          |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| 1   | **Tauri v2 schema is different from v1**. `tauri.conf.json` uses `app`/`build`/`bundle`/`plugins` top-level keys. Most tutorials online are v1 and will mislead.      | Use the [Tauri v2 docs](https://v2.tauri.app/) exclusively. Ignore v1 examples.     |
| 2   | **Capabilities are mandatory**. Without `capabilities/default.json`, plugins silently do nothing — no errors, just dead UI.                                           | Always create and verify `capabilities/default.json` before any plugin code.        |
| 3   | **SvelteKit + adapter-static**: Must set `fallback: 'index.html'` in adapter config AND `ssr: false` in `+layout.js`. Missing either → blank screen or 404 on reload. | Validate both configs are present before first build.                               |
| 4   | **FTS5 is optional in SQLite**. Some system-provided SQLite builds (especially older macOS) don't include FTS5. Search will be silently broken.                       | Use `libsqlite3-sys` with `bundled` feature in Cargo.toml to force a bundled build. |

### Dev/Platform

| #   | Gotcha                                                                                                                                                                                                                                        | Mitigation                                                                                                                           |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| 5   | **CSP must allow Tauri IPC in dev**. Add `connect-src 'self' ipc://localhost` to the CSP or all Tauri commands fail silently in dev.                                                                                                          | Include `ipc://localhost` in `connect-src` of CSP during development.                                                                |
| 6   | **`reqwest` + TLS on macOS**: Use `native-tls-vendored` feature to avoid system OpenSSL dependency issues. Note: reqwest 0.13 defaults to `rustls` (no system deps needed). Only use `native-tls-vendored` if you explicitly need native TLS. | Set `reqwest = { features = ["rustls", "json", "stream"] }` or `reqwest = { features = ["native-tls-vendored", "json", "stream"] }`. |
| 7   | **SQLite dev dependency**: `sqlx` needs `libsqlite3-dev` on Linux. On macOS it uses the system library. Missing on Linux → compile error.                                                                                                     | Add `libsqlite3-dev` (or `libsqlite3-devel`) to Linux build instructions.                                                            |
| 8   | **Tray icons are PNG everywhere**. Tauri v2 uses `.png` for tray icons on all platforms. Old `.icns`/`.ico` formats are v1 only. Provide 32x32 transparent PNG.                                                                               | Generate tray icon as 32x32 PNG in `src-tauri/icons/`.                                                                               |
| 9   | **Linux requires `libwebkit2gtk-4.1-dev`** (not 4.0). The v2 SDK bumped the WebKit requirement. Ubuntu 20.04 users must upgrade.                                                                                                              | Document: Ubuntu 22.04+ required, or install from backports.                                                                         |
| 10  | **Window state requires a plugin**. Tauri v2 doesn't auto-save window position/size. You need `tauri-plugin-window-state` explicitly. Not included by default.                                                                                | Add `tauri-plugin-window-state` to Cargo.toml and `@tauri-apps/plugin-window-state` to package.json.                                 |

### Runtime / Build

| #   | Gotcha                                                                                                                                                                                                                                                                                                                                                                                        | Mitigation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 11  | **Streaming events can drop**. Tauri events aren't a reliable message bus under heavy load. For LLM streaming, batch tokens in Rust and debounce frontend renders.                                                                                                                                                                                                                            | Use `tokio::sync::mpsc` channel for batching; debounce markdown render to 100ms.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 12  | **Plugin versions must match Tauri version**. All `tauri-plugin-*` crates must exactly match the `tauri` crate version. Mismatches → runtime panics.                                                                                                                                                                                                                                          | Pin all `tauri-*` deps to `=2.x.x` in Cargo.toml (not `^2.x.x`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 13  | **Tauri v2 uses `AppHandle` not `App`**. The v2 API passes `AppHandle` for most commands instead of the full `App`. Old v1 examples won't compile.                                                                                                                                                                                                                                            | Always use `AppHandle` in Tauri command functions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 14  | **macOS code signing required for plugins**: `tauri-plugin-updater` and `global-shortcut` may fail at runtime without code signing. Tauri auto-self-signs in dev only.                                                                                                                                                                                                                        | Test signed builds early in the dev cycle on macOS.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 15  | **`tauri.conf.json` `identifier` must match bundle ID**. Must be reverse-domain format (e.g., `com.clutch.app`). Wrong format → build error on macOS.                                                                                                                                                                                                                                         | Set `identifier: "com.clutch.app"` in `tauri.conf.json`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 16  | **Global shortcuts can fail on macOS**. System-owned shortcuts (e.g., Cmd+Shift+P for Page Setup) cause `with_shortcut()` to return `Err` and panic.                                                                                                                                                                                                                                          | Use `.and_then().unwrap_or_else()` chain with fallback shortcuts (e.g., Cmd+Alt+Shift+O). Never `.expect()` on shortcut registration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| 17  | **`tauri::async_runtime::block_on()` needed in setup**. `tokio::runtime::Handle::current()` panics because no Tokio runtime exists when the setup closure runs.                                                                                                                                                                                                                               | Use `tauri::async_runtime::block_on(db::init_pool(...))` instead of `handle.block_on(...)`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| 18  | **Updater plugin requires `pubkey` field**. The `UpdaterConfig` struct has `pubkey` as non-optional; missing it causes a deserialization error on startup.                                                                                                                                                                                                                                    | Always include a `pubkey` (dummy for dev) and `endpoints: []` in `plugins.updater` config.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 19  | **typesafe-i18n `setLocale()` must be called explicitly**. `initI18nSvelte()` creates the `LL` store with `getFallbackProxy()` — an empty-string proxy. The `translations` parameter is only consumed inside `setLocale(locale)`. Without calling `setLocale('en')` after `initI18nSvelte()`, every `{$LL.xxx()}` returns `""`. No errors, no warnings — just silently blank text everywhere. | Call `setLocale('en')` immediately after `initI18nSvelte()` in `i18n-svelte.ts`. Assign locale data directly (`loadedLocales.en = en`) before `setLocale()` to avoid ESM caching issues.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 20  | **Generated i18n files are overwritten**. `typesafe-i18n` generator overwrites `i18n-svelte.ts`, removing manual `import en from './en/index'` (default import), `loadedLocales.en = en`, and `setLocale('en')`. The Generator also rewrites `i18n-types.ts` from locale files — any keys added directly to types without corresponding locale entries will be lost.                          | Use `pnpm i18n` (not bare `typesafe-i18n`) — it chains `typesafe-i18n && bash scripts/fix-i18n.sh`. Never edit i18n-svelte.ts or i18n-types.ts by hand. Always add new keys to locale files FIRST, then regenerate. Run `npx vitest run src/__tests__/i18n.test.ts` after regeneration to verify nothing broke. The fix script applies: (1) `import en from './en/index'` — default import, NOT `import * as en` (namespace import wraps data in {default:...} breaking all lookups); (2) `loadedLocales.en = en as unknown as Translations`; (3) `setLocale('en')`. Idempotent — safe to run repeatedly. |
| 21  | **Stale DB data overrides new code defaults**. When you change a `$state(...)` default value in Svelte (e.g., shortcut key), any existing value in the `settings` SQLite table takes precedence because `onMount` loads from DB. Users never see the new default.                                                                                                                             | After changing a code default, DELETE stale rows from DB: `DELETE FROM settings WHERE key = 'old_key';`. Apply to both dev and production DBs. Always verify DB state after default changes.                                                                                                                                                                                                                                                                                                                                                                                                              |

### Maintenance

The Gotchas section is a living document. When you or an agent hits a new pitfall:

1. Add a row to the appropriate table
2. Include the mitigation (what prevents or fixes it)
3. Update this section before the next coding session

---

## 🌍 i18n Workflow

### Toolchain

| Script                     | Purpose                                                                  |
| -------------------------- | ------------------------------------------------------------------------ |
| `scripts/i18n-add-key.sh`  | Add one key to all 5 locales at once                                     |
| `scripts/i18n-add-lang.sh` | Bootstrap a new locale from en structure                                 |
| `scripts/i18n-validate.sh` | Generate types + fix adapter + cross-locale diff + runtime smoke + audit |
| `scripts/i18n-audit.sh`    | Scan code for untranslated strings                                       |
| `scripts/fix-i18n.sh`      | (Internal) Patch `i18n-svelte.ts` after generator runs                   |
| `pnpm i18n-validate`       | npm script: runs full validate pipeline                                  |
| `pnpm i18n`                | npm script: regenerate types + fix adapter                               |

### Adding keys to all locales

```bash
scripts/i18n-add-key.sh skills.verified "Verified" "Verificeret" "Verifiziert" "Zweryfikowano" "Vérifié"
```

If only the en value is provided, the other 4 locales get `[XX]` prefixed copies (safe — no blank text).

### Adding a new language

```bash
scripts/i18n-add-lang.sh es Spanish
```

Deep-copies en structure, prefixes all values with `[es]`. The app immediately shows `[es] Save` instead of blank text. Translate by removing the prefix.

### Validating before build

```bash
pnpm i18n-validate
```

This runs: `typesafe-i18n` → `fix-i18n.sh` → cross-locale key parity check → runtime smoke test → vitest i18n tests → audit scan. The `scripts/build.sh` script runs this automatically before building — a build with locale drift **refuses to build**.

### Golden rules

1. **Always add keys to all 5 locales simultaneously.** Use `i18n-add-key.sh` — never add to en alone.
2. **Run `pnpm i18n-validate` after any locale change.** Catches drift before it ships.
3. **Never edit `i18n-svelte.ts` or `i18n-types.ts` by hand.** The generator owns these files.
4. **Always use default import, never namespace import.** `import en from './en/index'` — NOT `import * as en`. Namespace import wraps default exports in `{default: {...}}`, breaking all lookups.
5. **If text goes blank:** Check that `i18n-svelte.ts` has: default import → `loadedLocales.en = en` → `setLocale('en')`. Run `pnpm i18n-validate` to auto-fix.

### Common failure modes

| Symptom               | Cause                                | Fix                                               |
| --------------------- | ------------------------------------ | ------------------------------------------------- |
| Blank text everywhere | `import * as en` (namespace import)  | Run `pnpm i18n-validate`                          |
| Blank text on one tab | Key missing in other locales         | `i18n-add-key.sh` or check `i18n-validate` output |
| LSP errors on `$LL.*` | `i18n-types.ts` stale after new keys | Run `pnpm i18n-validate` to regenerate            |
| Types don't match     | Edited `i18n-types.ts` by hand       | Run `pnpm i18n` — generator overwrites hand edits |

---

## 📋 Code Review & Improvement Backlog (v0.2.0)

Generated 2026-06-29. Severity: 🔴 Critical → 🟡 High → 🟢 Medium

### 🔴 Critical Security Fixes (before distribution)

| #   | File                              | Issue                                                      | Status                                                                     |
| --- | --------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1   | `skills.rs:432-448`               | RCE: execute_skill_action runs arbitrary sh -c commands    | ☑                                                                          |
| 2   | `web_fetcher.rs:68-89 vs 131-167` | DNS rebinding SSRF: IP check and HTTP use different DNS    | ☑                                                                          |
| 3   | `capabilities/default.json:24-27` | FS scope $HOME/** exposes SSH keys, credentials, passwords | ☑ (design — skills need full workspace access)                             |
| 4   | `tauri.conf.json:36`              | CSP allows script-src 'unsafe-inline'                      | ☑ (required — SvelteKit static adapter generates inline hydration scripts) |
| 5   | `api.rs:70, settings.rs`          | API keys stored plaintext in SQLite                        | ☑                                                                          |

### 🔴 Critical Code Bugs (v0.2.0 stability)

| #   | File                         | Issue                                                               | Status                                                                                                 |
| --- | ---------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| 6   | `chat/+page.svelte:134`      | handleSend() has no error boundary — failures are silent            | ☑                                                                                                      |
| 7   | `MarkdownRenderer.svelte:69` | Full markdown re-parse on every streaming token — debounce to 100ms | ☑                                                                                                      |
| 8   | `ProviderSelector.svelte:32` | Conflicting $effect resets selectedModel to hardcoded default       | ☑ (design — $effect correctly loads saved pref from DB; default is initial state before async resolve) |
| 9   | `NetworkStatus.svelte:14`    | let status is not $state — network dot never updates                | ☑                                                                                                      |
| 10  | `ChatMessage.svelte:50,61`   | saveEdit() and deleteMessage() never persist to DB — data loss      | ☑                                                                                                      |

### 🟡 High Priority

| #   | File                        | Issue                                                                     | Status |
| --- | --------------------------- | ------------------------------------------------------------------------- | ------ |
| 11  | `api.rs:169`                | Fake token counting: tokens_received += 1 per delta, not real tokens      | ☑      |
| 12  | `api.rs:12`                 | .http2_prior_knowledge() fails against servers without H2 prior knowledge | ☑      |
| 13  | `workspaces.rs:229,300`     | Path traversal bypass on canonicalize failure                             | ☑      |
| 14  | `model_cache.rs:328`        | refresh_models identical to get_models — returns stale cache              | ☑      |
| 15  | `web_fetcher.rs:232`        | batch_fetch sequential — should be concurrent via join_all                | ☑      |
| 16  | `search.rs` + migration 009 | FTS5 virtual table created but never populated/queried                    | ☑      |
| 17  | `context.rs:54,59`          | Token count failures silently return 0 — incorrect trimming               | ☑      |
| 19  | `skills.rs:391-410`         | write_file skips path check when file doesn't exist                       | ☑      |

### 🟢 Medium Priority

| #   | Area                   | Issue                                                           | Status                                                                                       |
| --- | ---------------------- | --------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| 20  | All async modules      | Replace std::fs blocking calls with tokio::fs (22 locations)    | ☑                                                                                            |
| 21  | AIAvatar/ChatMessage   | Extract duplicate provider color map to shared constant         | ☑                                                                                            |
| 22  | `chat.ts:256`          | Remove dead code: activeSession derived store                   | ☑                                                                                            |
| 23  | All .rs files          | Add Rustdoc comments to all public functions (62 fns, 13 files) | ☑                                                                                            |
| 24  | `export.rs:91-93`      | Remove or implement format_timestamp no-op function             | ☑                                                                                            |
| 25  | settings/+page.svelte  | Add aria-label to provider/model select elements                | ☑                                                                                            |
| 26  | WorkspaceSelector      | Add onkeydown handler to role="button" elements                 | ☑                                                                                            |
| 27  | `settings.rs:42-58`    | Mask API keys in get_all_settings response                      | ☑                                                                                            |
| 28  | `web_fetcher.rs:44-46` | Block IPv6 unique local addresses (fc00::/7)                    | ☑                                                                                            |
| 29  | All Tauri commands     | Add rate limiting to prevent DB exhaustion and API cost abuse   | ☑ (Dropped — single-user desktop app; existing queue cap + provider 429 handling sufficient) |

### Quick Wins (low effort, high impact)

| #   | Feature                                  | Effort   | Status                                                                            |
| --- | ---------------------------------------- | -------- | --------------------------------------------------------------------------------- |
| 30  | Fix 5 critical frontend bugs (#6-10)     | ~2 hours | ☑                                                                                 |
| 31  | Add console.debug to silent catch blocks | 15 min   | ☑                                                                                 |
| 32  | Debounce markdown rendering to 100ms     | 1 line   | ☑                                                                                 |
| 33  | Remove 'unsafe-inline' from CSP          | 1 line   | ☑ (Won't fix — required by SvelteKit static adapter for inline hydration scripts) |
| 34  | Replace window.eval() with navigate()    | 1 line   | ☑ (Won't fix — standard Tauri v2 SPA routing pattern)                             |
| 35  | Fix refresh_models to force-refresh      | ~5 lines | ☑                                                                                 |
| 36  | Add at least 5 unit tests (vitest)       | ~2 hours | ☑ (170 tests, 14 files)                                                           |
| 37  | Add 1 E2E test for send message flow     | ~1 hour  | ☑                                                                                 |

### 🔮 Deferred (blocked by Rust borrow checker — needs dedicated pass)

| #   | Feature           | Issue                                                                                                               | Status                                                                          |
| --- | ----------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| 38  | Shortcuts from DB | `app.global_shortcut()` + `sqlx::SqlitePool` borrow conflicts in setup closure. Needs `AppHandle` cloning refactor. | ☑                                                                               |
| 39  | Start minimized   | `app.get_webview_window()` borrow conflicts with `app.state::<SqlitePool>()`. Needs reorder or lazy init.           | ☑ (Already implemented via tauri.conf.json `visible: false` + conditional show) |
| 40  | Close to tray     | `window.on_window_event` callback requires `'static` lifetimes + thread-safe DB access. Needs `app_handle` clone.   | ☑ (Already implemented via CloseRequested handler)                              |

---

## 📋 Code Review & Improvement Backlog (v0.2.4)

Generated 2026-07-01. Severity: 🔴 Critical → 🟡 High → 🟢 Medium
Full-scope review: security, bugs, functionality gaps, test coverage, roadmap, quick wins.
Decisions locked: (2) Search — remove dead Cmd+K, do NOT rebuild FTS5. (3) Skills — build the missing UI. (4) Regenerate + virtual scrolling — in scope. (5) Retry queue (#59) — wire up.

### 🔴 Critical Security (re-opened holes — bypassable despite prior "fixes")

| #   | File                     | Issue                                                                                                                                                                                                                                                                        | Status                                                      |
| --- | ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| 41  | `skills.rs:457-513`      | RCE bypass: allowlist checks only first token, then runs full string via `sh -c`. `ls; rm -rf ~` bypasses all gates. Reject shell metachars `;&\|` `` ` `` `$><()` and/or drop `sh -c`, run program+args directly. Also blocking `std::process` in async → `tokio::process`. | ☑ (metacharacter guard + tokio::process + 11 tests)         |
| 42  | `web_fetcher.rs:135-176` | SSRF bypass: private-IP check only on original URL. `302 → 169.254.169.254` (cloud metadata) followed unchecked. Use `Policy::none()`, re-validate each redirect hop.                                                                                                        | ☑ (manual redirect loop + per-hop is_private_ip validation) |

### 🟡 High — Backend Bugs

| #   | File                 | Issue                                                                                                                                                              | Status |
| --- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ |
| 43  | `lib.rs:94`          | Startup panic: `.with_shortcut(...).unwrap()` violates Gotcha #16. Redundant (registration done safely at line 199). Remove it.                                    | ☑      |
| 44  | `api.rs:148`         | UTF-8 corruption: per-chunk `from_utf8_lossy` garbles multibyte chars (ø/ü/emoji/CJK) split across stream chunks. Buffer raw bytes, decode to last valid boundary. | ☑      |
| 45  | `web_fetcher.rs:187` | DoS: `resp.bytes()` buffers entire body before 10MB check. Check `content_length` early + stream with abort.                                                       | ☑      |
| 46  | `logs.rs:47-61`      | Arbitrary file read: non-empty `file` param used verbatim as path (reads `/etc/passwd`, `~/.ssh/id_rsa`). Whitelist against enumerated log files.                  | ☑      |

### 🔴 High — Frontend Bugs (user-facing breakage)

| #   | File                         | Issue                                                                                                                                                                                      | Status |
| --- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ |
| 47  | `MarkdownRenderer.svelte:56` | Copy-code button renders but no listener attached — feature entirely dead. Add onclick delegation on `.markdown` reading `data-block-id` from `codeBlockCache`.                            | ☑      |
| 48  | `api.ts:65`                  | Interrupted stream overwritten: unconditionally sets `complete` after invoke resolves, racing over `interrupted`. Guard with `wasInterrupted` flag.                                        | ☑      |
| 49  | `api.ts:22`                  | Assistant message stored with `sessionId: ''` — breaks store filters (e.g. retry). Pass `params.sessionId`.                                                                                | ☑      |
| 50  | `routes/+page.svelte:6`      | Stray artifact tag renders junk element on redirect page. Remove; move `goto('/chat')` into `onMount`.                                                                                     | ☑      |
| 51  | `chat/+page.svelte:288`      | Stop button cosmetic: sets status idle but backend stream keeps running + writes DB. Wire abort (Channel close/cancellation token). Updated to `tokio::sync::watch` + `StreamCancelState`. | ☑      |
| 52  | `chat/+page.svelte:216`      | `f.text()` called on binary/image attachments → garbage in prompt. Guard with `f.type.startsWith('text/')` / text-extension check.                                                         | ☑      |

### ⚠️ Functionality Gaps (built but not wired, or missing)

| #   | Feature                       | Status      | Action                                                                                                                                                                                  | Done |
| --- | ----------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| 53  | Search (Cmd+K)                | ❌ Broken   | **DECISION: Remove dead SearchBar + Cmd+K entirely.** `invoke('search')` unregistered, FTS5 dropped (mig 010). Remove `SearchBar.svelte`, Cmd+K handler, search i18n.                   | ☑    |
| 54  | Skills                        | ❌ No UI    | **DECISION: Build UI.** Backend + `skills.ts` complete but no component imports it. Add skills page/modal: browse installed, search skills.sh, install/uninstall, execute w/ workspace. | ☑    |
| 55  | Per-message edit/delete       | ⚠️ Reverted | Edit/delete buttons removed per user request — doesn't make sense on already-sent messages.                                                                                             | ☑    |
| 56  | Regenerate response           | ❌ Missing  | **In scope.** Only i18n strings exist. Add RefreshCw button on assistant messages → re-run from prior user message.                                                                     | ☑    |
| 57  | Context auto-trim             | ⚠️ Unwired  | `auto_trim_context` command never called from frontend. Wire into send path when context exceeds 90%.                                                                                   | ☑    |
| 58  | Auto-update UI                | ⚠️ Plumbing | Plugin + `updater.ts` exist; `updateStore` unused. Add startup check, "Update ready" prompt, Settings→General check-now + channel/auto-check.                                           | ☑    |
| 59  | Retry queue / offline         | ❌ Dead     | **DECISION: Wire up.** `enqueue` never called; `checkHealth` timer never started; "degraded" unreachable. Wire queue processor + periodic health check + offline message queueing.      | ☑    |
| 60  | Custom shortcut from DB (#38) | ❌ Dead     | `ShortcutRecorder` saves `shortcut_main` to DB; Rust uses hardcoded const, never reads it. Read setting at startup + re-register. (Prior Deferred #38.)                                 | ☑    |
| 61  | Virtual scrolling >50 msgs    | ❌ Missing  | **In scope.** Plain `{#each}`. Add virtual scrolling per AGENTS.md perf spec (buffer 3 above/below).                                                                                    | ☑    |

### 🧪 Test Coverage (target 80% lines / 90% branches)

| #   | Area                   | Issue                                                                                                                                                                                                     | Status                                                                       |
| --- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| 62  | Rust backend           | **0% coverage** (~3,800 LOC, zero `#[test]`). No `src-tauri/tests/`. Security fixes have no regression tests.                                                                                             | ☑ (72 tests: context 24, export 15, workspace 10, web_fetcher 12, skills 11) |
| 63  | Rust security tests    | Priority pure-fn tests: `is_private_ip`/`validate_url` (SSRF), `auto_trim`/`get_model_limit` (context), workspace path sandbox (traversal), skills command allowlist (RCE), export formatters (snapshot). | ☑ (SSRF + RCE done: 12 web_fetcher tests + 11 skills tests)                  |
| 64  | Frontend core services | `api.ts` (streamChat channel parsing), `chat.ts` (255 LOC store), `network.ts` all at 0%. Mock `@tauri-apps/api/core`.                                                                                    | ☑ (59 tests: api 11, chat-store 33, network 15)                              |
| 65  | Components             | 0 of 28 tested despite "every component" goal. Start with `ChatMessage` (edit/delete), `MarkdownRenderer` (DOMPurify XSS).                                                                                | ☑ (22 tests: ChatMessage 9, MarkdownRenderer 6, ProviderSelector 7)          |
| 66  | E2E flows              | Only 2 (app loads, send message). Add: provider switch, edit/delete/pin, export, settings.                                                                                                                | ☑ (8 new: settings 3, providers 3, export 2; 10 total)                       |

### 🟢 Medium / Quick Wins

| #   | File                                                                  | Issue                                                                                                                                | Status                                                       |
| --- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| 67  | `settings/+page.svelte:341`                                           | Build warning: `<label>` not associated with control. Convert to `<span>` heading.                                                   | ☑                                                            |
| 68  | `chat/+page.svelte:374`                                               | Build warning: resize-handle `div` mouse-only. Add `role="separator"`, `tabindex`, keydown, aria-label.                              | ☑                                                            |
| 69  | `ProviderSelector.svelte:100`                                         | Build warning: toggle button no accessible name. Add `role="switch"` + `aria-checked` + `aria-label`.                                | ☑                                                            |
| 70  | `StreamingIndicator.svelte:22`                                        | Build warning: self-closing `<span/>`. Use `<span></span>`.                                                                          | ☑                                                            |
| 71  | Multiple (Avatar/AIAvatar/ProviderSelector/FilePreview)               | Unused CSS warnings: `.avatar__icon`/`.ai-avatar__icon`/`.spin`/`.spinner` style child-component class props. Use `:global()`.       | ☑                                                            |
| 72  | `settings:107`, `shortcuts.ts:15`                                     | Malformed default `CmdOrCtrl+Ctrl+K` (double modifier). Fixed to `CmdOrCtrl+Shift+M`.                                                | ☑                                                            |
| 73  | ExportDialog/ThemeSelector/NetworkStatus                              | i18n labels computed once at init with `get(LL)` — won't re-translate on locale switch. Make `$derived` referencing `$LL.…()`.       | ☑                                                            |
| 74  | Toast/ErrorBanner/EmptyState/Icon                                     | `<svelte:component>` deprecated in Svelte 5. Render capitalized variable directly.                                                   | ☑ (Deferred — `@const` syntax not supported in this runtime) |
| 75  | `network.ts`, `skills.rs:519`, `directories.ts`, `cache-scheduler.ts` | Dead code: health-check, always-empty skill-id query, dangling `get_home_dir`/`get_cache_status` commands. Remove.                   | ☑                                                            |
| 76  | new-chat logic ×3                                                     | Duplicated in `chat/+page.svelte:350`, `SessionSidebar.svelte:146`, `routes/new/+page.svelte`. Extract `createNewSession()` service. | ☑                                                            |
| 77  | `model_cache.rs:357`                                                  | `refresh_models` doesn't bump cache `version` (hardcodes '1'). Inconsistent with `get_or_refresh_models`.                            | ☑                                                            |
| 78  | settings.rs/skills.rs                                                 | Fresh `reqwest::Client::new()` per call — no connection pooling. Share a `OnceLock` client like `api.rs:15`.                         | ☑                                                            |
| 79  | `SearchBar.svelte:151`                                                | Moot once #53 removes SearchBar. (`@html` of unsanitized snippet bypasses DOMPurify.)                                                | ☑                                                            |

### Cross-references to prior backlog

- #43 relates to Gotcha #16 (shortcut unwrap panic)
- #60 = prior Deferred #38 (shortcuts from DB) — now in scope
- #72 relates to Gotcha #21 (stale DB defaults)
- #53 supersedes any FTS5 search work — search is being removed, not fixed

### 🌊 Execution Ordering (six waves)

Sequenced by risk, dependency, and blast radius.

**Wave 1 — Critical Security → v0.2.4** (ship fast)

- #41 RCE, #42 SSRF, plus #63 (partial) — write regression tests _as_ each is fixed so the fix is locked immediately.

**Wave 2 — High Backend Bugs → v0.2.5**

- #43 shortcut panic (crash risk, one-line), #44 UTF-8 corruption (all non-EN locales), #45 fetch DoS, #46 log file read.
- If wiring #51's abort, add the cancellation Channel/token plumbing here.

**Wave 3 — High Frontend Bugs → v0.2.5**

- #50 stray tag (trivial) → #48 interrupted race → #49 empty sessionId → #47 dead copy button → #52 binary attachment guard → #51 Stop-button abort (heaviest; consumes Wave 2 plumbing).

**Wave 4 — Cleanup & Removals → v0.2.5**

- #53 remove Search (also erases #79) → #75 remove dead code → #76 extract `createNewSession()`. Delete before building new.

**Wave 5 — Functionality → v0.3.0** (feature release)

- Easy-wire first: #55 edit/delete → #56 regenerate (reuses #55 button row) → #57 auto-trim → #60 custom shortcut → #58 auto-update UI → #59 retry queue → then net-new: #54 Skills UI (biggest build), #61 virtual scrolling (self-contained).

**Wave 6 — Tests + Quick Wins → v0.3.1** (hardening)

- #62/#64/#65/#66 coverage (establish `src-tauri/tests/` harness first) + #63 remainder → #67–#74, #77, #78 polish.

**Cross-wave dependencies:**

- #51 (Stop) needs backend cancellation token — add in Wave 2, consume in Wave 3.
- #53 (remove Search) must precede Wave 6 so no tests are written for deleted code.
- #55 → #56 (regenerate reuses edit/delete action row).
- #63 straddles Wave 1 (security regression tests) and Wave 6 (remainder).

**Version cadence:** Wave 1 → v0.2.4 · Waves 2–4 → v0.2.5 · Wave 5 → v0.3.0 · Wave 6 → v0.3.1

---

## 🖼️ Brand Guidelines

| Element             | Value                 |
| ------------------- | --------------------- |
| **Name**            | Clutch                |
| **Tagline**         | Grab the conversation |
| **Primary Color**   | #6366f1 (Indigo)      |
| **Secondary Color** | #8b5cf6 (Purple)      |
| **Accent Color**    | #f43f5e (Rose)        |
| **Font**            | JetBrains Mono        |
| **Icon**            | Asteroid (Lucide)     |
| **Tray Icon**       | Simplified asteroid   |

---

## 🔮 Feature: File Reference Middleware (Option C)

**Status**: Planned — not yet implemented

### Concept

Before the user's message hits the API, scan it for file path references. If found, read the file and inject its contents into the message. The model never calls tools — it just sees file content as if the user pasted it.

### Detection patterns

| Pattern               | Example                  | Match                             |
| --------------------- | ------------------------ | --------------------------------- |
| Tilde path            | `~/Downloads/report.md`  | Reads `$HOME/Downloads/report.md` |
| Absolute path         | `/Users/mojoaar/foo.txt` | Reads if within allowed dirs      |
| Relative to workspace | `src/main.rs`            | Resolves against active workspace |
| Natural language      | "read the file X"        | Fuzzy: extract apparent path      |

### Security sandbox — only read from these locations

| Allowed                    | Blocked                           |
| -------------------------- | --------------------------------- |
| Active workspace directory | `~/.ssh/`, `~/.aws/`, `~/.gnupg/` |
| `~/Downloads/`             | `/etc/`, `/var/`, system dirs     |
| `~/Desktop/`               | Hidden files (`~/.` prefix)       |
| `~/Documents/`             | Path traversal (`../`)            |

### Implementation

**Rust side** (`src-tauri/src/file_resolver.rs` — new):

1. `detect_file_references(text: &str) -> Vec<String>` — regex scan for path patterns
2. `resolve_and_read(path: &str, workspace: Option<&str>) -> Result<String, String>` — resolve tilde/relative paths, security check, read file
3. `inject_file_contents(message: &str, workspace: Option<&str>) -> String` — main entry point: detect → read → inject

**API integration** (`api.rs:stream_chat`):

```rust
let enhanced_content = file_resolver::inject_file_contents(
    &last_message.content,
    active_workspace.as_deref()
);
```

**Frontend** (`api.ts` / `chat/+page.svelte`):

- Show "Reading file..." toast when file references detected
- Pass active workspace path to `sendMessage`

### Injection format

```
[File: ~/Downloads/report.md]
<file contents here>
---
User message: read the new chat file in my downloads folder?
```

### Edge cases

| Case                          | Behavior                                                      |
| ----------------------------- | ------------------------------------------------------------- |
| File not found                | `[File ~/Downloads/foo.txt not found]` appended               |
| File too large (>1MB)         | First 500 lines + `[...truncated at 500 lines]`               |
| Binary file                   | `[~/Downloads/img.png is a binary file — skipped]`            |
| Path outside sandbox          | `[Path /etc/passwd is outside allowed directories — blocked]` |
| Tilde with no HOME            | `[Could not resolve ~ — no home directory]`                   |
| Multiple files in one message | All injected, separated                                       |

### Files to create/modify

| File                             | What                                                 |
| -------------------------------- | ---------------------------------------------------- |
| `src-tauri/src/file_resolver.rs` | New — detection, resolution, security, injection     |
| `src-tauri/src/api.rs`           | Call `inject_file_contents` before building messages |
| `src-tauri/src/lib.rs`           | Register `mod file_resolver`                         |
| `src/lib/services/api.ts`        | Pass `activeWorkspace` to `sendMessage`              |
| `src/routes/chat/+page.svelte`   | Read active workspace, pass to send                  |

### Effort

~2 hours. Pure Rust on the backend, minor frontend plumbing. Zero API changes — just file reading + string manipulation.

---

## 🔮 Feature: Slash Commands

**Status**: Planned — not yet implemented

### Concept

Users type `/` in the chat input to trigger a command palette. Commands are processed client-side (UI actions, settings changes, navigation) and can also be injected into the AI context (model-aware commands). An autocomplete dropdown appears as the user types.

### Dual execution model

| Type            | Processing                                                                          | Examples                                                   |
| --------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| **Client-side** | Command is intercepted and executed before message hits API                         | `/theme dracula`, `/export`, `/model`, `/provider`         |
| **AI-aware**    | Command and its resolved data are injected into the message context sent to the LLM | `/read src/main.rs`, `/fetch https://...`, `/skill golang` |

Commands that modify UI state (`/theme`, `/model`, `/provider`) are client-side only. Commands that gather data (`/read`, `/fetch`, `/skill`, `/ls`) modify the message content before it reaches the API.

### Command list

| Command                 | Type        | Action                                                                                                                                                                   |
| ----------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/model <name>`         | Client-side | Switch the active model for the current session                                                                                                                          |
| `/provider <id>`        | Client-side | Switch the active provider for the current session                                                                                                                       |
| `/workspace`            | Client-side | Show the active workspace path (banner or toast)                                                                                                                         |
| `/ls [path]`            | AI-aware    | List files in workspace directory. Injects file listing into message context. Supports multiple configured workspaces — relative paths resolve against active workspace. |
| `/read <file>`          | AI-aware    | Read file from workspace and inject contents into message context                                                                                                        |
| `/add-workspace <path>` | Client-side | Add a new workspace directory from the given path                                                                                                                        |
| `/fetch <url>`          | AI-aware    | Fetch webpage content and inject as markdown into message context                                                                                                        |
| `/github <repo>`        | AI-aware    | Fetch a GitHub repository's README and inject into message context                                                                                                       |
| `/skills`               | Client-side | List installed skills in chat (banner or system message)                                                                                                                 |
| `/skill <name>`         | AI-aware    | Load skill instructions and inject into system prompt for the current request                                                                                            |
| `/search-skills <q>`    | Client-side | Search skills.sh and display results in chat                                                                                                                             |
| `/theme <name>`         | Client-side | Switch the active theme immediately                                                                                                                                      |

### Autocomplete UI

- **Trigger**: Typing `/` in the chat input opens a command palette overlay above the input
- **Filtering**: Commands filter in real-time as the user types
- **Categories**: Grouped by type (Chat, Workspace, Web, Skills, App)
- **Keyboard**: Arrow keys navigate, Enter selects, Escape dismisses
- **Parameter hints**: Selected command shows placeholder text for arguments (e.g., `/model <model-name>`)

### Message injection format (AI-aware commands)

```
/system
Command output from /read src/main.rs:
<file contents>

Command output from /fetch https://example.com:
<page content>
---
User message: can you review this?
```

### Files to create/modify

| File                                     | Change                                                            |
| ---------------------------------------- | ----------------------------------------------------------------- |
| `src/lib/components/SlashCommand.svelte` | New — command palette overlay with autocomplete                   |
| `src/lib/components/ChatInput.svelte`    | Add `/` detection, command parsing, autocomplete integration      |
| `src/lib/services/commands.ts`           | New — command registry, parser, executor                          |
| `src/routes/chat/+page.svelte`           | Wire command actions to stores/settings                           |
| `src-tauri/src/`                         | Any new backend commands needed (e.g., `/ls` for multi-workspace) |

### Effort

~4-6 hours. New Svelte component + command service + ChatInput integration + wiring.
