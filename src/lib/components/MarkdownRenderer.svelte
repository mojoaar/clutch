<script lang="ts">
  import { Marked } from 'marked';
  import DOMPurify from 'dompurify';
  import hljs from 'highlight.js';
  import '../styles/highlight-theme.css';

  interface Props {
    content: string;
  }

  let { content }: Props = $props();

  let codeBlockCache = new Map<string, { language: string; code: string }>();
  let copiedStates = new Map<string, boolean>();

  const marked = new Marked({
    gfm: true,
    breaks: true,
    renderer: {
      code({ text, lang }: { text: string; lang?: string }) {
        const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
        const highlighted = hljs.highlight(text, { language }).value;
        return `<pre><code class="hljs language-${language}">${highlighted}</code></pre>`;
      },
    },
  });

  function processMarkdown(text: string): string {
    try {
      const raw = marked.parse(text) as string;
      const sanitized = DOMPurify.sanitize(raw, {
        ALLOWED_TAGS: [
          'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
          'p', 'br', 'strong', 'em', 'del', 'u', 'a',
          'ul', 'ol', 'li',
          'pre', 'code', 'blockquote',
          'table', 'thead', 'tbody', 'tr', 'th', 'td',
          'img', 'span', 'div',
          'hr', 'sub', 'sup',
        ],
        ALLOWED_ATTR: ['href', 'src', 'alt', 'class', 'target', 'rel', 'title'],
      });

      codeBlockCache.clear();
      let blockIndex = 0;

      return sanitized.replace(
        /<pre><code class="hljs language-(\w+)">([\s\S]*?)<\/code><\/pre>/g,
        (_match: string, language: string, code: string) => {
          const id = `code-${blockIndex++}`;
          codeBlockCache.set(id, { language, code });

          return `<div class="code-block-wrapper" data-block-id="${id}">
          <div class="code-block-header">
            <span class="code-block-lang">${language}</span>
            <button class="code-block-copy" data-action="copy" title={$LL.copy()}>
              <span data-copy-icon="${id}">Copy</span>
            </button>
          </div>
          <pre data-block-id="${id}"><code class="hljs language-${language}">${code}</code></pre>
        </div>`;
        },
      );
    } catch {
      return DOMPurify.sanitize(content);
    }
  }

  let finalHtml = $derived(processMarkdown(content));

  function handleCodeCopy(e: MouseEvent) {
    const target = e.target as HTMLElement;
    const button = target.closest('.code-block-copy') as HTMLElement | null;
    if (!button) return;
    const wrapper = button.closest('.code-block-wrapper') as HTMLElement | null;
    if (!wrapper) return;
    const blockId = wrapper.dataset.blockId;
    if (!blockId) return;
    const entry = codeBlockCache.get(blockId);
    if (!entry) return;
    navigator.clipboard.writeText(entry.code).then(() => {
      const icon = wrapper.querySelector('[data-copy-icon]') as HTMLElement | null;
      if (icon) { icon.textContent = 'Copied!'; setTimeout(() => { icon.textContent = 'Copy'; }, 2000); }
    });
  }
</script>

{#if content}
  <div class="markdown" onclick={handleCodeCopy}>{@html finalHtml}</div>
{/if}

<style>
  .markdown {
    font-size: 14px;
    line-height: 1.7;
    color: var(--text);
    word-wrap: break-word;
    overflow-wrap: break-word;
  }

  .markdown :global(p) {
    margin: 0 0 12px;
  }

  .markdown :global(p:last-child) {
    margin-bottom: 0;
  }

  .markdown :global(h1),
  .markdown :global(h2),
  .markdown :global(h3),
  .markdown :global(h4) {
    margin: 20px 0 10px;
    font-weight: 600;
    line-height: 1.3;
  }

  .markdown :global(h1) { font-size: 1.5em; }
  .markdown :global(h2) { font-size: 1.3em; }
  .markdown :global(h3) { font-size: 1.15em; }
  .markdown :global(h4) { font-size: 1em; }

  .markdown :global(h1:first-child),
  .markdown :global(h2:first-child),
  .markdown :global(h3:first-child),
  .markdown :global(h4:first-child) {
    margin-top: 0;
  }

  .markdown :global(ul),
  .markdown :global(ol) {
    margin: 0 0 12px;
    padding-left: 20px;
  }

  .markdown :global(li) {
    margin-bottom: 4px;
  }

  .markdown :global(a) {
    color: var(--primary);
    text-decoration: none;
  }

  .markdown :global(a:hover) {
    text-decoration: underline;
  }

  .markdown :global(blockquote) {
    margin: 0 0 12px;
    padding: 4px 14px;
    border-left: 3px solid var(--primary);
    color: var(--text-muted);
  }

  .markdown :global(table) {
    width: 100%;
    margin: 0 0 12px;
    border-collapse: collapse;
    font-size: 13px;
  }

  .markdown :global(th),
  .markdown :global(td) {
    padding: 8px 12px;
    border: 1px solid var(--border);
    text-align: left;
  }

  .markdown :global(th) {
    background: var(--surface);
    font-weight: 600;
  }

  .markdown :global(hr) {
    margin: 20px 0;
    border: none;
    border-top: 1px solid var(--border);
  }

  .markdown :global(img) {
    max-width: 100%;
    border-radius: 8px;
  }

  .markdown :global(.code-block-wrapper) {
    margin: 0 0 12px;
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid var(--border);
    background: var(--code-bg);
  }

  .markdown :global(.code-block-header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }

  .markdown :global(.code-block-lang) {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dim);
  }

  .markdown :global(.code-block-copy) {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .markdown :global(.code-block-copy:hover) {
    background: var(--surface-hover);
    color: var(--text);
  }

  .markdown :global(pre) {
    margin: 0;
    padding: 16px;
    overflow-x: auto;
    font-family: 'JetBrains Mono', monospace;
    font-size: 13px;
    line-height: 1.6;
  }

  .markdown :global(code) {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.9em;
  }

  .markdown :global(:not(pre) > code) {
    padding: 2px 6px;
    border-radius: 4px;
    background: var(--surface-hover);
    color: var(--accent);
  }

  .markdown :global(.hljs) {
    background: transparent;
    color: var(--text);
  }
</style>
