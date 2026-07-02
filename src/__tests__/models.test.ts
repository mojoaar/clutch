import { describe, it, expect, beforeEach } from 'vitest';
import {
	getDefaultModels,
	groupModelsByCategory,
	clearCache,
	type ModelInfo,
} from '$lib/services/models';

describe('getDefaultModels', () => {
	it('returns models for deepseek provider', () => {
		const models = getDefaultModels('deepseek');
		expect(models.length).toBeGreaterThan(0);
		expect(models.every((m) => m.provider === 'deepseek')).toBe(true);
	});

	it('returns models for opencode_go provider', () => {
		const models = getDefaultModels('opencode_go');
		expect(models.length).toBeGreaterThan(0);
		const categories = new Set(models.map((m) => m.category));
		expect(categories.has('MiniMax')).toBe(true);
		expect(categories.has('Kimi')).toBe(true);
	});

	it('returns models for opencode_zen provider', () => {
		const models = getDefaultModels('opencode_zen');
		expect(models.length).toBeGreaterThan(0);
		const hasClaude = models.some((m) => m.category === 'Claude');
		expect(hasClaude).toBe(true);
	});

	it('all returned models have required fields', () => {
		const models = getDefaultModels('deepseek');
		for (const m of models) {
			expect(m.id).toBeTruthy();
			expect(m.name).toBeTruthy();
			expect(m.provider).toBeTruthy();
		}
	});

	it('returns empty array for unknown provider', () => {
		expect(getDefaultModels('unknown_provider')).toEqual([]);
	});
});

describe('groupModelsByCategory', () => {
	it('groups models by category', () => {
		const models: ModelInfo[] = [
			{ id: 'a', name: 'A', provider: 'opencode_go', category: 'Kimi', context_length: null },
			{ id: 'b', name: 'B', provider: 'opencode_go', category: 'Kimi', context_length: null },
			{ id: 'c', name: 'C', provider: 'opencode_go', category: 'GLM', context_length: null },
		];
		const groups = groupModelsByCategory(models);
		expect(groups.get('Kimi')).toHaveLength(2);
		expect(groups.get('GLM')).toHaveLength(1);
	});

	it('places null categories under "Other"', () => {
		const models: ModelInfo[] = [
			{ id: 'a', name: 'A', provider: 'p', category: null, context_length: null },
		];
		const groups = groupModelsByCategory(models);
		expect(groups.get('Other')).toHaveLength(1);
	});

	it('returns an empty map for an empty array', () => {
		const groups = groupModelsByCategory([]);
		expect(groups.size).toBe(0);
	});
});

describe('clearCache', () => {
	beforeEach(() => {
		clearCache();
	});

	it('clears all caches when called without provider', () => {
		const models = getDefaultModels('deepseek');
		expect(models.length).toBeGreaterThan(0);
		clearCache();
		// cache state is internal, but we ensure no throw
	});

	it('clears specific provider cache', () => {
		// Verify no errors clearing a specific provider
		clearCache('deepseek');
	});
});
