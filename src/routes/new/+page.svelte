<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { createNewSession } from '$lib/services/sessions';
  import LL from '$lib/i18n/i18n-svelte';

  let error = $state('');
  let created = $state(false);

  onMount(async () => {
    try {
      await createNewSession($LL.chat.newChat());
      created = true;
      goto('/chat', { replaceState: true });
    } catch (e) {
      error = String(e);
    }
  });
</script>

{#if error}
  <div style="padding: 2rem; text-align: center;">
    <p style="color: var(--danger);">{$LL.errors.unknownError()}: {error}</p>
    <button onclick={() => goto('/chat')}>Go to Chat</button>
  </div>
{:else if !created}
  <div style="padding: 2rem; text-align: center;">
    <p>{$LL.chat.newChat()}...</p>
  </div>
{/if}
