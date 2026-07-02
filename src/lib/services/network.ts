import { writable, derived, get } from "svelte/store";
import { addToast } from "$lib/stores/toast";
import LL from "$lib/i18n/i18n-svelte";

export type NetworkStatus = "online" | "degraded" | "offline";

export interface RetryItem {
  id: string;
  operation: string;
  args: any[];
  attempts: number;
  maxAttempts: number;
  createdAt: number;
}

type QueueProcessor = (item: RetryItem) => Promise<void>;

let _queueProcessor: QueueProcessor | null = null;
let _periodicTimer: ReturnType<typeof setInterval> | null = null;
let _processing = false;

export function registerQueueProcessor(fn: QueueProcessor) {
  _queueProcessor = fn;
}

function createNetworkStore() {
  const status = writable<NetworkStatus>("online");
  const queue = writable<RetryItem[]>([]);
  const queueSize = derived(queue, ($queue) => $queue.length);

  function setStatus(s: NetworkStatus) {
    status.set(s);
  }

  function handleOnline() {
    setStatus("online");
    if (get(queue).length > 0) {
      processQueue();
    }
  }

  function handleOffline() {
    setStatus("offline");
  }

  function startPeriodicCheck() {
    stopPeriodicCheck();
    _periodicTimer = setInterval(() => {
      if (navigator.onLine) {
        if (get(status) !== "online") {
          setStatus("online");
        }
      } else {
        setStatus("offline");
      }
    }, 30_000);
  }

  function stopPeriodicCheck() {
    if (_periodicTimer !== null) {
      clearInterval(_periodicTimer);
      _periodicTimer = null;
    }
  }

  function startMonitoring() {
    if (typeof window !== "undefined") {
      window.addEventListener("online", handleOnline);
      window.addEventListener("offline", handleOffline);

      if (!navigator.onLine) {
        setStatus("offline");
      }

      startPeriodicCheck();
    }
  }

  function stopMonitoring() {
    if (typeof window !== "undefined") {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    }
    stopPeriodicCheck();
  }

  async function processQueue() {
    if (_processing) return;
    if (!_queueProcessor) return;

    _processing = true;
    const items = get(queue);
    if (items.length === 0) {
      _processing = false;
      return;
    }

    for (const item of items) {
      try {
        await _queueProcessor(item);
        dequeue(item.id);
      } catch (error) {
        item.attempts++;
        if (item.attempts < item.maxAttempts) {
          const delay = Math.pow(2, item.attempts - 1) * 1000;
          await new Promise((resolve) => setTimeout(resolve, delay));
        } else {
          dequeue(item.id);
          addToast(
            get(LL).networkStatus.sendFailedAfterRetries(),
            "error",
            8000,
          );
          console.debug("Queue item exhausted retries:", {
            id: item.id,
            operation: item.operation,
          });
        }
      }
    }

    _processing = false;
  }

  function enqueue(operation: string, args: any[], maxAttempts = 3): string {
    const id = crypto.randomUUID();
    const item: RetryItem = {
      id,
      operation,
      args,
      attempts: 0,
      maxAttempts,
      createdAt: Date.now(),
    };

    queue.update((items) => {
      if (items.length >= 50) {
        return [...items.slice(1), item];
      }
      return [...items, item];
    });

    return id;
  }

  function dequeue(id: string) {
    queue.update((items) => items.filter((i) => i.id !== id));
  }

  function clearQueue(): boolean {
    queue.set([]);
    return true;
  }

  function retryAll() {
    processQueue();
  }

  return {
    status,
    queue,
    queueSize,
    startMonitoring,
    stopMonitoring,
    startPeriodicCheck,
    stopPeriodicCheck,
    processQueue,
    enqueue,
    dequeue,
    clearQueue,
    retryAll,
  };
}

export const network = createNetworkStore();
