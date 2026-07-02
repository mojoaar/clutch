<script lang="ts">
  import { chatStore, type StreamingStatus } from '$lib/stores/chat';
  import { filterFiles } from '$lib/utils/file-utils';
  import FilePreview from './FilePreview.svelte';
  import DropZone from './DropZone.svelte';
  import { v4 as uuid } from 'uuid';
  import { Send, Paperclip, Square } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    sessionId: string;
    provider?: string;
    model?: string;
    disabled?: boolean;
    onSend?: (content: string, files: File[]) => void;
    onStop?: () => void;
  }

  let { sessionId, provider, model, disabled = false, onSend, onStop }: Props = $props();

  let inputValue = $state('');
  let attachedFiles = $state<File[]>([]);
  let isDragOver = $state(false);
  let dragCounter = $state(0);
  let textareaEl = $state<HTMLTextAreaElement | null>(null);

  let isStreaming = $derived($chatStore.streamingStatus === 'streaming');
  let canSend = $derived((inputValue.trim().length > 0 || attachedFiles.length > 0) && !isStreaming && !disabled);
  let calculatedRows = $derived(Math.min(Math.max(inputValue.split('\n').length, 1), 8));

  function handleSend() {
    if (!canSend) return;
    const content = inputValue.trim();
    if (content || attachedFiles.length > 0) {
      onSend?.(content, [...attachedFiles]);
    }
    inputValue = '';
    attachedFiles = [];
    textareaEl?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  function handleInput() {
    if (textareaEl) {
      textareaEl.style.height = 'auto';
      textareaEl.style.height = Math.min(textareaEl.scrollHeight, 200) + 'px';
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
