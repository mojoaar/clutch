<script lang="ts">
  import { chatStore } from '$lib/stores/chat';
  import { filterFiles } from '$lib/utils/file-utils';
  import FilePreview from './FilePreview.svelte';
  import DropZone from './DropZone.svelte';
  import SlashCommand from './SlashCommand.svelte';
  import { COMMANDS, parseCommand, filterCommands, type CommandDef } from '$lib/services/commands';
  import { Send, Paperclip, Square, Terminal } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface SlashOption {
    label: string;
    value: string;
  }

  interface Props {
    sessionId: string;
    provider?: string;
    model?: string;
    disabled?: boolean;
    onSend?: (content: string, files: File[]) => void;
    onStop?: () => void;
    onSlashSend?: (commandId: string, args: string[], feedbackText: string) => void;
    getSlashOptions?: (commandId: string) => SlashOption[] | null;
  }

  let {
    sessionId,
    provider,
    model,
    disabled = false,
    onSend,
    onStop,
    onSlashSend,
    getSlashOptions,
  }: Props = $props();

  let inputValue = $state('');
  let attachedFiles = $state<File[]>([]);
  let isDragOver = $state(false);
  let dragCounter = $state(0);
  let textareaEl = $state<HTMLTextAreaElement | null>(null);
  let showSlashPalette = $state(false);
  let activeSlashIndex = $state(0);
  let slashOptionMode = $state(false);
  let slashOptionCommand = $state<CommandDef | null>(null);
  let slashOptions = $state<SlashOption[]>([]);
  let slashOptionsTitle = $state('');
  let slashOptionIndex = $state(0);

  let isStreaming = $derived($chatStore.streamingStatus === 'streaming');
  let canSend = $derived((inputValue.trim().length > 0 || attachedFiles.length > 0) && !isStreaming && !disabled);
  let calculatedRows = $derived(Math.min(Math.max(inputValue.split('\n').length, 1), 8));

  let slashQuery = $derived(inputValue.startsWith('/') ? inputValue.slice(1) : '');
  let filteredSlashCommands = $derived.by(() => {
    if (slashQuery.length === 0) return COMMANDS.slice(0, 10);
    return filterCommands('/' + slashQuery, 12);
  });
  let selectedSlashCommand = $derived(filteredSlashCommands[activeSlashIndex] ?? null);

  $effect(() => {
    const becameSlash = inputValue.length === 1 && inputValue === '/';
    if (becameSlash) {
      showSlashPalette = true;
      activeSlashIndex = 0;
    } else if (!inputValue.startsWith('/')) {
      showSlashPalette = false;
    }
  });

  $effect(() => {
    if (showSlashPalette && activeSlashIndex >= filteredSlashCommands.length) {
      activeSlashIndex = Math.max(0, filteredSlashCommands.length - 1);
    }
  });

  function handleSend() {
    if (isStreaming || disabled) return;

    const content = inputValue.trim();
    if (content || attachedFiles.length > 0) {
      onSend?.(content, [...attachedFiles]);
    } else {
      return;
    }
    inputValue = '';
    attachedFiles = [];
    resetSlash();
    textareaEl?.focus();
  }

  function resetSlash() {
    showSlashPalette = false;
    slashOptionMode = false;
    slashOptionCommand = null;
    slashOptions = [];
    activeSlashIndex = 0;
    slashOptionIndex = 0;
  }

  function executeSlashCommand() {
    if (!selectedSlashCommand) return;
    const cmd = selectedSlashCommand;

    if (getSlashOptions) {
      try {
        const opts = getSlashOptions(cmd.id);
        if (opts && opts.length > 0) {
          slashOptionMode = true;
          slashOptionCommand = cmd;
          slashOptions = opts;
          slashOptionsTitle = cmd.label;
          slashOptionIndex = 0;
          return;
        }
      } catch {
        // fall through to normal execution
      }
    }

    const parts = inputValue.trim().split(/\s+/);
    const args = parts.slice(1);
    const feedbackText = args.length > 0
      ? `${cmd.label} ${args.join(' ')}`
      : cmd.label;

    onSlashSend?.(cmd.id, args, feedbackText);
    inputValue = '';
    attachedFiles = [];
    resetSlash();
    textareaEl?.focus();
  }

  function executeSlashOption() {
    const option = slashOptions[slashOptionIndex];
    if (!option || !slashOptionCommand) return;

    onSlashSend?.(slashOptionCommand.id, [option.value], `${slashOptionCommand.label} ${option.label}`);
    inputValue = '';
    attachedFiles = [];
    resetSlash();
    textareaEl?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (slashOptionMode) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        slashOptionIndex = Math.min(slashOptionIndex + 1, slashOptions.length - 1);
        return;
      }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        slashOptionIndex = Math.max(slashOptionIndex - 1, 0);
        return;
      }
      if (e.key === 'Escape') {
        e.preventDefault();
        slashOptionMode = false;
        return;
      }
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        executeSlashOption();
        return;
      }
    } else if (showSlashPalette) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        activeSlashIndex = Math.min(activeSlashIndex + 1, filteredSlashCommands.length - 1);
        return;
      }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        activeSlashIndex = Math.max(activeSlashIndex - 1, 0);
        return;
      }
      if (e.key === 'Escape') {
        e.preventDefault();
        showSlashPalette = false;
        return;
      }
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        executeSlashCommand();
        return;
      }
      if (e.key === 'Tab') {
        e.preventDefault();
        if (selectedSlashCommand) {
          inputValue = selectedSlashCommand.label + ' ';
          activeSlashIndex = 0;
        }
        return;
      }
    } else {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSend();
      }
    }
  }

  function handleInput() {
    if (textareaEl) {
      textareaEl.style.height = 'auto';
      textareaEl.style.height = Math.min(textareaEl.scrollHeight, 200) + 'px';
    }
    if (!inputValue.startsWith('/')) {
      showSlashPalette = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    dragCounter = 0;

    if (e.dataTransfer?.files) {
      const files = filterFiles(e.dataTransfer.files);
      attachedFiles = [...attachedFiles, ...files];
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer!.dropEffect = 'copy';
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragCounter++;
    isDragOver = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    dragCounter--;
    if (dragCounter <= 0) {
      isDragOver = false;
      dragCounter = 0;
    }
  }

  function handlePaste(e: ClipboardEvent) {
    const items = e.clipboardData?.files;
    if (items && items.length > 0) {
      e.preventDefault();
      const files = filterFiles(items);
      attachedFiles = [...attachedFiles, ...files];
    }
  }

  function handleFileSelect() {
    const input = document.createElement('input');
    input.type = 'file';
    input.multiple = true;
    input.accept = 'image/*,.txt,.md,.json,.js,.ts,.py,.rs,.go,.rb,.java,.c,.cpp,.html,.css,.scss,.yaml,.yml,.xml,.sql,.sh,.toml,.svelte,.vue,.csv,.log,.env';
    input.onchange = () => {
      if (input.files) {
        const files = filterFiles(input.files);
        attachedFiles = [...attachedFiles, ...files];
      }
    };
    input.click();
  }

  function removeFile(index: number) {
    attachedFiles = attachedFiles.filter((_, i) => i !== index);
  }

  function handleStop() {
    onStop?.();
  }
</script>

<div
  class="chat-input"
  class:chat-input--drag={isDragOver}
  role="region"
  aria-label={$LL.aria.chatInput()}
  ondragover={handleDragOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <DropZone active={isDragOver} />

  {#if showSlashPalette}
    <SlashCommand
      commands={filteredSlashCommands}
      activeIndex={slashOptionMode ? slashOptionIndex : activeSlashIndex}
      query={slashQuery}
      showOptions={slashOptionMode}
      options={slashOptions}
      optionsTitle={slashOptionMode ? slashOptionsTitle : ''}
    />
  {/if}

  {#if attachedFiles.length > 0}
    <div class="chat-input__files">
      {#each attachedFiles as file, i (file.name + file.size + i)}
        <FilePreview
          name={file.name}
          size={file.size}
          type={file.type}
          onRemove={() => removeFile(i)}
        />
      {/each}
    </div>
  {/if}

  <div class="chat-input__row">
    <textarea
      bind:this={textareaEl}
      class="chat-input__textarea"
      bind:value={inputValue}
      rows={calculatedRows}
      placeholder={$LL.chat.typeMessage()}
      aria-label={$LL.aria.chatMessageInput()}
      onkeydown={handleKeydown}
      oninput={handleInput}
      onpaste={handlePaste}
      disabled={disabled}
    ></textarea>

    <div class="chat-input__actions">
      <button
        class="chat-input__btn"
        onclick={handleFileSelect}
        aria-label={$LL.aria.attachFiles()}
        title={$LL.aria.attachFiles()}
      >
        <Paperclip size={18} strokeWidth={1.5} />
      </button>

      {#if isStreaming}
        <button
          class="chat-input__btn chat-input__btn--stop"
          onclick={handleStop}
          aria-label={$LL.aria.stopGenerating()}
          title={$LL.aria.stopGenerating()}
        >
          <Square size={14} strokeWidth={2} />
        </button>
      {:else}
        <button
          class="chat-input__btn chat-input__btn--send"
          class:chat-input__btn--disabled={!canSend}
          onclick={handleSend}
          aria-label={$LL.aria.sendMessage()}
          title={$LL.aria.sendMessage()}
          disabled={!canSend}
        >
          <Send size={18} strokeWidth={1.5} />
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .chat-input {
    position: relative;
    padding: 12px 16px 16px;
    border-top: 1px solid var(--border);
    background: var(--bg);
  }

  .chat-input__files {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 10px;
    max-height: 180px;
    overflow-y: auto;
  }

  .chat-input__row {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--surface);
    transition: border-color 0.15s;
  }

  .chat-input__row:focus-within {
    border-color: var(--primary);
  }

  .chat-input--drag .chat-input__row {
    border-color: var(--primary);
  }

  .chat-input__textarea {
    flex: 1;
    border: none;
    outline: none;
    resize: none;
    font-family: inherit;
    font-size: 14px;
    line-height: 1.6;
    color: var(--text);
    background: transparent;
    max-height: 200px;
    min-height: 24px;
  }

  .chat-input__textarea::placeholder {
    color: var(--text-dim);
  }

  .chat-input__actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .chat-input__btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .chat-input__btn:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .chat-input__btn--send {
    background: var(--primary);
    color: white;
  }

  .chat-input__btn--send:hover {
    background: var(--primary-hover);
    color: white;
  }

  .chat-input__btn--disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .chat-input__btn--disabled:hover {
    background: var(--primary);
    color: white;
  }

  .chat-input__btn--stop {
    background: var(--accent);
    color: white;
  }

  .chat-input__btn--stop:hover {
    background: var(--accent);
    opacity: 0.9;
  }
</style>
