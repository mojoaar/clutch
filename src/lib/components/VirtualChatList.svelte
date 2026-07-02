<script lang="ts">
  import type { Message } from '$lib/stores/chat';
  import { chatStore } from '$lib/stores/chat';
  import ChatMessage from './ChatMessage.svelte';
  import StreamingIndicator from './StreamingIndicator.svelte';
  import StreamingErrorBanner from './StreamingErrorBanner.svelte';
  import LL from '$lib/i18n/i18n-svelte';
  import { tick } from 'svelte';

  interface Props {
    messages: Message[];
    streamingMessageId: string | null;
    lastInterruptedId?: string;
    onRegenerate?: (messageId: string) => void;
  }

  let {
    messages,
    streamingMessageId,
    lastInterruptedId = '',
    onRegenerate,
  }: Props = $props();

  const THRESHOLD = 50;
  const BUFFER = 3;
  const ESTIMATE_SIZE = 120;

  let scrollContainer = $state<HTMLDivElement>();
  let scrollTop = $state(0);
  let containerHeight = $state(0);
  let measuredHeights = $state(new Map<string, number>());
  let spacerTopHeight = $state(0);
  let spacerBottomHeight = $state(0);
  let startIndex = $state(0);
  let endIndex = $state(0);
  let shouldAutoScroll = $state(true);
  let scheduledScrollId: number | null = null;

  function getHeight(index: number): number {
    const msg = messages[index];
    if (!msg) return 0;
    return measuredHeights.get(msg.id) ?? ESTIMATE_SIZE;
  }

  function computeVisibleRange() {
    if (!scrollContainer || messages.length <= THRESHOLD) {
      startIndex = 0;
      endIndex = messages.length;
      spacerTopHeight = 0;
      spacerBottomHeight = 0;
      return;
    }

    let acc = 0;
    let start = 0;
    for (let i = 0; i < messages.length; i++) {
      const h = getHeight(i);
      if (acc + h >= scrollTop) {
        start = i;
        break;
      }
      acc += h;
    }
    start = Math.max(0, start - BUFFER);

    let end = start;
    let visAcc = 0;
    for (let i = start; i < messages.length; i++) {
      const h = getHeight(i);
      visAcc += h;
      end = i + 1;
      if (visAcc >= containerHeight + BUFFER * ESTIMATE_SIZE) break;
    }
    end = Math.min(messages.length, end + BUFFER);

    if (streamingMessageId) {
      const streamingIdx = messages.findIndex((m) => m.id === streamingMessageId);
      if (streamingIdx >= 0 && (streamingIdx < start || streamingIdx >= end)) {
        end = Math.min(messages.length, streamingIdx + 1);
        start = Math.min(streamingIdx, start);
      }
    }

    let topH = 0;
    for (let i = 0; i < start; i++) topH += getHeight(i);

    let bottomH = 0;
    for (let i = end; i < messages.length; i++) bottomH += getHeight(i);

    startIndex = start;
    endIndex = end;
    spacerTopHeight = topH;
    spacerBottomHeight = bottomH;
  }

  $effect(() => {
    scrollTop;
    containerHeight;
    messages;
    void tick().then(() => computeVisibleRange());
  });

  function scrollToBottom(behavior: 'instant' | 'smooth' = 'instant') {
    const el = scrollContainer;
    if (!el) return;
    if (scheduledScrollId !== null) cancelAnimationFrame(scheduledScrollId);
    scheduledScrollId = requestAnimationFrame(() => {
      scheduledScrollId = null;
      requestAnimationFrame(() => {
        el.scrollTo({ top: el.scrollHeight, behavior });
      });
    });
  }

  $effect(() => {
    const len = messages.length;
    if (len > 0 && shouldAutoScroll) {
      void tick().then(() => scrollToBottom());
    }
  });

  function handleScroll() {
    const el = scrollContainer;
    if (!el) return;
    scrollTop = el.scrollTop;
    containerHeight = el.clientHeight;
    const atBottom = el.scrollHeight - el.scrollTop - el.clientHeight < el.clientHeight;
    shouldAutoScroll = atBottom;
  }

  $effect(() => {
    const el = scrollContainer;
    if (!el) return;
    const ro = new ResizeObserver(() => {
      containerHeight = el.clientHeight;
    });
    ro.observe(el);
    return () => ro.disconnect();
  });

  let visibleSlice = $derived(
    messages.length <= THRESHOLD ? messages : messages.slice(startIndex, endIndex),
  );

  const measureOnResize = (node: HTMLElement, messageId: string) => {
    if (messages.length <= THRESHOLD) return { destroy() {} };
    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const h = entry.contentRect.height;
        if (h > 0) {
          measuredHeights.set(messageId, h);
          measuredHeights = measuredHeights;
          void tick().then(() => computeVisibleRange());
        }
      }
    });
    ro.observe(node);
    queueMicrotask(() => {
      const h = node.getBoundingClientRect().height;
      if (h > 0) {
        measuredHeights.set(messageId, h);
        measuredHeights = measuredHeights;
      }
    });
    return { destroy() { ro.disconnect(); } };
  };
</script>

<div
  class="chat__messages"
  role="log"
  aria-label={$LL.aria.chatMessages()}
  bind:this={scrollContainer}
  onscroll={handleScroll}
>
  {#if messages.length > THRESHOLD}
    <div style="height: {spacerTopHeight}px; flex-shrink: 0;"></div>
  {/if}

  {#each visibleSlice as message (message.id)}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      class="virtual-message-wrapper"
      use:measureOnResize={message.id}
    >
      <ChatMessage {message} isStreaming={message.id === streamingMessageId} {onRegenerate} />
    </div>
  {/each}

  {#if messages.length > THRESHOLD}
    <div style="height: {spacerBottomHeight}px; flex-shrink: 0;"></div>
  {/if}

  {#if $chatStore.streamingStatus === 'streaming'}
    <StreamingIndicator />
  {/if}

  {#if $chatStore.streamingStatus === 'interrupted' && lastInterruptedId}
    <StreamingErrorBanner
      messageId={lastInterruptedId}
      sessionId={$chatStore.activeSessionId ?? ''}
      provider=""
      model=""
    />
  {/if}
</div>

<style>
  .chat__messages {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
    contain: layout style;
  }

  .virtual-message-wrapper {
    contain: layout style;
  }
</style>
