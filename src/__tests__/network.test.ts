import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";

vi.mock("$lib/stores/toast", () => ({
  addToast: vi.fn(),
}));

vi.mock("$lib/i18n/i18n-svelte", () => {
  let value: any = {
    networkStatus: {
      sendFailedAfterRetries: () => "Send failed after retries",
    },
  };
  const subs: Array<(v: any) => void> = [];
  const store = {
    subscribe(fn: (v: any) => void) {
      fn(value);
      subs.push(fn);
      return () => {};
    },
    set(v: any) {
      value = v;
      subs.forEach((f) => f(v));
    },
    update(fn: (v: any) => any) {
      store.set(fn(value));
    },
  };
  return { default: store };
});

import { network, registerQueueProcessor } from "$lib/services/network";
import { addToast } from "$lib/stores/toast";

describe("network store", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    network.clearQueue();
  });

  describe("enqueue", () => {
    it("adds item to queue with unique id", () => {
      const id = network.enqueue("sendMessage", ["arg1"], 3);
      const items = get(network.queue);
      expect(items).toHaveLength(1);
      expect(items[0].id).toBe(id);
      expect(items[0].operation).toBe("sendMessage");
      expect(items[0].args).toEqual(["arg1"]);
      expect(items[0].maxAttempts).toBe(3);
      expect(items[0].attempts).toBe(0);
    });

    it("generates unique ids for each item", () => {
      const id1 = network.enqueue("a", [], 3);
      const id2 = network.enqueue("b", [], 3);
      expect(id1).not.toBe(id2);
    });

    it("overflows at 50 items with FIFO eviction", () => {
      for (let i = 0; i < 50; i++) {
        network.enqueue("op", [`arg-${i}`], 3);
      }
      expect(get(network.queue)).toHaveLength(50);

      const evicted = get(network.queue)[0].args[0];
      network.enqueue("overflow", ["newest"], 3);

      const items = get(network.queue);
      expect(items).toHaveLength(50);
      expect(items[0].args[0]).not.toBe(evicted);
      expect(items[49].args[0]).toBe("newest");
    });

    it("sets createdAt timestamp", () => {
      const before = Date.now();
      network.enqueue("test", [], 3);
      const item = get(network.queue)[0];
      expect(item.createdAt).toBeGreaterThanOrEqual(before);
      expect(item.createdAt).toBeLessThanOrEqual(Date.now());
    });
  });

  describe("dequeue", () => {
    it("removes item by id", () => {
      const id1 = network.enqueue("a", [], 3);
      const id2 = network.enqueue("b", [], 3);

      network.dequeue(id1);

      const items = get(network.queue);
      expect(items).toHaveLength(1);
      expect(items[0].id).toBe(id2);
    });

    it("is a no-op for unknown id", () => {
      network.enqueue("a", [], 3);
      network.dequeue("nonexistent");
      expect(get(network.queue)).toHaveLength(1);
    });
  });

  describe("clearQueue", () => {
    it("empties queue and returns true", () => {
      network.enqueue("a", [], 3);
      network.enqueue("b", [], 3);
      const result = network.clearQueue();
      expect(result).toBe(true);
      expect(get(network.queue)).toEqual([]);
    });

    it("returns true on empty queue", () => {
      const result = network.clearQueue();
      expect(result).toBe(true);
    });
  });

  describe("queueSize", () => {
    it("tracks queue length as a derived store", () => {
      expect(get(network.queueSize)).toBe(0);

      network.enqueue("a", [], 3);
      expect(get(network.queueSize)).toBe(1);

      network.enqueue("b", [], 3);
      expect(get(network.queueSize)).toBe(2);

      network.clearQueue();
      expect(get(network.queueSize)).toBe(0);
    });
  });

  describe("processQueue", () => {
    it("increments attempt counter on retryable failure", async () => {
      vi.useFakeTimers();

      let callCount = 0;
      registerQueueProcessor(async () => {
        callCount++;
        throw new Error("Transient failure");
      });

      const id = network.enqueue("test", ["arg"], 5);
      const promise = network.processQueue();
      await vi.advanceTimersByTimeAsync(1000);
      await promise;

      const items = get(network.queue);
      expect(items).toHaveLength(1);
      expect(items[0].id).toBe(id);
      expect(items[0].attempts).toBe(1);
      expect(callCount).toBe(1);

      vi.useRealTimers();
    });

    it("dequeues item when maxAttempts exhausted", async () => {
      registerQueueProcessor(async () => {
        throw new Error("Permanent failure");
      });

      network.enqueue("test", ["arg"], 1);

      await network.processQueue();

      expect(get(network.queue)).toHaveLength(0);
      expect(addToast).toHaveBeenCalledOnce();
    });

    it("dequeues item after successful processing", async () => {
      registerQueueProcessor(async () => {});

      network.enqueue("test", ["arg"], 3);
      await network.processQueue();

      expect(get(network.queue)).toHaveLength(0);
    });

    it("does nothing if no processor registered", async () => {
      (registerQueueProcessor as any)(null);

      network.enqueue("test", [], 3);
      await network.processQueue();
      expect(get(network.queue)).toHaveLength(1);
    });

    it("does nothing if queue is empty", async () => {
      registerQueueProcessor(async () => {
        throw new Error("Should not be called");
      });
      await network.processQueue();
      expect(true).toBe(true);
    });

    it("processes multiple items in sequence successfully", async () => {
      const processed: string[] = [];
      registerQueueProcessor(async (item: any) => {
        processed.push(item.operation);
      });

      network.enqueue("op1", [], 3);
      network.enqueue("op2", [], 3);
      network.enqueue("op3", [], 3);

      await network.processQueue();

      expect(processed).toEqual(["op1", "op2", "op3"]);
      expect(get(network.queue)).toHaveLength(0);
    });
  });
});
