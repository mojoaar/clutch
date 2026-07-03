# Changelog

All notable changes to Clutch will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-07-03

### Added

- 5 new languages: Swedish (sv), Norwegian (no), Spanish (es), Portuguese (pt), Italian (it)
- All ~294 i18n keys fully translated for each new locale
- Language name keys added to all existing locales for the selector dropdown

## [0.4.1] - 2026-07-03

### Added

- Dynamic option menus for `/model`, `/provider`, and `/theme` — select from available choices instead of typing manually
- Slash command palette now scrolls into view when navigating with arrow keys
- Full i18n coverage for slash commands: 20 keys across 5 locales (descriptions, categories, feedback, errors, aria)

### Changed

- Slash command descriptions use `descriptionKey` field with i18n lookup (not hardcoded strings)

### Fixed

- Scroll-into-view: slash command palette now follows keyboard navigation

## [0.4.0] - 2026-07-03

### Added

- File Reference Middleware: detect file paths in messages (~/path, /abs/path, ./relative) and inject contents into API context automatically
- Security sandbox for file reading: blocks .ssh/, .gnupg/, .aws/, /etc/, /var/, path traversal
- Slash Commands: type `/` to trigger command palette with 12 commands (theme, model, provider, workspace, add-workspace, skills, search-skills, read, ls, fetch, github, skill)
- Slash command autocomplete overlay with keyboard navigation (Arrow keys, Tab, Enter, Escape)
- Client-side commands execute instantly (/theme, /model, /provider, /workspace, /add-workspace)
- AI-aware commands inject resolved content into message context (/read, /ls, /fetch, /github, /skill)
- resolve_and_read_file Tauri command for slash command file reading

## [0.3.3] - 2026-07-03

### Fixed

- Token counting: messages now persistently store real token counts (chatStore.setMessageTokens + finalTokens capture + user message estimate via content.length/4)
- ContextBar now shows session total tokens instead of per-response streaming count
- Web fetch auto-trim: context window trimming now accounts for web-fetched content injected after trimming
- Streaming indicator shows real BPE token count from Rust tiktoken-rs (replaced fake random counter)

## [0.3.2] - 2026-07-02

### Added

- 15 Anthropic skills curated into catalog (DOCX, PDF, PPTX, XLSX, Internal Comms, Doc Coauthoring, Frontend Design, Canvas Design, Brand Guidelines, Theme Factory, MCP Builder, Webapp Testing, Web Artifacts Builder, Skill Creator, Algorithmic Art)
- Skills update mechanism — check GitHub for newer SKILL.md versions (per-source commit SHA / release tag comparison)
- Per-skill update badges and "Update" button on Skills page
- 6 new i18n keys across all 5 locales for update UI

### Changed

- Skills catalog restructured: 25 curated skills (10 Superpowers + 15 Anthropic), grouped in 6 categories
- Skills page redesigned with Installed/Browse tabs and client-side curated catalog search
- Site landing page skills section updated from skills.sh to curated catalog
- Docs skills page rewritten for bundled, offline-first model

### Removed

- Dead skills.sh API code (~200 lines Rust: search_skills, trending_skills)
- Dead frontend searchSkills and trendingSkills services
- Dead SkillInfo struct (Rust + TypeScript)
- 4 agent-only Superpowers from catalog (dispatching-parallel-agents, executing-plans, subagent-driven-development, using-superpowers)
- skills.sh references from site, docs, and README

### Fixed

- Empty-branch 404 in skill update (empty-string guard on branch fallback)
- has_update logic — skills without stored version now correctly flag as needing update
- update_skill persists branch setting to database
- get_skill_setting returns proper Err for missing DB keys

## [0.3.1] - 2026-07-02

### Added

- 47 new Rust unit tests (72 total): context, export, workspace traversal
- 59 new Vitest service tests (170 total): chat-store, api, network
- 22 new Vitest component tests: ChatMessage, MarkdownRenderer, ProviderSelector
- 8 new Playwright E2E tests (10 total): settings, providers, export

### Changed

- Replaced `std::fs` with `tokio::fs` in 22 sites across 5 Rust files
- Added Rustdoc comments to all 62 public functions in 13 files
- Shared `reqwest::Client` via `OnceLock` instead of fresh per-call
- Bump model cache version on refresh (was hardcoded to '1')

### Fixed

- 7 hardcoded aria-label/title strings replaced with i18n references
- Build warnings: label association, non-interactive element, unused CSS
- Reactive i18n labels (ExportDialog, ThemeSelector, NetworkStatus)
- `settingsTabs.skills` missing from da/de/pl/fr locales

### Security

- 19 untranslated strings fixed — audit clean, zero hardcoded UI text

## [0.3.0] - 2026-07-01

### Added

- Skills UI page with installed + browse tabs, client-side curated catalog
- Regenerate response button on assistant messages
- Auto-update settings tab with check-now, channel, auto-check toggles
- Update-available banner in app layout
- Retry queue processor with offline message queueing + queue badge
- Custom global shortcut from DB settings (overrides hardcoded fallback)
- VirtualChatList component for sessions with >50 messages
- Superpowers skills integration (14 skills from obra/superpowers)

### Changed

- Fallback system prompt: anti-hallucination language (no tool calls)
- Curated skills catalog: verified GitHub repos only, skills.sh API removed
- Stop button now aborts backend stream via `tokio::sync::watch`

### Fixed

- Dead copy-code button (click delegation on markdown container)
- Interrupted stream status overwritten on invoke resolve
- Empty sessionId on assistant message store entry
- Stray artifact tag + premature goto on redirect page
- Binary attachment crash (MIME guard before f.text())
- UTF-8 corruption on multi-byte chars split across stream chunks
- Content-Length early check + streaming size abort (DoS prevention)
- Settings tab link `CmdOrCtrl+Shift+M` → `CmdOrCtrl+Ctrl+K` (correct shortcut)

### Removed

- Dead SearchBar component, directories.ts, cache-scheduler.ts
- Search i18n keys from all 5 locales (Cmd+K search)
- Duplicated new-chat logic (extracted to createNewSession service)

### Security

- RCE fix: shell metacharacter rejection in skills execute_skill_action
- SSRF fix: per-hop redirect IP validation in web fetcher

## [0.2.5] - 2026-07-01

### Security

- Blocked arbitrary file read via log file path validation (basename lookup against enumerated files)

### Fixed

- Removed redundant `.with_shortcut().unwrap()` that could panic on startup
- UTF-8 stream corruption — multi-byte characters (ø, ü, emoji, CJK) no longer garbled when split across SSE chunks
- DoS vulnerability — web fetcher now checks `Content-Length` before download and streams body with abort on size limit
- Copy-code button in markdown code blocks now works (click delegation + clipboard API)
- Interrupted stream status no longer overwritten with `complete` on invoke resolve
- Assistant messages now store correct `sessionId` (was empty string)
- Stray artifact tag removed from redirect page; `goto` moved into `onMount`
- Binary/image attachments no longer inject garbage into prompts (MIME guard)
- Stop button now aborts backend stream via `tokio::sync::watch` cancellation token
- Cleaned up unused `DialogExt` import warning

## [0.2.4] - 2026-07-01

### Security

- RCE prevention — shell metacharacter guard (`$(`, `` ` ``, `;`, `|`, `&&`, `||`, `>`, `<`) in skill `run_command`
- SSRF prevention — per-hop private-IP validation on HTTP redirects in web fetcher
- Switched `std::process::Command` to `tokio::process::Command` (non-blocking)

### Added

- 23 Rust regression tests (11 RCE metacharacter tests, 12 SSRF/validation tests)

## [0.2.3] - 2026-06-30

### Added

- Default system prompt when no session-specific prompt is set (prevents models defaulting to Chinese)
- 82 Vitest unit tests across 7 files (time-utils, file-utils, errors, models, providers, themes)
- E2E test for send message chat flow (Playwright)
- Gotcha #20 in AGENTS.md: never modify applied SQLite migrations

### Changed

- All 22 blocking `std::fs` calls replaced with `tokio::fs` (db, logs, lib, workspaces, skills)

### Fixed

- Startup crash when a previously-applied migration is modified (restored migration 009, added migration 010)

### Documentation

- 62 public Rust functions now have Rustdoc comments across 13 files

## [0.2.2] - 2026-06-30

### Added

- German (de), Polish (pl), French (fr) translations — 5 languages total, ~450 translation keys
- Full i18n wiring — all 105+ hardcoded English strings now use translation system
- Landing page (`site/`) — rewritten with hero gradient, stats bar, providers/skills/FAQ sections
- Skills docs page (`site/docs/skills.html`)
- Provider balance status now translated (available/unavailable)

### Changed

- AI name unified to "Clutch" across all 5 languages (no more "Assistant"/"Assistent"/etc.)
- Settings layout redesigned — 5% horizontal padding, save button right-aligned
- Context menu "Rename" action uses dedicated i18n key instead of generic "Edit"
- Version display in settings is now dynamic (reads from `tauri.conf.json` at runtime)

### Fixed

- Rust streaming: removed `.http2_prior_knowledge()` for stable HTTP/2 ALPN negotiation
- Rust streaming: drain remaining SSE buffer on stream end and chunk errors (mid-word truncation fix)
- Rust streaming: per-chunk 30s read timeout added (stall detection)
- Rust streaming: empty provider responses now return proper errors instead of blank messages
- Rust streaming: non-"stop" finish_reasons (content_filter, length) now handled instead of silent ignore
- Danish translations: all æ/ø/å special characters fixed (28 values, previously ASCII fallback)
- Startup: `start_minimized` no longer flashes the window (main window set to `visible: false`)
- Landing page download binary updated from `0.2.0` to `0.2.2`

### Removed

- Quick Ask shortcut (dead — never registered by backend)
- Show Popup shortcut (dead — popup window removed)
- Popup-related shortcut references in docs/settings.html
- Old `0.2.0` binary from site downloads

## [0.2.1] - 2026-06-30

### Added

- Developer mode toggle (right-click → Inspect Element), works immediately via Tauri command
- Start on boot via `tauri-plugin-autostart` — toggle in Settings → Startup
- Close to tray instead of quitting — respects Settings → Startup toggle
- Start minimized to tray — respects Settings → Startup toggle
- Saved shortcuts actually apply at app startup (previously always used hardcoded defaults)
- Skills command permissions (3 toggles in Settings → Permissions, all off by default)
- `bump-version.sh` script for syncing version across tauri.conf.json, Cargo.toml, package.json

### Changed

- Refresh models button now actually forces fresh API fetch (previously returned stale cache)
- Session dates now compare calendar days instead of timestamps ("Today"/"Yesterday" correct after midnight)
- Models tab reordered to follow Providers in settings sidebar
- Workspaces section now has descriptive explanation text
- Developer mode toggle has explanatory description

### Fixed

- API key masking reverted (was breaking DeepSeek auth by returning truncated keys)
- skills `run_command` restored with command whitelist and permission-level enforcement
- FS scope reverted to `$HOME/**` after over-restriction broke file reading
- Markdown debounce reverted (was preventing message rendering until navigation)
- CSP `script-src 'unsafe-inline'` restored for Vite dev mode compatibility
- `handleSend` error boundary: system error message shown on send failure
- `NetworkStatus` dot now reactive (`$state` wrapper added)
- Message edit/delete now persisted to SQLite
- Conflicting `$effect` deleted in ProviderSelector (no more model reset on settings load)
- DNS rebinding SSRF fixed with `.resolve()` IP pinning in web fetcher

### Removed

- Dead `countTokens` function in ContextBar.svelte
- Dead `activeSession` derived store in chat.ts

### Security

- Command whitelist for skills `run_command` (dangerous commands behind permission toggles)
- Sensitive path blocklist (SSH keys, GPG keys, AWS credentials, Keychains) blocked from file reading
- DNS rebinding protection via IP pinning in web fetcher
- `get_all_settings` no longer masks API keys (masking was causing auth failures)

## [0.2.0] - 2026-06-29

### Added

- Multi-provider chat support (DeepSeek, OpenCode Go, OpenCode Zen)
- Web fetch injection into LLM context (URL detection → HTML→MD conversion)
- Local file system read (directories and files) injected as LLM context
- File attachment reading via drag-and-drop (text/code files)
- User profile management (display name, avatar icon/image upload)
- Light/dark theme toggle in topbar with DB persistence
- Resizable sidebar (drag handle, width persisted to DB)
- Export conversations (Markdown, JSON, HTML, Plain Text)
- Copy conversation to clipboard from context menu
- Pulsing connection status dot
- "AI is thinking…" streaming indicator
- Syntax highlighting in code blocks (highlight.js theme)
- App icon (Lucide Astroid gradient circle)
- Build script (`scripts/build.sh`) for production deployment
- Windows and Linux drag support (resize handle)

### Changed

- Welcome screen merged into chat page (dead-space replacement)
- Default Provider moved to Providers tab in Settings
- Network status now provider-aware (yellow "No API key" when unconfigured)
- Settings auto-save: API keys, model selections, toggle state
- Avatar types simplified to Icon and Image only
- Settings persistence: all settings loaded from DB on mount

### Fixed

- i18n: translations now load correctly via `setLocale('en')` in i18n-svelte.ts
- Model persistence: default model respected across provider switches
- 401 console noise replaced with `navigator.onLine` health check
- Tray icon transparency (rsvg-convert instead of qlmanage)
- Global shortcuts: macOS fallback to avoid system-owned hotkeys
- Updater plugin: require `pubkey` field in config
- Tokio runtime in Tauri setup: use `tauri::async_runtime::block_on()`

### Architecture

- Static reqwest HTTP/2 client with connection pooling for faster LLM responses
- Dynamic context limits per model (1M DeepSeek, 200K Claude, etc.)
- Model categorization via prefix inference (MiniMax, Kimi, GLM, Claude, GPT, etc.)
- Model name formatting with dots between version numbers only
- FS scope: `$HOME/**` + `$APPDATA/**` for cross-platform file access
- Message persistence to SQLite with FK validation

### Security

- FS scope limited to `$HOME` + `$APPDATA`
- Private IP blocking in web fetcher
- API keys stored in SQLite, never logged
- CSP: `style-src 'self' 'unsafe-inline'` for Svelte scoped styles

## [0.1.0] - 2026-06-29

### Added

- Initial project scaffold (Tauri v2 + SvelteKit + SQLite)
- System tray icon with show/hide/quit menu
- Global shortcuts (Cmd+Shift+P for popup, Cmd+Shift+M for main)
- Dark/light theme system (Clutch, Nord, Dracula, Cyberpunk)
- i18n framework (English + Danish, ~90 translation keys)
- Database schema with 9 migrations (sessions, messages, settings, user_profile, model_cache, FTS5 search)
- Streaming chat API integration (DeepSeek + OpenCode endpoints)
- Context window management with auto-trim
- Message rendering with Markdown + syntax highlighting
- Chat input with drag-and-drop file attachments
- Session sidebar with pin, archive, delete, bulk actions
- FTS5 full-text search (Cmd/Ctrl+K)
- Settings pages (General, Providers, Startup, Shortcuts, Workspaces, Export, Models)
- Workspace management with auto-detection
- Skills integration (skills.sh API)
- Web fetching (secure HTTP, HTML→MD, GitHub integration)
- Export (Markdown/JSON/HTML/Text)
- Auto-update framework (tauri-plugin-updater)
- CI/CD workflow (GitHub Actions, 3-platform build matrix)
