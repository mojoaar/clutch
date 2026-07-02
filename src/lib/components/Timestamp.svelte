<script lang="ts">
  import { getSetting } from '$lib/services/settings';

  interface Props {
    date: string;
    format?: 'auto' | '12h' | '24h';
  }

  let { date, format = 'auto' }: Props = $props();

  let is12h = $state(false);

  $effect(() => {
    if (format === 'auto') {
      is12h = !/^[^APMapm]*[APMapm]/.test(new Date().toLocaleTimeString());
    } else {
      is12h = format === '12h';
    }
  });

  $effect(() => {
    getSetting('time_format').then((saved) => {
      if (saved === '12') is12h = true;
      if (saved === '24') is12h = false;
    });
  });

  let time = $derived(
    new Date(date).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: is12h }),
  );

  let full = $derived(
    new Date(date).toLocaleString([], {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      hour12: is12h,
    }),
  );
</script>

<time class="timestamp" datetime={date} title={full}>
  {time}
</time>

<style>
  .timestamp {
    font-size: 11px;
    color: var(--text-dim);
    white-space: nowrap;
  }
</style>
