<script lang="ts">
  import '../app.css';
  import '../lib/stores/theme';
  import { onMount, onDestroy } from 'svelte';
  import { setLocale } from '$lib/i18n/i18n-svelte';
  import type { Locales } from '$lib/i18n/i18n-types';
  import { getSetting, setSetting } from '$lib/services/settings';
  import { goto, afterNavigate } from '$app/navigation';
  import { Settings, Sun, Moon, Puzzle } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';
  import { themeMode } from '$lib/stores/theme';
  import Avatar from '$lib/components/Avatar.svelte';
  import * as db from '$lib/db';
  import Toast from '$lib/components/Toast.svelte';
  import ErrorBanner from '$lib/components/ErrorBanner.svelte';
  import { updateStore } from '$lib/services/updater';

  let profile: { displayName: string; avatarType: string; avatarData: string; avatarColor: string } | null = $state(null);
  let showUpdateBanner = $state(false);
  let updateVersion = $state('');
  let updateDismissed = $state(false);

  const unsubUpdate = updateStore.subscribe((s) => {
    if (s.status === 'ready' && !updateDismissed) {
      showUpdateBanner = true;
      updateVersion = s.version || '';
    }
  });

  onDestroy(() => unsubUpdate());

  function handleDismissUpdate() {
    showUpdateBanner = false;
    updateDismissed = true;
  }

  async function handleRestartUpdate() {
    await updateStore.restart();
  }

  onMount(async () => {
    const savedLang = await getSetting('language');
    if (savedLang && savedLang !== 'en') setLocale(savedLang as Locales);
    const savedMode = await getSetting('theme_mode');
    if (savedMode === 'light' || savedMode === 'dark') {
      themeMode.set(savedMode);
    }
    const autoCheck = await getSetting('auto_check');
    if (autoCheck !== 'false') {
      updateStore.checkForUpdates();
    }
  });

  function toggleTheme() {
    const next = $themeMode === 'dark' ? 'light' : 'dark';
    themeMode.set(next);
    setSetting('theme_mode', next).catch(() => {});
  }

  afterNavigate(async () => {
    try { profile = await db.getUserProfile(); } catch (_) {}
  });

  interface Props {
    children?: import('svelte').Snippet;
  }

  let { children }: Props = $props();
</script>

<ErrorBanner />
<Toast />

{#if showUpdateBanner}
  <div class="update-banner">
    <span class="update-banner__text">
      {$LL.updates.ready()}{#if updateVersion} (v{updateVersion}){/if}
    </span>
    <div class="update-banner__actions">
      <button class="update-banner__btn update-banner__btn--primary" onclick={handleRestartUpdate}>
        {$LL.updates.restartNow()}
      </button>
      <button class="update-banner__btn" onclick={handleDismissUpdate}>
        {$LL.dismiss()}
      </button>
    </div>
  </div>
{/if}

<nav class="topbar">
  {#if profile}
    <button class="topbar__avatar" onclick={() => goto('/settings')} aria-label={$LL.aria.profile()}>
      <Avatar
        name={profile.displayName || undefined}
        emoji={profile.avatarType === 'emoji' ? profile.avatarData || undefined : undefined}
        imageUrl={profile.avatarType === 'image' ? profile.avatarData || undefined : undefined}
        color={profile.avatarColor}
        size={24}
      />
    </button>
  {/if}
  <button class="topbar__link" onclick={() => goto('/skills')} aria-label={$LL.aria.skills()}>
    <Puzzle strokeWidth={1.5} size={18} />
  </button>
  <button class="topbar__link" onclick={() => goto('/settings')} aria-label={$LL.aria.settings()}>
    <Settings strokeWidth={1.5} size={18} />
  </button>
  <button class="topbar__link" onclick={toggleTheme} aria-label={$LL.aria.toggleTheme()}>
    {#if $themeMode === 'dark'}
      <Sun strokeWidth={1.5} size={18} />
    {:else}
      <Moon strokeWidth={1.5} size={18} />
    {/if}
  </button>
</nav>

{#if children}
  {@render children()}
{/if}

<style>
  .topbar {
    position: fixed;
    top: 0;
    right: 0;
    display: flex;
    gap: 4px;
    padding: 8px 12px;
    z-index: 100;
    background: var(--bg);
  }

  .topbar__link {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .topbar__link:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .topbar__avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .update-banner {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 200;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 10px 20px;
    background: var(--primary);
    color: white;
    font-size: 13px;
    font-weight: 500;
  }
  .update-banner__text {
    flex: 1;
  }
  .update-banner__actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }
  .update-banner__btn {
    padding: 4px 14px;
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 6px;
    background: transparent;
    color: white;
    font-family: inherit;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }
  .update-banner__btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }
  .update-banner__btn--primary {
    background: white;
    color: var(--primary);
    border-color: white;
  }
  .update-banner__btn--primary:hover {
    background: rgba(255, 255, 255, 0.9);
  }
</style>
