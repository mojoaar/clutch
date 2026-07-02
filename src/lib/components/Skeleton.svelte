<script lang="ts">
  interface Props {
    width?: string;
    height?: string;
    radius?: string;
    count?: number;
    gap?: string;
  }

  let { width = '100%', height = '16px', radius = '6px', count = 1, gap = '8px' }: Props = $props();

  let items = Array.from({ length: count });
</script>

<div class="skeleton-group" style="--gap: {gap}">
  {#each items as _}
    <div
      class="skeleton"
      style="width: {width}; height: {height}; border-radius: {radius}"
    />
  {/each}
</div>

<style>
  .skeleton-group {
    display: flex;
    flex-direction: column;
    gap: var(--gap);
  }

  .skeleton {
    background: var(--surface-hover);
    animation: skeleton-pulse 1.6s ease-in-out infinite;
  }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 0.8; }
  }
</style>
