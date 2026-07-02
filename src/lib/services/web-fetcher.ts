import { invoke } from "@tauri-apps/api/core";

export interface FetchOptions {
  timeoutSecs?: number;
  maxSizeBytes?: number;
  mode?: "raw" | "markdown" | "info";
  headers?: [string, string][];
  followRedirects?: number;
}

export interface FetchResult {
  url: string;
  content: string;
  statusCode: number;
  contentType?: string;
  contentLength?: number;
  title?: string;
  description?: string;
}

export interface BatchFetchResult {
  results: (FetchResult | null)[];
  errors: [string, string][];
}

export interface WebpageInfo {
  url: string;
  title?: string;
  description?: string;
  statusCode: number;
  contentType?: string;
  contentLength?: number;
}

export async function fetchUrl(
  url: string,
  options?: FetchOptions,
): Promise<FetchResult> {
  return invoke<FetchResult>("fetch_url", { url, options });
}

export async function batchFetch(
  urls: string[],
  options?: FetchOptions,
): Promise<BatchFetchResult> {
  return invoke<BatchFetchResult>("batch_fetch", { urls, options });
}

export async function fetchGithubReadme(repoUrl: string): Promise<FetchResult> {
  return invoke<FetchResult>("fetch_github_readme", { repoUrl });
}

export async function fetchWebpageInfo(url: string): Promise<WebpageInfo> {
  return invoke<WebpageInfo>("fetch_webpage_info", { url });
}
