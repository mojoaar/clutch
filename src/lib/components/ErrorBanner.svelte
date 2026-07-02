<script lang="ts">
  import LL from '$lib/i18n/i18n-svelte';
  import { banners, dismissBanner } from '$lib/stores/toast';
  import { CircleAlert, AlertTriangle, Info, X } from '@lucide/svelte';

  const iconMap: Record<string, typeof CircleAlert> = {
    error: CircleAlert,
    warning: AlertTriangle,
    info: Info,
  };
</script>

{#if $banners.length > 0}
  <div class="banners" role="alert">
    {#each $banners as banner (banner.id)}
      <div class="banner banner--{banner.variant}">
        <div class="banner__icon">
          <svelte:component this={iconMap[banner.variant]} strokeWidth={1.5} size={16} />
        </div>
        <span class="banner__message">{banner.message}</span>
        {#if banner.action}
          <button class="banner__action" onclick={banner.action.onClick}>
            {banner.action.label}
          </button>
        {/if}
        <button
          class="banner__close"
          aria-label={$LL.dismiss()}
          onclick={() => dismissBanner(banner.id)}
        >
          <X strokeWidth={1.5} size={14} />
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .banners {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin: 8px 0;
  }

  .banner {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.4;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
  }

  .banner--error {
    border-color: var(--error);
    background: color-mix(in srgb, var(--error) 8%, var(--surface));
  }

  .banner--warning {
    border-color: var(--warning);
    background: color-mix(in srgb, var(--warning) 8%, var(--surface));
  }

  .banner--info {
    border-color: var(--primary);
    background: color-mix(in srgb, var(--primary) 8%, var(--surface));
  }

  .banner__icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .banner--error .banner__icon { color: var(--error); }
  .banner--warning .banner__icon { color: var(--warning); }
  .banner--info .banner__icon { color: var(--primary); }

  .banner__message { flex: 1; min-width: 0; }

  .banner__action {
    flex-shrink: 0;
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    border: none;
    cursor: pointer;
    background: var(--primary);
    color: white;
  }

  .banner__close {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .banner__close:hover {
    background: var(--surface-hover);
    color: var(--text);
  }
</style>
