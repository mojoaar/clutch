<script lang="ts">
  import { themeName, themeMode } from '$lib/stores/theme';
  import { themes, type ThemeName, type ThemeMode } from '$lib/themes';
  import { Check } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  const themeNames: ThemeName[] = ['catppuccin', 'clutch', 'cyberpunk', 'dracula', 'github', 'monokai', 'nord', 'tokyo-night'];
  let modeOptions = $derived<{ value: ThemeMode; label: string }[]>([
    { value: 'system', label: $LL.theme.system() as string },
    { value: 'light', label: $LL.theme.light() as string },
    { value: 'dark', label: $LL.theme.dark() as string },
  ]);
</script>

<div class="theme-selector">
  <div class="setting-group">
      <label class="setting-label" for="theme-name-group">{$LL.theme.theme()}</label>
    <div class="theme-grid">
      {#each themeNames as name}
        <button
          class="theme-card"
          class:theme-card--active={$themeName === name}
          onclick={() => ($themeName = name)}
          aria-label={name}
          title={name}
        >
          <div class="theme-card__preview" style:background={themes[name].dark.colors['--color-bg']}>
            <div class="theme-card__swatch" style:background={themes[name].dark.colors['--color-primary']}></div>
            <div class="theme-card__swatch" style:background={themes[name].dark.colors['--color-accent']}></div>
          </div>
          <span class="theme-card__name">{name[0].toUpperCase() + name.slice(1)}</span>
          {#if $themeName === name}
            <Check size={14} strokeWidth={2.5} class="theme-card__check" />
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <div class="setting-group">
      <label class="setting-label" for="theme-mode-group">{$LL.theme.mode()}</label>
    <div class="mode-tabs">
      {#each modeOptions as opt}
        <button
          class="mode-tab"
          class:mode-tab--active={$themeMode === opt.value}
          onclick={() => ($themeMode = opt.value)}
        >
          {opt.label}
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .theme-selector {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .setting-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .theme-card {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 10px;
    border: 1.5px solid var(--border);
    border-radius: 10px;
    background: var(--surface);
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
    color: var(--text);
  }

  .theme-card:hover {
    border-color: var(--text-dim);
  }

  .theme-card--active {
    border-color: var(--primary);
  }

  .theme-card__preview {
    width: 100%;
    height: 32px;
    border-radius: 6px;
    display: flex;
    gap: 4px;
    padding: 6px;
    box-sizing: border-box;
  }

  .theme-card__swatch {
    width: 14px;
    height: 14px;
    border-radius: 3px;
  }

  .theme-card__name {
    font-size: 12px;
    font-weight: 500;
  }

  .theme-card__check {
    position: absolute;
    top: 6px;
    right: 6px;
    color: var(--primary);
  }

  .mode-tabs {
    display: flex;
    gap: 4px;
    background: var(--surface);
    border-radius: 8px;
    padding: 3px;
  }

  .mode-tab {
    flex: 1;
    padding: 6px 12px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-dim);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
  }

  .mode-tab:hover {
    color: var(--text);
  }

  .mode-tab--active {
    background: var(--bg);
    color: var(--text);
    font-weight: 600;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }
</style>
