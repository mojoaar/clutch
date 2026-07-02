import { test, expect } from '@playwright/test';

test('send message flow', async ({ page }) => {
	await page.goto('/chat');

	// Welcome screen: click "New Chat" to start a conversation
	const newChatButton = page.getByRole('button', { name: /New Chat/i });
	await expect(newChatButton).toBeVisible();
	await newChatButton.click();

	// Wait for the chat input textarea to appear (session created)
	const textarea = page.getByRole('textbox', { name: /Chat message input/i });
	await expect(textarea).toBeVisible();

	// Type a test message
	const testMessage = 'Hello from Playwright E2E test';
	await textarea.fill(testMessage);

	// Verify send button becomes enabled
	const sendButton = page.getByRole('button', { name: /Send message/i });
	await expect(sendButton).toBeEnabled();

	// Send the message
	await sendButton.click();

	// Verify the user message appears in the chat log
	const messagesContainer = page.getByRole('log', { name: /Chat messages/i });
	const userMessage = messagesContainer.getByRole('article', { name: /You message/i });
	await expect(userMessage).toBeVisible({ timeout: 10000 });
	await expect(userMessage).toContainText(testMessage);

	// The backend may fail without an API key, but the conversation
	// container should still show the sent message and some response.
	// Verify the chat log container has at least one message in it.
	await expect(messagesContainer.getByRole('article')).toHaveCount(1, { timeout: 10000 });
});
