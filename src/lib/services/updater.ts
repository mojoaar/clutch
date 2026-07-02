import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { writable } from 'svelte/store';

export type UpdateStatus = 'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'error';

interface UpdateState {
  status: UpdateStatus;
  version?: string;
  body?: string;
  date?: string;
  error?: string;
}

function createUpdateStore() {
  const { subscribe, set, update } = writable<UpdateState>({
    status: 'idle',
  });

  return {
    subscribe,
    set,
    update,

    async checkForUpdates(): Promise<void> {
      set({ status: 'checking' });

      try {
        const update = await check();

        if (update) {
          set({
            status: 'available',
            version: update.version,
            body: update.body,
            date: update.date,
          });
        } else {
          set({ status: 'idle' });
        }
      } catch (err) {
        set({
          status: 'error',
          error: err instanceof Error ? err.message : String(err),
        });
      }
    },

    async downloadAndInstall(): Promise<void> {
      set({ status: 'downloading' });

      try {
        const update = await check();

        if (!update) {
          set({ status: 'idle' });
          return;
        }

        let downloaded = 0;
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              set({
                status: 'downloading',
                version: update.version,
                body: update.body,
                date: update.date,
              });
              break;
            case 'Progress':
              downloaded += event.data.chunkLength;
              break;
            case 'Finished':
              set({
                status: 'ready',
                version: update.version,
                body: update.body,
                date: update.date,
              });
              break;
          }
        });
      } catch (err) {
        set({
          status: 'error',
          error: err instanceof Error ? err.message : String(err),
        });
      }
    },

    async restart(): Promise<void> {
      await relaunch();
    },

    reset() {
      set({ status: 'idle' });
    },
  };
}

export const updateStore = createUpdateStore();
