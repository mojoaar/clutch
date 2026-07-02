export function formatShortcutForDisplay(shortcut: string): string {
    const isMac = typeof navigator !== 'undefined' && navigator.platform.includes('Mac');
    return shortcut
        .replace('CmdOrCtrl', isMac ? 'Cmd' : 'Ctrl')
        .replace(/Key/g, '')
        .replace(/\+/g, ' + ');
}
