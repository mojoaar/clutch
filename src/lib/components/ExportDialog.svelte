<script lang="ts">
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    format: string;
    includeMetadata: boolean;
    includeTimestamps: boolean;
    includeProviderInfo: boolean;
    onFormatChange: (format: string) => void;
    onMetadataChange: (value: boolean) => void;
    onTimestampsChange: (value: boolean) => void;
    onProviderInfoChange: (value: boolean) => void;
  }

  let {
    format,
    includeMetadata,
    includeTimestamps,
    includeProviderInfo,
    onFormatChange,
    onMetadataChange,
    onTimestampsChange,
    onProviderInfoChange,
  }: Props = $props();

  let formats = $derived([
    { value: 'markdown', label: $LL.exportFormats.markdown() },
    { value: 'json', label: $LL.exportFormats.json() },
    { value: 'html', label: $LL.exportFormats.html() },
    { value: 'text', label: $LL.exportFormats.text() },
  ]);
</script>

<div class="export-dialog">
  <div class="field">
      <label class="field-label" for="export-format">{$LL.exportSettings.format()}</label>
    <select
      class="field-select"
      value={format}
      oninput={(e) => onFormatChange(e.currentTarget.value)}
    >
      {#each formats as f}
        <option value={f.value}>{f.label}</option>
      {/each}
    </select>
  </div>

  <div class="checks">
    <label class="check">
      <input
        type="checkbox"
        checked={includeMetadata}
        oninput={(e) => onMetadataChange(e.currentTarget.checked)}
      />
      <span>{$LL.exportSettings.includeMetadata()}</span>
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={includeTimestamps}
        oninput={(e) => onTimestampsChange(e.currentTarget.checked)}
      />
      <span>{$LL.exportSettings.includeTimestamps()}</span>
    </label>
    <label class="check">
      <input
        type="checkbox"
        checked={includeProviderInfo}
        oninput={(e) => onProviderInfoChange(e.currentTarget.checked)}
      />
      <span>{$LL.exportSettings.includeProviderInfo()}</span>
    </label>
  </div>
</div>

<style>
  .export-dialog {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .field-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .field-select {
    padding: 7px 10px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--bg);
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    outline: none;
    cursor: pointer;
    appearance: none;
  }

  .field-select:focus {
    border-color: var(--primary);
  }

  .checks {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .check {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--text);
    cursor: pointer;
  }

  .check input[type="checkbox"] {
    width: 15px;
    height: 15px;
    accent-color: var(--primary);
    cursor: pointer;
  }
</style>
