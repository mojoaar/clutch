<script lang="ts">
  import type { Component } from 'svelte';

  interface Props {
    icon?: Component;
    title: string;
    description: string;
    action?: { label: string; href?: string; onClick?: () => void };
  }

  let { icon, title, description, action }: Props = $props();
</script>

<div class="empty-state">
  {#if icon}
    <div class="empty-state__icon">
      <svelte:component this={icon} strokeWidth={1.5} size={40} />
    </div>
  {/if}
  <h3 class="empty-state__title">{title}</h3>
  <p class="empty-state__desc">{description}</p>
  {#if action}
    <a class="empty-state__action" href={action.href} onclick={action.onClick}>
      {action.label}
    </a>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 48px 24px;
    min-height: 200px;
  }

  .empty-state__icon {
    color: var(--text-dim);
    margin-bottom: 16px;
  }

  .empty-state__title {
    margin: 0 0 8px;
    font-size: 16px;
    font-weight: 600;
    color: var(--text);
  }

  .empty-state__desc {
    margin: 0 0 20px;
    max-width: 300px;
    font-size: 13px;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .empty-state__action {
    display: inline-flex;
    padding: 8px 20px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    background: var(--primary);
    color: white;
  }

  .empty-state__action:hover {
    background: var(--primary-hover);
  }
</style>
