<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import {
    getSkillDetail,
    installSkill,
    uninstallSkill,
    listInstalledSkills,
    checkForUpdates,
    updateSkill,
    type SkillDetail,
    type SkillUpdateInfo,
  } from '$lib/services/skills';
  import {
    CURATED_SKILLS,
    searchCurated,
    type CuratedSkill,
  } from '$lib/data/curated-skills';
  import { addToast } from '$lib/stores/toast';
  import LL from '$lib/i18n/i18n-svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import {
    Puzzle,
    Search,
    Download,
    Trash2,
    Code,
    Stars,
    ExternalLink,
    ArrowLeft,
    Package,
    User,
    RefreshCw,
    AlertCircle,
  } from '@lucide/svelte';

  type TabId = 'installed' | 'browse';

  let activeTab = $state<TabId>('installed');
  let installedSkills = $state<SkillDetail[]>([]);
  let curatedResults = $state<CuratedSkill[]>([]);
  let searchQuery = $state('');
  let loading = $state(false);
  let installing = $state<Set<string>>(new Set());
  let expandedInstalled = $state<Set<string>>(new Set());
  let expandedBrowse = $state<Set<string>>(new Set());
  let browseDetails = $state<Map<string, SkillDetail>>(new Map());
  let searchTimer: ReturnType<typeof setTimeout> | null = null;
  let checkingUpdates = $state(false);
  let updateInfos = $state<SkillUpdateInfo[]>([]);
  let updatingSkills = $state<Set<string>>(new Set());

  const installedIds = $derived(new Set(installedSkills.map((s) => s.id)));

  async function loadInstalled() {
    try {
      installedSkills = await listInstalledSkills();
    } catch (e) {
      addToast(String(e), 'error');
    }
  }

  function loadCurated() {
    curatedResults = [...CURATED_SKILLS];
  }

  function handleSearchInput(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    searchQuery = value;
    if (searchTimer) clearTimeout(searchTimer);
    if (!value.trim()) {
      loadCurated();
      return;
    }
    searchTimer = setTimeout(() => {
      curatedResults = searchCurated(value.trim());
    }, 300);
  }

  async function handleInstallCurated(skill: CuratedSkill) {
    installing = new Set([...installing, skill.id]);
    try {
      await installSkill(skill.id, skill.source, skill.branch);
      addToast($LL.skills.installedSuccess(), 'success');
      await loadInstalled();
      curatedResults = [...curatedResults];
    } catch (e) {
      addToast($LL.skills.installFailed() + ': ' + String(e), 'error');
    } finally {
      const next = new Set(installing);
      next.delete(skill.id);
      installing = next;
    }
  }

  async function handleUninstall(id: string, name: string) {
    if (!confirm($LL.skills.uninstallConfirm({ name }))) return;
    try {
      await uninstallSkill(id);
      addToast($LL.skills.uninstalledSuccess(), 'success');
      await loadInstalled();
    } catch (e) {
      addToast($LL.skills.uninstallFailed() + ': ' + String(e), 'error');
    }
  }

  async function handleCheckUpdates() {
    checkingUpdates = true;
    try {
      updateInfos = await checkForUpdates();
      const count = updateInfos.filter((u) => u.has_update).length;
      if (count === 0) {
        addToast($LL.skills.upToDate(), 'success');
      }
    } catch (e) {
      addToast(String(e), 'error');
    } finally {
      checkingUpdates = false;
    }
  }

  async function handleUpdateSkill(skillId: string) {
    updatingSkills = new Set([...updatingSkills, skillId]);
    try {
      await updateSkill(skillId);
      addToast($LL.skills.installedSuccess(), 'success');
      await loadInstalled();
      updateInfos = updateInfos.filter((u) => u.id !== skillId);
    } catch (e) {
      addToast($LL.skills.installFailed() + ': ' + String(e), 'error');
    } finally {
      const next = new Set(updatingSkills);
      next.delete(skillId);
      updatingSkills = next;
    }
  }

  function toggleInstalledInstructions(id: string) {
    const next = new Set(expandedInstalled);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    expandedInstalled = next;
  }

  async function toggleBrowseDetailCurated(skill: CuratedSkill) {
    const next = new Set(expandedBrowse);
    if (next.has(skill.id)) {
      next.delete(skill.id);
      expandedBrowse = next;
      return;
    }
    next.add(skill.id);
    expandedBrowse = next;
    if (!browseDetails.has(skill.id)) {
      try {
        const detail = await getSkillDetail(skill.id, skill.source, skill.branch);
        browseDetails = new Map(browseDetails).set(skill.id, detail);
      } catch (e) {
        addToast(String(e), 'error');
        next.delete(skill.id);
        expandedBrowse = next;
      }
    }
  }

  onMount(() => {
    loadInstalled();
    loadCurated();
  });
</script>

<div class="skills-layout">
  <div class="skills-sidebar">
    <div class="skills-sidebar__header">
      <a href="/" class="skills-back" aria-label={$LL.back()}>
        <ArrowLeft size={16} strokeWidth={1.5} />
      </a>
      <h2 class="skills-title">{$LL.skills.title()}</h2>
    </div>
    <nav class="skills-nav">
      <button
        class="skills-nav-item"
        class:skills-nav-item--active={activeTab === 'installed'}
        onclick={() => (activeTab = 'installed')}
      >
        <Puzzle size={15} strokeWidth={1.5} />
        <span>{$LL.skills.installed()}</span>
      </button>
      <button
        class="skills-nav-item"
        class:skills-nav-item--active={activeTab === 'browse'}
        onclick={() => (activeTab = 'browse')}
      >
        <Search size={15} strokeWidth={1.5} />
        <span>{$LL.skills.browse()}</span>
      </button>
    </nav>
  </div>

  <div class="skills-content">
    {#if activeTab === 'installed'}
      <div class="skills-section">
        <div class="skills-section__header">
          <h3 class="skills-section__title">{$LL.skills.installed()}</h3>
          <button
            class="skills-update-btn"
            onclick={handleCheckUpdates}
            disabled={checkingUpdates}
            aria-label={$LL.skills.checkForUpdates()}
          >
            {#if checkingUpdates}
              <RefreshCw size={13} class="spin" />
              {$LL.skills.checking()}
            {:else}
              <RefreshCw size={13} />
              {$LL.skills.checkForUpdates()}
            {/if}
          </button>
        </div>
        {#if updateInfos.filter((u) => u.has_update).length > 0}
          <div class="skills-update-banner">
            <AlertCircle size={14} strokeWidth={1.5} />
            <span>
              {updateInfos.filter((u) => u.has_update).length} {$LL.skills.updateAvailable().toLowerCase()}
            </span>
          </div>
        {/if}
        {#if installedSkills.length === 0}
          <EmptyState
            icon={Puzzle}
            title={$LL.skills.noSkillsInstalled()}
            description={$LL.skills.addSkillsCTA()}
            action={{ label: $LL.skills.browse(), onClick: () => (activeTab = 'browse') }}
          />
        {:else}
          <div class="skills-grid">
            {#each installedSkills as skill (skill.id)}
              <div class="skill-card">
                <div class="skill-card__header">
                  <div class="skill-card__info">
                    <h4 class="skill-card__name">{skill.name}</h4>
                    <p class="skill-card__desc">{skill.description || skill.id}</p>
                  </div>
                  <div class="skill-card__actions">
                    <button
                      class="skill-card__action-btn"
                      onclick={() => toggleInstalledInstructions(skill.id)}
                      aria-label={$LL.skills.instructions()}
                      title={$LL.skills.instructions()}
                    >
                      <Code size={14} strokeWidth={1.5} />
                    </button>
                    <button
                      class="skill-card__action-btn skill-card__action-btn--danger"
                      onclick={() => handleUninstall(skill.id, skill.name)}
                      aria-label={$LL.skills.uninstallSkill()}
                      title={$LL.skills.uninstallSkill()}
                    >
                      <Trash2 size={14} strokeWidth={1.5} />
                    </button>
                  </div>
                </div>
                <div class="skill-card__meta">
                  <span class="skill-card__source">
                    <ExternalLink size={11} strokeWidth={1.5} />
                    {skill.source || 'Unknown'}
                  </span>
                  {#if updateInfos.find((u) => u.id === skill.id)?.has_update}
                    <span class="skill-card__update-badge">
                      <AlertCircle size={11} strokeWidth={1.5} />
                      {$LL.skills.updateAvailable()}
                    </span>
                  {/if}
                </div>
                {#if updateInfos.find((u) => u.id === skill.id)?.has_update}
                  <div class="skill-card__update-action">
                    <span class="skill-card__update-version">
                      {$LL.skills.updateAvailable()}: {updateInfos.find((u) => u.id === skill.id)?.latest_version}
                    </span>
                    <button
                      class="skill-card__update-btn"
                      onclick={() => handleUpdateSkill(skill.id)}
                      disabled={updatingSkills.has(skill.id)}
                    >
                      {#if updatingSkills.has(skill.id)}
                        <RefreshCw size={12} class="spin" />
                        {$LL.skills.updating()}
                      {:else}
                        <Download size={12} strokeWidth={1.5} />
                        {$LL.skills.updateSkill()}
                      {/if}
                    </button>
                  </div>
                {/if}
                {#if expandedInstalled.has(skill.id)}
                  <div class="skill-card__instructions">
                    <h5 class="skill-card__instructions-title">{$LL.skills.instructions()}</h5>
                    <pre class="skill-card__code">{skill.instructions || ''}</pre>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {:else if activeTab === 'browse'}
      <div class="skills-section">
        <div class="skills-search">
          <Search size={15} strokeWidth={1.5} class="skills-search__icon" />
          <input
            class="skills-search__input"
            type="text"
            placeholder={$LL.skills.searchSkills()}
            value={searchQuery}
            oninput={handleSearchInput}
            aria-label={$LL.skills.searchSkills()}
          />
        </div>

        <h3 class="skills-section__title">
          {$LL.skills.curatedCatalog()}
        </h3>

        {#if loading}
          <p class="skills-loading">{$LL.loading()}</p>
        {:else if curatedResults.length === 0}
          <EmptyState
            icon={Search}
            title={$LL.noResults()}
            description={$LL.skills.noTrendingResults()}
          />
        {:else}
          <div class="skills-grid">
            {#each curatedResults as skill (skill.id)}
              <div class="skill-card">
                <div class="skill-card__header">
                  <div
                    class="skill-card__info"
                    role="button"
                    tabindex="0"
                    onkeydown={(e) => e.key === 'Enter' && toggleBrowseDetailCurated(skill)}
                    onclick={() => toggleBrowseDetailCurated(skill)}
                  >
                    <h4 class="skill-card__name">{skill.name}</h4>
                    <p class="skill-card__desc">{skill.description || skill.id}</p>
                  </div>
                  <div class="skill-card__actions">
                    {#if installedIds.has(skill.id)}
                      <span class="skill-card__installed-badge">{$LL.skills.installed()}</span>
                    {:else}
                      <button
                        class="skill-card__action-btn skill-card__action-btn--install"
                        onclick={() => handleInstallCurated(skill)}
                        disabled={installing.has(skill.id)}
                        aria-label={$LL.skills.installSkill()}
                      >
                        {#if installing.has(skill.id)}
                          {$LL.loading()}
                        {:else}
                          <Download size={14} strokeWidth={1.5} />
                        {/if}
                      </button>
                    {/if}
                  </div>
                </div>
                <div class="skill-card__meta">
                  <span class="skill-card__source">
                    <Package size={11} strokeWidth={1.5} />
                    {$LL.skills.category()}: {skill.category}
                  </span>
                  <span class="skill-card__source">
                    <User size={11} strokeWidth={1.5} />
                    {$LL.skills.author()}: {skill.author}
                  </span>
                  <span class="skill-card__source skill-card__source--verified" title={$LL.skills.verified()}>
                    <Stars size={11} strokeWidth={1.5} />
                    Verified
                  </span>
                </div>
                {#if expandedBrowse.has(skill.id)}
                  <div class="skill-card__instructions">
                    <h5 class="skill-card__instructions-title">{$LL.skills.instructions()}</h5>
                    <pre class="skill-card__code">{browseDetails.get(skill.id)?.instructions || $LL.loading()}</pre>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .skills-layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .skills-sidebar {
    width: 220px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .skills-sidebar__header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px;
    border-bottom: 1px solid var(--border);
  }

  .skills-back {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 6px;
    color: var(--text-dim);
  }

  .skills-back:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .skills-title {
    margin: 0;
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
  }

  .skills-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px;
  }

  .skills-nav-item {
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

  .skills-nav-item:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .skills-nav-item--active {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
    color: var(--primary);
    font-weight: 600;
  }

  .skills-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px calc(5% + 80px) 24px 5%;
  }

  .skills-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .skills-section__title {
    margin: 0;
    font-size: 13px;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .skills-search {
    display: flex;
    align-items: center;
    gap: 8px;
    border: 1px solid var(--border);
    border-radius: 7px;
    background: var(--bg);
    padding: 0 10px;
  }

  .skills-search:focus-within {
    border-color: var(--primary);
  }

  .skills-search__icon {
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .skills-search__input {
    flex: 1;
    padding: 8px 0;
    border: none;
    border-radius: 0;
    background: transparent;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    outline: none;
  }

  .skills-search__input::placeholder {
    color: var(--text-dim);
  }

  .skills-loading {
    font-size: 13px;
    color: var(--text-muted);
    padding: 8px 0;
  }

  .skills-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .skill-card {
    border: 1px solid var(--border);
    border-radius: 10px;
    background: var(--surface);
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .skill-card__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .skill-card__info {
    flex: 1;
    min-width: 0;
    cursor: pointer;
  }

  .skill-card__info[role="button"]:focus-visible {
    outline: 2px solid var(--primary);
    outline-offset: 2px;
    border-radius: 4px;
  }

  .skill-card__name {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
  }

  .skill-card__desc {
    margin: 4px 0 0;
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .skill-card__actions {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .skill-card__action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
  }

  .skill-card__action-btn:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  .skill-card__action-btn--danger:hover {
    background: color-mix(in srgb, var(--error) 15%, transparent);
    color: var(--error);
  }

  .skill-card__action-btn--install {
    color: var(--primary);
  }

  .skill-card__action-btn--install:hover {
    background: color-mix(in srgb, var(--primary) 15%, transparent);
    color: var(--primary-hover);
  }

  .skill-card__action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .skill-card__installed-badge {
    padding: 3px 10px;
    border-radius: 6px;
    background: color-mix(in srgb, var(--success) 15%, transparent);
    color: var(--success);
    font-size: 11px;
    font-weight: 600;
  }

  .skill-card__meta {
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 11px;
    color: var(--text-dim);
    flex-wrap: wrap;
  }

  .skill-card__source {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .skill-card__source--verified {
    color: var(--success);
  }

  .skill-card__stars {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .skill-card__instructions {
    border-top: 1px solid var(--border);
    padding-top: 10px;
  }

  .skill-card__instructions-title {
    margin: 0 0 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .skill-card__code {
    max-height: 300px;
    overflow: auto;
    padding: 12px;
    border-radius: 6px;
    background: var(--code-bg, color-mix(in srgb, var(--bg) 50%, var(--surface)));
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.5;
    color: var(--text-muted);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .skills-section__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .skills-update-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 4px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    color: var(--text-dim);
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .skills-update-btn:hover {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
    color: var(--primary);
    border-color: var(--primary);
  }

  .skills-update-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .skills-update-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 7px;
    background: color-mix(in srgb, var(--warning) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--warning) 25%, transparent);
    color: var(--warning);
    font-size: 12px;
    font-weight: 500;
  }

  .skill-card__update-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    color: var(--warning);
    font-weight: 600;
  }

  .skill-card__update-action {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    border-radius: 6px;
    background: color-mix(in srgb, var(--warning) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--warning) 15%, transparent);
  }

  .skill-card__update-version {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .skill-card__update-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 4px 10px;
    border: 1px solid var(--warning);
    border-radius: 5px;
    background: var(--warning);
    color: #000;
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .skill-card__update-btn:hover {
    opacity: 0.85;
  }

  .skill-card__update-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
