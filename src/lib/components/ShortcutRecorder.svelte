<script lang="ts">
  import { browser } from '$app/environment';
  import LL from '$lib/i18n/i18n-svelte';
  import { formatShortcutForDisplay } from '$lib/utils/shortcut-utils';

  interface Props {
    action: string;
    value: string;
    onChange: (value: string) => void;
  }

  let { action, value, onChange }: Props = $props();

  let recording = $state(false);
  let keysDown = $state<Set<string>>(new Set());
  let displayValue = $state(value);
  let formattedDisplay = $derived(formatShortcutForDisplay(displayValue));

  $effect(() => {
    displayValue = value;
  });

  const modifierMap: Record<string, string> = {
    Control: 'Ctrl',
    Meta: 'Cmd',
    Alt: 'Alt',
    Shift: 'Shift',
  };

  function formatShortcut(keys: string[]): string {
    const mods = keys.filter((k) => k in modifierMap).map((k) => modifierMap[k]);
    const rest = keys.filter((k) => !(k in modifierMap));
    return [...mods.sort(), ...rest].join('+');
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    keysDown.add(e.key);
    displayValue = formatShortcut([...keysDown]);
  }

  function handleKeyUp(e: KeyboardEvent) {
    if (!recording) return;

    const final = [...keysDown];
    if (final.length > 0) {
      const shortcut = formatShortcut(final);
      displayValue = shortcut;
      onChange(shortcut);
    }

    keysDown.clear();
    recording = false;
  }

  function startRecording() {
    recording = true;
    displayValue = '...';
    keysDown.clear();
  }

  $effect(() => {
    if (browser && recording) {
      window.addEventListener('keydown', handleKeyDown, true);
      window.addEventListener('keyup', handleKeyUp, true);
      return () => {
        window.removeEventListener('keydown', handleKeyDown, true);
        window.removeEventListener('keyup', handleKeyUp, true);
        keysDown.clear();
      };
    }
  });
</script>

<div class="shortcut-recorder">
  <span class="shortcut-action">{$LL.shortcuts.shortcut({ action })}</span>
  <button class="shortcut-input" onclick={startRecording} class:shortcut-input--recording={recording}>
    {#if recording}
      <span class="recording-dot"></span>
      {formattedDisplay}
    {:else}
      {formattedDisplay || $LL.clickToRecord()}
    {/if}
  </button>
</div>

<style>
  .shortcut-recorder {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 0;
  }

  .shortcut-recorder + .shortcut-recorder {
    border-top: 1px solid var(--border);
  }

  .shortcut-action {
    font-size: 13px;
    color: var(--text);
  }

  .shortcut-input {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    min-width: 140px;
    padding: 6px 12px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--bg);
    color: var(--text-dim);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
    text-align: center;
    justify-content: center;
  }

  .shortcut-input:hover {
    border-color: var(--text-dim);
    color: var(--text);
  }

  .shortcut-input--recording {
    border-color: var(--primary);
    color: var(--primary);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--primary) 20%, transparent);
  }

  .recording-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--error);
    animation: pulse 1s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }
</style>
