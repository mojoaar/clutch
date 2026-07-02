import { writable, get } from 'svelte/store';
import type { Toast, Banner, ToastVariant, BannerVariant } from '$lib/services/errors';

function createId(): string {
  return crypto.randomUUID();
}

export const toasts = writable<Toast[]>([]);
export const banners = writable<Banner[]>([]);

let toastTimers = new Map<string, ReturnType<typeof setTimeout>>();

export function addToast(
  message: string,
  variant: ToastVariant = 'info',
  durationMs = 8000,
  action?: Toast['action'],
): string {
  const id = createId();
  const toast: Toast = { id, message, variant, dismissible: true, durationMs, action };
  toasts.update((t) => [...t, toast]);

  if (durationMs > 0) {
    const timer = setTimeout(() => dismissToast(id), durationMs);
    toastTimers.set(id, timer);
  }

  return id;
}

export function dismissToast(id: string): void {
  const timer = toastTimers.get(id);
  if (timer) {
    clearTimeout(timer);
    toastTimers.delete(id);
  }
  toasts.update((t) => t.filter((toast) => toast.id !== id));
}

export function addBanner(
  message: string,
  variant: BannerVariant = 'info',
  action?: Banner['action'],
): string {
  const id = createId();
  banners.update((b) => [...b, { id, message, variant, action }]);
  return id;
}

export function dismissBanner(id: string): void {
  banners.update((b) => b.filter((banner) => banner.id !== id));
}
