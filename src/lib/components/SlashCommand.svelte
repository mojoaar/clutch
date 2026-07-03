<script lang="ts">
  import { tick } from 'svelte';
  import type { CommandDef, CommandCategory } from '$lib/services/commands';
  import LL from '$lib/i18n/i18n-svelte';

  interface SlashOption {
    label: string;
    value: string;
  }

  interface Props {
    commands: CommandDef[];
    activeIndex: number;
    query: string;
    showOptions?: boolean;
    options?: SlashOption[];
    optionsTitle?: string;
  }

  let {
    commands,
    activeIndex,
    query,
    showOptions = false,
    options = [],
    optionsTitle = '',
  }: Props = $props();

  const categoryOrder: CommandCategory[] = ['chat', 'workspace', 'web', 'skills', 'app'];

  function categoryLabel(cat: CommandCategory): string {
    const labels: Record<CommandCategory, () => string> = {
      chat: $LL.slashCommands.categories.chat,
      workspace: $LL.slashCommands.categories.workspace,
      web: $LL.slashCommands.categories.web,
      skills: $LL.slashCommands.categories.skills,
      app: $LL.slashCommands.categories.app,
    };
    return labels[cat]();
  }

  function cmdDescription(cmd: CommandDef): string {
    const descs = $LL.slashCommands.descriptions as Record<string, () => string>;
    return descs[cmd.descriptionKey]?.() ?? cmd.descriptionKey;
  }

  let grouped = $derived.by(() => {
    const groups: Record<string, CommandDef[]> = {};
    for (const cmd of commands) {
      const cat = cmd.category;
      if (!groups[cat]) groups[cat] = [];
      groups[cat].push(cmd);
    }
    return categoryOrder
      .filter((cat) => groups[cat] && groups[cat].length > 0)
      .map((cat) => ({ category: cat, label: categoryLabel(cat), commands: groups[cat] }));
  });

  $effect(() => {
    const idx = activeIndex;
    if (idx >= 0) {
      tick().then(() => {
        const el = document.querySelector('.slash-command-item--active');
        el?.scrollIntoView({ block: 'nearest' });
      });
    }
  });
</script>

<div class="slash-command-overlay" role="listbox" aria-label={$LL.slashCommands.aria.commandPalette()}>
  {#if showOptions}
    <div class="slash-command-group">
      <div class="slash-command-group__header">{optionsTitle}</div>
      {#each options as opt, i}
        <button
          class="slash-command-item"
          class:slash-command-item--active={i === activeIndex}
          role="option"
          aria-selected={i === activeIndex}
          tabindex={-1}
        >
          <span class="slash-command-item__label">{opt.label}</span>
        </button>
      {/each}
    </div>
  {:else}
    {#each grouped as group}
      <div class="slash-command-group">
        <div class="slash-command-group__header">{group.label}</div>
        {#each group.commands as cmd, idx}
          {@const globalIdx = commands.indexOf(cmd)}
          <button
            class="slash-command-item"
            class:slash-command-item--active={globalIdx === activeIndex}
            role="option"
            aria-selected={globalIdx === activeIndex}
            tabindex={-1}
          >
            <span class="slash-command-item__label">{cmd.label}</span>
            {#if cmd.args && cmd.args.length > 0}
              <span class="slash-command-item__args">
                {cmd.args.map((a) => `<${a.placeholder}>`).join(' ')}
              </span>
            {:else}
              <span class="slash-command-item__args"></span>
            {/if}
            <span class="slash-command-item__desc">{cmdDescription(cmd)}</span>
          </button>
        {/each}
      </div>
    {/each}
  {/if}
</div>

<style>
  .slash-command-overlay {
    position: absolute;
    bottom: calc(100% + 4px);
    left: 12px;
    right: 12px;
    max-height: 280px;
    overflow-y: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
    z-index: 100;
    padding: 6px;
  }

  .slash-command-group__header {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dim);
    padding: 6px 10px 2px;
  }

  .slash-command-item {
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: auto auto;
    gap: 0 8px;
    align-items: baseline;
    width: 100%;
    text-align: left;
    padding: 6px 10px;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    transition: background 0.1s;
  }

  .slash-command-item--active {
    background: var(--primary);
    color: white;
  }

  .slash-command-item--active .slash-command-item__args,
  .slash-command-item--active .slash-command-item__desc {
    color: rgba(255, 255, 255, 0.7);
  }

  .slash-command-item__label {
    font-weight: 600;
    font-family: 'JetBrains Mono', monospace;
    font-size: 12px;
  }

  .slash-command-item__args {
    font-family: 'JetBrains Mono', monospace;
    font-size: 11px;
    color: var(--text-dim);
  }

  .slash-command-item__desc {
    grid-column: 1 / -1;
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
