<script lang="ts">
  import { UserCircle } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';

  interface Props {
    name?: string;
    imageUrl?: string;
    emoji?: string;
    color?: string;
    size?: number;
  }

  let { name, imageUrl, emoji, color = 'var(--primary)', size = 32 }: Props = $props();

  let initials = $derived(
    name
      ? name
          .split(' ')
          .map((n) => n[0])
          .join('')
          .toUpperCase()
          .slice(0, 2)
      : '',
  );
</script>

<div class="avatar" style="width: {size}px; height: {size}px; font-size: {Math.floor(size / 2.2)}px; background: {color}">
  {#if imageUrl}
    <img class="avatar__img" src={imageUrl} alt={name ?? $LL.aria.userAvatar()} />
  {:else if emoji}
    <span class="avatar__emoji">{emoji}</span>
  {:else if initials}
    <span class="avatar__initials">{initials}</span>
  {:else}
    <UserCircle class="avatar__icon" size={size} strokeWidth={1.5} />
  {/if}
</div>

<style>
  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--primary);
    color: white;
    flex-shrink: 0;
    overflow: hidden;
    font-weight: 600;
  }

  .avatar__img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar__initials {
    line-height: 1;
  }

  .avatar__emoji {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    font-size: 1.4em;
  }

  :global(.avatar__icon) {
    color: white;
  }
</style>
