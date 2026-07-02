import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

export interface ExportOptions {
  format: 'markdown' | 'text' | 'json' | 'html';
  includeMetadata: boolean;
  includeTimestamps: boolean;
  includeProviderInfo: boolean;
  includeSystemPrompt: boolean;
}

export async function exportSession(sessionId: string, options: ExportOptions): Promise<void> {
  const content = await invoke<string>('export_session', {
    sessionId,
    options,
  });

  const extensions: Record<string, string> = {
    markdown: 'md',
    text: 'txt',
    json: 'json',
    html: 'html',
  };

  const ext = extensions[options.format] || 'md';

  const path = await save({
    filters: [
      {
        name: options.format.toUpperCase(),
        extensions: [ext],
      },
    ],
    defaultPath: `chat-export.${ext}`,
  });

  if (path) {
    await writeTextFile(path, content);
  }
}
