export type AppError =
  | { kind: 'api'; provider: string; status: number; message: string }
  | { kind: 'network'; detail: string }
  | { kind: 'file_system'; operation: string; path: string; detail: string }
  | { kind: 'parse'; source: string; detail: string }
  | { kind: 'validation'; field: string; detail: string }
  | { kind: 'timeout'; operation: string }
  | { kind: 'stream_interrupted'; sessionId: string; partialContent: string };

export type ToastVariant = 'info' | 'success' | 'warning' | 'error';
export type BannerVariant = 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  message: string;
  variant: ToastVariant;
  dismissible: boolean;
  durationMs: number;
  action?: { label: string; onClick: () => void };
}

export interface Banner {
  id: string;
  message: string;
  variant: BannerVariant;
  action?: { label: string; onClick: () => void };
}

export function errorMessage(err: AppError): string {
  switch (err.kind) {
    case 'api':
      return `${err.provider} API error (${err.status}): ${err.message}`;
    case 'network':
      return `Network error: ${err.detail}`;
    case 'file_system':
      return `File system error (${err.operation} on ${err.path}): ${err.detail}`;
    case 'parse':
      return `Parse error (${err.source}): ${err.detail}`;
    case 'validation':
      return `Invalid ${err.field}: ${err.detail}`;
    case 'timeout':
      return `${err.operation} timed out`;
    case 'stream_interrupted':
      return `Response stream interrupted for session ${err.sessionId}`;
  }
}

export function errorToastVariant(err: AppError): ToastVariant {
  switch (err.kind) {
    case 'api':
      return err.status >= 500 ? 'error' : 'warning';
    case 'network':
    case 'timeout':
      return 'warning';
    case 'stream_interrupted':
      return 'warning';
    default:
      return 'error';
  }
}

const RETRYABLE_ERRORS = new Set(['network', 'timeout', 'stream_interrupted']);

export function isRetryable(err: AppError): boolean {
  return RETRYABLE_ERRORS.has(err.kind);
}

export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  isRetryable: (err: unknown) => boolean,
  maxRetries = 3,
  baseDelayMs = 1000,
): Promise<T> {
  let lastError: unknown;
  for (let i = 0; i <= maxRetries; i++) {
    try {
      return await fn();
    } catch (e) {
      lastError = e;
      if (i === maxRetries || !isRetryable(e)) {
        throw e;
      }
      const delay = baseDelayMs * Math.pow(2, i);
      await new Promise((resolve) => setTimeout(resolve, delay));
    }
  }
  throw lastError;
}
