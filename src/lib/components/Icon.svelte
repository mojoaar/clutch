<script lang="ts">
  import type { Component } from 'svelte';

  interface Props {
    icon: Component;
    size?: number;
    class?: string;
  }

  let { icon, size = 16, class: className }: Props = $props();
</script>

<svelte:component this={icon} strokeWidth={1.5} size={size} class={className} />
