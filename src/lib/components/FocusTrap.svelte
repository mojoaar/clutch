<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    enabled?: boolean;
    children?: import('svelte').Snippet;
  }

  let { enabled = true, children }: Props = $props();

  let container: HTMLElement;
  let previousFocus: HTMLElement | null = null;

  const FOCUSABLE = 'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])';

  $effect(() => {
    if (enabled) {
      previousFocus = document.activeElement as HTMLElement;
      trapFocus();
    }
    return () => {
      if (previousFocus && enabled) {
        previousFocus.focus();
      }
    };
  });

  function trapFocus() {
    if (!container) return;
    const focusable = container.querySelectorAll<HTMLElement>(FOCUSABLE);
    if (focusable.length === 0) return;

    const first = focusable[0];
    const last = focusable[focusable.length - 1];

    first.focus();

    function handler(e: KeyboardEvent) {
      if (e.key !== 'Tab') return;
      if (e.shiftKey) {
        if (document.activeElement === first) {
          e.preventDefault();
          last.focus();
        }
      } else {
        if (document.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
    }

    container.addEventListener('keydown', handler);
    return () => container.removeEventListener('keydown', handler);
  }
</script>

<div class="focus-trap" bind:this={container}>
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .focus-trap { display: contents; }
</style>
