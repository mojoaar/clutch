<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { devMode } from '$lib/stores/dev';
  import { goto } from '$app/navigation';

  interface LogFile { name: string; path: string; size: number; }

  let files = $state<LogFile[]>([]);
  let lines = $state<string[]>([]);
  let selectedFile = $state('');
  let filterLevel = $state('');
  let autoScroll = $state(true);
  let container = $state<HTMLDivElement>();
  let isLoading = $state(false);

  $effect(() => {
    if ($devMode) {
      loadLogs();
    }
  });

  async function loadLogs(file?: string) {
    isLoading = true;
    try {
      const result = await invoke<any>('get_logs', { file: file || selectedFile || '' });
      files = result.files;
      lines = result.lines;
      if (files.length > 0 && !selectedFile) {
        selectedFile = files[0].path;
      }
      applyFilter();
      if (autoScroll && container) {
        requestAnimationFrame(() => container?.scrollTo(0, container.scrollHeight));
      }
    } catch (e) {
      lines = [`Error loading logs: ${e}`];
    }
    isLoading = false;
  }

  function applyFilter() {
    if (autoScroll && container) {
      requestAnimationFrame(() => container?.scrollTo(0, container.scrollHeight));
    }
  }

  function levelClass(line: string): string {
    if (line.includes('"ERROR"') || line.includes('"level":"ERROR"')) return 'log-line--error';
    if (line.includes('"WARN"') || line.includes('"level":"WARN"')) return 'log-line--warn';
    return '';
  }

  const filteredLines = $derived(
    filterLevel
      ? lines.filter((l) => l.includes(`\"${filterLevel.toUpperCase()}\"`))
      : lines,
  );

  function formatLine(line: string): string {
    try {
      const parsed = JSON.parse(line);
      const ts = parsed.timestamp ? new Date(parsed.timestamp).toLocaleTimeString() : '';
      const level = (parsed.level || '').toUpperCase();
      const msg = parsed.fields?.message || parsed.message || '';
      return `${ts} [${level}] ${msg}`;
    } catch {
      return line;
    }
  }

</script>

{#if !$devMode}
  <div class="log-gate">
    <p>Developer Mode is disabled.</p>
    <button onclick={() => goto('/settings')}>Enable in Settings</button>
  </div>
{:else}
  <div class="log-viewer">
    <div class="log-header">
      <div class="log-controls">
        <select bind:value={selectedFile} onchange={() => loadLogs(selectedFile)}>
          {#each files as f}
            <option value={f.path}>{f.name}</option>
          {/each}
        </select>
        <select bind:value={filterLevel}>
          <option value="">All levels</option>
          <option value="ERROR">ERROR</option>
          <option value="WARN">WARN</option>
          <option value="INFO">INFO</option>
          <option value="DEBUG">DEBUG</option>
        </select>
        <button onclick={() => loadLogs()}>Refresh</button>
      </div>
      <label class="auto-scroll-label">
        <input type="checkbox" bind:checked={autoScroll} /> Auto-scroll
      </label>
    </div>
    <div class="log-lines" bind:this={container}>
      {#if isLoading}
        <div class="log-empty">Loading...</div>
      {:else if filteredLines.length === 0}
        <div class="log-empty">No log entries found</div>
      {:else}
        {#each filteredLines as line}
          <div class="log-line {levelClass(line)}">{formatLine(line)}</div>
        {/each}
      {/if}
    </div>
  </div>
{/if}

<style>
  .log-gate {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
    color: var(--text-dim);
    font-size: 14px;
  }
  .log-gate button {
    background: var(--primary);
    color: white;
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
  }
  .log-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg);
    color: var(--text);
    font-family: 'JetBrains Mono', monospace;
    padding: 0;
  }
  .log-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .log-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .log-controls select,
  .log-controls button {
    background: var(--bg-dim);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
  }
  .auto-scroll-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-dim);
  }
  .log-lines {
    flex: 1;
    overflow-y: auto;
    font-size: 12px;
    line-height: 1.6;
    padding: 8px 16px;
  }
  .log-line {
    white-space: pre-wrap;
    word-break: break-all;
  }
  .log-line--error {
    color: var(--danger, #ef4444);
  }
  .log-line--warn {
    color: var(--warning, #f59e0b);
  }
  .log-empty {
    color: var(--text-dim);
    font-style: italic;
    padding: 16px;
  }
</style>
