<script lang="ts">
  import { page } from '$app/stores';
  import { themeName, themeMode } from '$lib/stores/theme';
  import { devMode as devModeStore } from '$lib/stores/dev';
  import { themes, type ThemeName, type ThemeMode } from '$lib/themes';
  import { PROVIDERS } from '$lib/services/providers';
  import { setSetting, getAllSettings, getSetting } from '$lib/services/settings';
  import { invoke } from '@tauri-apps/api/core';
  import { enable as autostartEnable, disable as autostartDisable, isEnabled as autostartIsEnabled } from '@tauri-apps/plugin-autostart';
  import { addToast } from '$lib/stores/toast';
  import { onMount, onDestroy } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readFile } from '@tauri-apps/plugin-fs';
  import ThemeSelector from '$lib/components/ThemeSelector.svelte';

  let providers = $state({...PROVIDERS});
  import ProviderSelector from '$lib/components/ProviderSelector.svelte';
  import ModelSelector from '$lib/components/ModelSelector.svelte';
  import WorkspaceSelector from '$lib/components/WorkspaceSelector.svelte';
  import ExportDialog from '$lib/components/ExportDialog.svelte';
  import ShortcutRecorder from '$lib/components/ShortcutRecorder.svelte';
  import { updateStore } from '$lib/services/updater';
  import {
    Settings,
    Globe,
    PlugZap,
    Rocket,
    Keyboard,
    FolderOpen,
    Download,
    Cpu,
    ArrowLeft,
    RefreshCw,
  } from '@lucide/svelte';
  import LL from '$lib/i18n/i18n-svelte';
  import { setLocale } from '$lib/i18n/i18n-svelte';
  import type { Locales } from '$lib/i18n/i18n-types';
  import Avatar from '$lib/components/Avatar.svelte';
  import * as db from '$lib/db';

  type TabId = 'general' | 'providers' | 'startup' | 'shortcuts' | 'workspaces' | 'export' | 'models' | 'updates';

  interface Tab {
    id: TabId;
    label: string;
    icon: typeof Settings;
  }

  const tabs: { id: TabId; icon: typeof Settings }[] = [
    { id: 'general', icon: Settings },
    { id: 'providers', icon: PlugZap },
    { id: 'models', icon: Cpu },
    { id: 'startup', icon: Rocket },
    { id: 'shortcuts', icon: Keyboard },
    { id: 'workspaces', icon: FolderOpen },
    { id: 'export', icon: Download },
    { id: 'updates', icon: RefreshCw },
  ];

  const tabLabels = $derived({
    general: $LL.settingsTabs.general(),
    providers: $LL.settingsTabs.providers(),
    models: $LL.settingsTabs.models(),
    startup: $LL.settingsTabs.startup(),
    shortcuts: $LL.settingsTabs.shortcuts(),
    workspaces: $LL.settingsTabs.workspaces(),
    export: $LL.settingsTabs.export(),
    updates: $LL.settingsTabs.updates(),
  });

  let activeTab = $state<TabId>('general');

  $effect(() => {
    const tab = $page.url.searchParams.get('tab') as TabId | null;
    if (tab && tabs.some((t) => t.id === tab)) {
      activeTab = tab;
    }
  });

  // General
  let timeFormat = $state('auto');
  let showTimestamps = $state(true);
  let devMode = $state(false);

  $effect(() => {
    invoke('toggle_dev_mode', { enabled: devMode }).catch((e) => { console.debug('Failed to toggle dev mode', e); });
    devModeStore.set(devMode);
  });
  let skillAllowFileWrite = $state(false);
  let skillAllowNetwork = $state(false);
  let skillAllowProcess = $state(false);

  // Providers
  let apiKeys: Record<string, string> = $state({});
  let providerModels: Record<string, string> = $state({});
  let providerEnabled: Record<string, boolean> = $state({});

  // Startup
  let startOnBoot = $state(false);

  onMount(async () => { try { startOnBoot = await autostartIsEnabled(); } catch (e) { console.debug('Failed to check autostart status', e); } });
  $effect(() => {
    if (startOnBoot) { autostartEnable().catch((e) => { console.debug('Failed to enable autostart', e); }); } else { autostartDisable().catch((e) => { console.debug('Failed to disable autostart', e); }); }
  });

  let startMinimized = $state(false);
  let closeToTray = $state(true);
  let showTrayIcon = $state(true);

  // Shortcuts
  let shortcutMain = $state('CmdOrCtrl+Shift+M');

  // Workspaces
  let workspaces = $state<string[]>([]);
  let activeWorkspace = $state<string | null>(null);

  // Export
  let exportFormat = $state('markdown');
  let exportIncludeMetadata = $state(true);
  let exportIncludeTimestamps = $state(true);
  let exportIncludeProviderInfo = $state(true);

  let language = $state('en');
  let defaultProvider = $state('deepseek');
  let appVersion = $state('');

  // Updates
  let updateAutoCheck = $state(true);
  let updateAutoDownload = $state(true);
  let updateChannel = $state('stable');
  let updateLastChecked = $state('');
  let updateStatus = $state('idle');

  const unsubUpdate = updateStore.subscribe((s) => {
    updateStatus = s.status;
    if (s.status === 'idle' && !updateLastChecked) {
      updateLastChecked = new Date().toISOString();
      setSetting('last_update_check', updateLastChecked).catch(() => {});
    }
  });

  let lastCheckedDisplay = $derived.by(() => {
    if (!updateLastChecked) return $LL.updates.never();
    const d = new Date(updateLastChecked);
    const datePart = d.toLocaleDateString('en-GB');
    let hour12Val: boolean;
    if (timeFormat === '12') hour12Val = true;
    else if (timeFormat === '24') hour12Val = false;
    else hour12Val = !/^[^APMapm]*[APMapm]/.test(new Date().toLocaleTimeString());
    const timePart = d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: hour12Val });
    return `${datePart} ${timePart}`;
  });

  onDestroy(() => unsubUpdate());

  // Profile
  let profileName = $state('');
  let profileType = $state<'icon' | 'image'>('icon');
  let profileColor = $state('#6366f1');
  let profileImageUrl = $state('');

  async function handleChooseImage() {
    try {
      const selected = await open({
        filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'] }],
        multiple: false,
      });
      if (!selected) return;
      const path = typeof selected === 'string' ? selected : selected;
      const bytes = await readFile(path);
      const ext = path.split('.').pop()?.toLowerCase() || 'png';
      const mime = { png: 'image/png', jpg: 'image/jpeg', jpeg: 'image/jpeg', gif: 'image/gif', webp: 'image/webp' }[ext] || 'image/png';
      const base64 = btoa(String.fromCharCode(...bytes));
      profileImageUrl = `data:${mime};base64,${base64}`;
    } catch (e) {
      addToast(`Failed to load image: ${e}`, 'error');
    }
  }

  onMount(async () => {
    try {
      const all = await getAllSettings();
      for (const [key, value] of Object.entries(all)) {
        if (key.startsWith('default_model_') && value) providerModels[key.replace('default_model_', '')] = value;
        else if (key.startsWith('provider_enabled_')) {
          const pid = key.replace('provider_enabled_', '');
          const val = value === 'true';
          providerEnabled[pid] = val;
          PROVIDERS[pid].enabled = val;
          if (providers[pid]) providers[pid].enabled = val;
        }
        else if (key === 'language') language = value;
        else if (key === 'default_provider') defaultProvider = value;
        else if (key === 'time_format') timeFormat = value;
        else if (key === 'show_timestamps') showTimestamps = value === 'true';
        else if (key === 'dev_mode') { devMode = value === 'true'; devModeStore.set(devMode); }
        else if (key === 'skill_allow_file_write') skillAllowFileWrite = value === 'true';
        else if (key === 'skill_allow_network') skillAllowNetwork = value === 'true';
        else if (key === 'skill_allow_process') skillAllowProcess = value === 'true';
        else if (key === 'start_on_boot') startOnBoot = value === 'true';
        else if (key === 'start_minimized') startMinimized = value === 'true';
        else if (key === 'close_to_tray') closeToTray = value === 'true';
        else if (key === 'show_tray_icon') showTrayIcon = value !== 'false';
        else if (key === 'shortcut_main') shortcutMain = value;
        else if (key === 'workspaces') workspaces = JSON.parse(value);
        else if (key === 'active_workspace') activeWorkspace = value || null;
        else if (key === 'export_format') exportFormat = value;
        else if (key === 'export_metadata') exportIncludeMetadata = value === 'true';
        else if (key === 'export_timestamps') exportIncludeTimestamps = value === 'true';
        else if (key === 'export_provider_info') exportIncludeProviderInfo = value === 'true';
        else if (key === 'auto_check') updateAutoCheck = value !== 'false';
        else if (key === 'auto_download') updateAutoDownload = value !== 'false';
        else if (key === 'update_channel') updateChannel = value;
        else if (key === 'last_update_check') updateLastChecked = value;
      }

      for (const providerId of ['deepseek', 'opencode_go', 'opencode_zen']) {
        const k = await getSetting(`api_key_${providerId}`);
        if (k) apiKeys[providerId] = k;
      }

      const profile = await db.getUserProfile();
      if (profile) {
        profileName = profile.displayName;
        profileType = profile.avatarType as 'icon' | 'image';
        profileColor = profile.avatarColor;
        if (profile.avatarType === 'image' && profile.avatarData) {
          profileImageUrl = profile.avatarData;
        }
      }

      appVersion = await invoke<string>('get_app_version');
    } catch (e) { console.debug('Failed to load settings on mount', e); }
  });

  function handleApiKeyChange(providerId: string, key: string) {
    apiKeys[providerId] = key;
    setSetting(`api_key_${providerId}`, key).catch((e) => { console.debug('Failed to save API key setting', providerId, e); });
  }

  function handleModelChange(providerId: string, model: string) {
    providerModels[providerId] = model;
    setSetting(`default_model_${providerId}`, model).catch((e) => { console.debug('Failed to save default model setting', providerId, e); });
  }

  function handleToggleProvider(providerId: string) {
    const enabled = !providers[providerId].enabled;
    providers[providerId].enabled = enabled;
    providerEnabled[providerId] = enabled;
    PROVIDERS[providerId].enabled = enabled;
    setSetting(`provider_enabled_${providerId}`, String(enabled)).catch((e) => { console.debug('Failed to save provider enabled setting', providerId, e); });
  }

  function handleAddWorkspace() {
    const path = prompt($LL.enterWorkspacePath());
    if (path && !workspaces.includes(path)) {
      workspaces = [...workspaces, path];
    }
  }

  function handleRemoveWorkspace(path: string) {
    workspaces = workspaces.filter((w) => w !== path);
    if (activeWorkspace === path) {
      activeWorkspace = workspaces[0] || null;
    }
  }

  async function handleSaveSettings() {
    try {
      await setSetting('time_format', timeFormat);
      await setSetting('show_timestamps', String(showTimestamps));
      await setSetting('dev_mode', String(devMode));
      await setSetting('skill_allow_file_write', String(skillAllowFileWrite));
      await setSetting('skill_allow_network', String(skillAllowNetwork));
      await setSetting('skill_allow_process', String(skillAllowProcess));
      for (const [provider, key] of Object.entries(apiKeys)) {
        if (key) await setSetting(`api_key_${provider}`, key);
      }
      for (const [provider, model] of Object.entries(providerModels)) {
        await setSetting(`default_model_${provider}`, model);
      }
      for (const [provider, enabled] of Object.entries(providerEnabled)) {
        await setSetting(`provider_enabled_${provider}`, String(enabled));
      }
      await setSetting('start_on_boot', String(startOnBoot));
      await setSetting('start_minimized', String(startMinimized));
      await setSetting('close_to_tray', String(closeToTray));
      await setSetting('show_tray_icon', String(showTrayIcon));
      await invoke('toggle_tray_icon', { visible: showTrayIcon });
      await setSetting('shortcut_main', shortcutMain);
      await setSetting('workspaces', JSON.stringify(workspaces));
      await setSetting('active_workspace', activeWorkspace || '');
      await setSetting('export_format', exportFormat);
      await setSetting('export_metadata', String(exportIncludeMetadata));
      await setSetting('export_timestamps', String(exportIncludeTimestamps));
      await setSetting('export_provider_info', String(exportIncludeProviderInfo));
      await setSetting('auto_check', String(updateAutoCheck));
      await setSetting('auto_download', String(updateAutoDownload));
      await setSetting('update_channel', updateChannel);
      await setSetting('language', language);
      await setSetting('default_provider', defaultProvider);
      await db.updateUserProfile({
        displayName: profileName,
        avatarType: profileType,
        avatarData: profileType === 'image' ? profileImageUrl : '',
        avatarColor: profileColor,
      });
      addToast($LL.toast.settingsSaved(), 'info');
    } catch (e) {
      addToast(`Failed to save settings: ${e}`, 'error');
    }
  }

  function handleLanguageChange(e: Event) {
    language = (e.target as HTMLSelectElement).value;
    setLocale(language as Locales);
  }
</script>

<div class="settings-layout">
  <div class="settings-sidebar">
    <div class="settings-sidebar__header">
      <a href="/" class="settings-back" aria-label={$LL.back()}>
        <ArrowLeft size={16} strokeWidth={1.5} />
      </a>
      <h2 class="settings-title">{$LL.settings()}</h2>
    </div>
    <nav class="settings-nav">
      {#each tabs as tab}
        <button
          class="settings-nav-item"
          class:settings-nav-item--active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
        >
          <tab.icon size={15} strokeWidth={1.5} />
          <span>{tabLabels[tab.id]}</span>
        </button>
      {/each}
    </nav>
  </div>

  <div class="settings-content">
    {#if activeTab === 'general'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.profile.title()}</h3>

        <div class="profile-preview">
          <Avatar
            name={profileName || undefined}
            imageUrl={profileType === 'image' ? profileImageUrl || undefined : undefined}
            color={profileColor}
            size={48}
          />
          <div class="profile-info">
            <span class="profile-name">{profileName || $LL.profile.setName()}</span>
            <span class="profile-type">{profileType}</span>
          </div>
        </div>

        <div class="field">
          <label class="field-label" for="profile-name">{$LL.profile.displayName()}</label>
          <input
            id="profile-name"
            class="field-select"
            type="text"
            placeholder={$LL.profile.yourName()}
            bind:value={profileName}
          />
        </div>

        <div class="field">
          <label class="field-label" for="profile-type">{$LL.profile.avatarType()}</label>
          <select id="profile-type" class="field-select" bind:value={profileType}>
            <option value="icon">{$LL.profile.iconDefault()}</option>
            <option value="image">{$LL.profile.image()}</option>
          </select>
        </div>

        {#if profileType === 'image'}
          <div class="field">
            <span class="field-label">{$LL.profile.image()}</span>
            <button class="settings-save-btn" onclick={handleChooseImage} type="button">
              {profileImageUrl ? $LL.profile.changeImage() : $LL.profile.chooseImage()}
            </button>
            {#if profileImageUrl}
              <img class="profile-image-preview" src={profileImageUrl} alt={$LL.profile.avatarPreview()} />
            {/if}
          </div>
        {/if}

        {#if profileType === 'icon'}
          <div class="field">
            <label class="field-label" for="profile-color">{$LL.profile.avatarColor()}</label>
            <input
              id="profile-color"
              class="field-select"
              type="color"
              bind:value={profileColor}
            />
          </div>
        {/if}
      </div>

      <div class="settings-divider"></div>

      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.settingsSections.appearance()}</h3>
        <ThemeSelector />

        <div class="settings-divider"></div>

        <h3 class="settings-section__title">{$LL.settingsSections.locale()}</h3>
        <div class="field">
          <label class="field-label" for="settings-language">{$LL.language()}</label>
          <select
            id="settings-language"
            class="field-select"
            value={language}
            oninput={handleLanguageChange}
          >
            <option value="en">{$LL.english()}</option>
            <option value="da">{$LL.danish()}</option>
            <option value="de">{$LL.german()}</option>
            <option value="pl">{$LL.polish()}</option>
            <option value="fr">{$LL.french()}</option>
          </select>
        </div>

        <div class="settings-divider"></div>

        <h3 class="settings-section__title">{$LL.settingsSections.timeDate()}</h3>
        <div class="field">
          <label class="field-label" for="settings-time-format">{$LL.theme.timeFormat()}</label>
          <select
            id="settings-time-format"
            class="field-select"
            bind:value={timeFormat}
          >
            <option value="auto">{$LL.theme.auto()}</option>
            <option value="12">{$LL.theme.hour12()}</option>
            <option value="24">{$LL.theme.hour24()}</option>
          </select>
        </div>

        <label class="check">
          <input type="checkbox" bind:checked={showTimestamps} />
          <span>{$LL.settingsSections.showTimestamps()}</span>
        </label>

        <div class="settings-divider"></div>

        <h3 class="settings-section__title">{$LL.startup.showTrayIcon()}</h3>
        <label class="check">
          <input type="checkbox" bind:checked={showTrayIcon} />
          <span>{$LL.startup.showTrayIconDesc()}</span>
        </label>

        <div class="settings-divider"></div>

        <h3 class="settings-section__title">{$LL.settingsSections.permissions()}</h3>
        <label class="check">
          <input type="checkbox" bind:checked={skillAllowFileWrite} />
          <span>{$LL.permissions.allowFileWrite()}</span>
        </label>
        <p class="about-license" style="margin:-6px 0 8px 24px">{$LL.permissions.allowFileWriteDesc()}</p>
        <label class="check">
          <input type="checkbox" bind:checked={skillAllowNetwork} />
          <span>{$LL.permissions.allowNetwork()}</span>
        </label>
        <p class="about-license" style="margin:-6px 0 8px 24px">{$LL.permissions.allowNetworkDesc()}</p>
        <label class="check">
          <input type="checkbox" bind:checked={skillAllowProcess} />
          <span>{$LL.permissions.allowProcess()}</span>
        </label>
        <p class="about-license" style="margin:-6px 0 8px 24px">{$LL.permissions.allowProcessDesc()}</p>

        <div class="settings-divider"></div>

        <h3 class="settings-section__title">{$LL.settingsSections.developer()}</h3>
        <label class="check">
          <input type="checkbox" bind:checked={devMode} />
          <span>{$LL.settingsSections.developerMode()}</span>
        </label>
        <p class="about-license" style="margin:-4px 0 8px 24px">{$LL.settingsSections.developerHint()}</p>

        <div class="settings-divider"></div>
        <h3 class="settings-section__title">{$LL.settingsSections.about()}</h3>
        <div class="about-section">
          <p><strong>Clutch</strong> v{appVersion} — {$LL.welcome.tagline()}</p>
          <p>{$LL.about.builtBy()}</p>
          <p><a href="https://github.com/mojoaar/clutch" target="_blank" rel="noopener">github.com/mojoaar/clutch</a></p>
          <p class="about-license">{$LL.about.license()}</p>
        </div>
      </div>

    {:else if activeTab === 'providers'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.settingsSections.apiProviders()}</h3>

        <div class="field">
          <label class="field-label" for="settings-default-provider">{$LL.providers.defaultModel()}</label>
          <select
            id="settings-default-provider"
            class="field-select"
            bind:value={defaultProvider}
            aria-label={$LL.aria.defaultProvider()}
          >
            {#each Object.values(providers) as p}
              <option value={p.id}>{p.name}</option>
            {/each}
          </select>
        </div>

        <div class="provider-list">
          {#each Object.values(providers) as provider}
            <ProviderSelector
              {provider}
              apiKey={apiKeys[provider.id] || ''}
              onApiKeyChange={handleApiKeyChange}
              onModelChange={handleModelChange}
              onToggle={handleToggleProvider}
            />
          {/each}
        </div>
      </div>

    {:else if activeTab === 'startup'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.settingsSections.startupBehavior()}</h3>
        <div class="checks">
          <label class="check">
            <input type="checkbox" bind:checked={startOnBoot} />
            <span>{$LL.startup.startOnBoot()}</span>
          </label>
          <label class="check">
            <input type="checkbox" bind:checked={startMinimized} />
            <span>{$LL.startup.startMinimized()}</span>
          </label>
          <label class="check">
            <input type="checkbox" bind:checked={closeToTray} />
            <span>{$LL.startup.closeToTray()}</span>
          </label>
        </div>
      </div>

    {:else if activeTab === 'shortcuts'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.settingsSections.keyboardShortcuts()}</h3>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.newChat()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">N</kbd>
          </div>
        </div>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.archiveChat()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">Shift</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">A</kbd>
          </div>
        </div>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.deleteChat()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">⌫</kbd>
          </div>
        </div>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.renameChat()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">E</kbd>
          </div>
        </div>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.pinUnpinChat()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">Shift</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">P</kbd>
          </div>
        </div>
        <div class="shortcut-row shortcut-row--static">
          <div class="shortcut-row__action">{$LL.shortcutActions.copyConversation()}</div>
          <div class="shortcut-row__keys">
            <kbd class="shortcut-kbd">Cmd</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">Shift</kbd><span class="shortcut-plus">+</span><kbd class="shortcut-kbd">C</kbd>
          </div>
        </div>
        <ShortcutRecorder
          action={$LL.shortcutActions.toggleMainWindow()}
          value={shortcutMain}
          onChange={(v) => (shortcutMain = v)}
        />

      </div>

    {:else if activeTab === 'workspaces'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.workspaces.title()}</h3>
        <p class="about-license" style="margin-bottom:12px">{$LL.workspaces.description()}</p>
        <WorkspaceSelector
          {workspaces}
          {activeWorkspace}
          onAdd={handleAddWorkspace}
          onRemove={handleRemoveWorkspace}
          onSelect={(path) => (activeWorkspace = path)}
        />
      </div>

    {:else if activeTab === 'export'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.settingsSections.defaultExport()}</h3>
        <ExportDialog
          format={exportFormat}
          includeMetadata={exportIncludeMetadata}
          includeTimestamps={exportIncludeTimestamps}
          includeProviderInfo={exportIncludeProviderInfo}
          onFormatChange={(v) => (exportFormat = v)}
          onMetadataChange={(v) => (exportIncludeMetadata = v)}
          onTimestampsChange={(v) => (exportIncludeTimestamps = v)}
          onProviderInfoChange={(v) => (exportIncludeProviderInfo = v)}
        />
      </div>

    {:else if activeTab === 'models'}
      <div class="settings-section">
        <ModelSelector {apiKeys} />
      </div>

    {:else if activeTab === 'updates'}
      <div class="settings-section">
        <h3 class="settings-section__title">{$LL.updates.title()}</h3>

        <div class="checks">
          <label class="check">
            <input type="checkbox" bind:checked={updateAutoCheck} />
            <span>{$LL.updates.autoCheck()}</span>
          </label>
          <label class="check">
            <input type="checkbox" bind:checked={updateAutoDownload} />
            <span>{$LL.updates.autoDownload()}</span>
          </label>
        </div>

        <div class="field">
          <label class="field-label" for="settings-update-channel">{$LL.updates.channel()}</label>
          <select
            id="settings-update-channel"
            class="field-select"
            bind:value={updateChannel}
          >
            <option value="stable">{$LL.updates.stable()}</option>
            <option value="beta">{$LL.updates.beta()}</option>
          </select>
        </div>

        <div class="settings-divider"></div>

        <div class="update-status-section">
          <button
            class="settings-save-btn"
            onclick={() => {
              updateLastChecked = new Date().toISOString();
              setSetting('last_update_check', updateLastChecked).catch(() => {});
              updateStore.checkForUpdates();
            }}
            disabled={updateStatus === 'checking' || updateStatus === 'downloading'}
            type="button"
          >
            {#if updateStatus === 'checking' || updateStatus === 'downloading'}
              {$LL.updates.checking()}
            {:else}
              {$LL.updates.checkNow()}
            {/if}
          </button>

          <div class="update-info">
            <p class="update-info__version">Clutch v{appVersion}</p>
            <p class="update-info__last-checked">
              {$LL.updates.lastChecked()}:
              {lastCheckedDisplay}
            </p>
          </div>
        </div>
      </div>
    {/if}

    <div class="settings-actions">
      <button class="settings-save-btn" onclick={handleSaveSettings}>
        {$LL.save()}
      </button>
    </div>
  </div>
</div>

<style>
  .settings-layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .settings-sidebar {
    width: 220px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .settings-sidebar__header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px;
    border-bottom: 1px solid var(--border);
  }

  .settings-back {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 6px;
    color: var(--text-dim);
  }

  .settings-back:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .settings-title {
    margin: 0;
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px;
  }

  .settings-nav-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border: none;
    border-radius: 7px;
    background: transparent;
    color: var(--text-dim);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    text-align: left;
  }

  .settings-nav-item:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .settings-nav-item--active {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
    color: var(--primary);
    font-weight: 600;
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px calc(5% + 80px) 24px 5%;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .settings-section__title {
    margin: 0;
    font-size: 13px;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .settings-divider {
    height: 1px;
    background: var(--border);
    margin: 4px 0;
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
    gap: 10px;
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

  .provider-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .settings-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 24px;
    border-top: 1px solid var(--border);
  }

  .settings-save-btn {
    padding: 8px 24px;
    border: none;
    border-radius: 8px;
    background: var(--primary);
    color: white;
    font-family: inherit;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .settings-save-btn:hover {
    background: var(--primary-hover);
  }

  .profile-preview {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 12px;
    border-radius: 10px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .profile-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
  }

  .profile-type {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: capitalize;
  }

  .about-section {
    font-size: 13px;
    color: var(--text-muted);
    line-height: 1.8;
  }

  .about-section a {
    color: var(--primary);
    text-decoration: none;
  }

  .about-section a:hover {
    text-decoration: underline;
  }

  .about-license {
    font-size: 11px;
    color: var(--text-dim);
  }

  .profile-image-preview {
    margin-top: 8px;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid var(--border);
  }
  .shortcut-row--static {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--surface);
    margin-bottom: 8px;
  }

  .shortcut-row__action {
    font-size: 13px;
    color: var(--text);
  }

  .shortcut-row__keys {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .shortcut-kbd {
    padding: 2px 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface-hover);
    font-family: inherit;
    font-size: 12px;
    color: var(--text-dim);
  }

  .shortcut-plus {
    font-size: 12px;
    color: var(--text-dim);
  }
  .update-status-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;
  }
  .update-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .update-info__version {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }
  .update-info__last-checked {
    font-size: 11px;
    color: var(--text-dim);
    margin: 0;
  }
</style>
