export interface CuratedSkill {
  id: string;
  name: string;
  description: string;
  source: string;
  branch: string;
  category: string;
  author: string;
}

export const CURATED_SKILLS: CuratedSkill[] = [
  {
    id: 'skills/brainstorming',
    name: 'Brainstorming',
    description: 'Explores user intent, requirements and design before implementation. Must be used before any creative work.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/finishing-a-development-branch',
    name: 'Finishing a Development Branch',
    description: 'Guides completion of development work by presenting structured options for merge, PR, or cleanup.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/receiving-code-review',
    name: 'Receiving Code Review',
    description: 'Use when receiving code review feedback, before implementing suggestions. Requires technical rigor and verification.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/requesting-code-review',
    name: 'Requesting Code Review',
    description: 'Use when completing tasks, implementing major features, or before merging to verify work meets requirements.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/systematic-debugging',
    name: 'Systematic Debugging',
    description: 'Use when encountering any bug, test failure, or unexpected behavior, before proposing fixes.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/test-driven-development',
    name: 'Test-Driven Development',
    description: 'Use when implementing any feature or bugfix, before writing implementation code.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/using-git-worktrees',
    name: 'Using Git Worktrees',
    description: 'Ensures an isolated workspace exists via native tools or git worktree fallback before starting feature work.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/verification-before-completion',
    name: 'Verification Before Completion',
    description: 'Use when about to claim work is complete, fixed, or passing. Requires running verification commands before asserting success.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/writing-plans',
    name: 'Writing Plans',
    description: 'Use when you have a spec or requirements for a multi-step task, before touching code.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/writing-skills',
    name: 'Writing Skills',
    description: 'Use when creating new skills, editing existing skills, or verifying skills work before deployment.',
    source: 'obra/superpowers',
    branch: 'main',
    category: 'Workflow',
    author: 'Superpowers',
  },
  {
    id: 'skills/docx',
    name: 'DOCX',
    description: 'Create, edit, and read Word documents.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Document',
    author: 'Anthropic',
  },
  {
    id: 'skills/pdf',
    name: 'PDF',
    description: 'Read, extract, and manipulate PDF files.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Document',
    author: 'Anthropic',
  },
  {
    id: 'skills/pptx',
    name: 'PPTX',
    description: 'Create and edit PowerPoint presentations.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Document',
    author: 'Anthropic',
  },
  {
    id: 'skills/xlsx',
    name: 'XLSX',
    description: 'Create, edit, and analyze Excel spreadsheets.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Document',
    author: 'Anthropic',
  },
  {
    id: 'skills/internal-comms',
    name: 'Internal Comms',
    description: 'Write status updates, memos, and newsletters.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Writing',
    author: 'Anthropic',
  },
  {
    id: 'skills/doc-coauthoring',
    name: 'Doc Coauthoring',
    description: 'Co-author documents with AI assistance.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Writing',
    author: 'Anthropic',
  },
  {
    id: 'skills/frontend-design',
    name: 'Frontend Design',
    description: 'UI/UX design guidance and frontend best practices.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Design',
    author: 'Anthropic',
  },
  {
    id: 'skills/canvas-design',
    name: 'Canvas Design',
    description: 'Visual design creation and layout guidance.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Design',
    author: 'Anthropic',
  },
  {
    id: 'skills/brand-guidelines',
    name: 'Brand Guidelines',
    description: 'Brand identity and style guide creation.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Design',
    author: 'Anthropic',
  },
  {
    id: 'skills/theme-factory',
    name: 'Theme Factory',
    description: 'Generate theme artifacts and visual styles.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Design',
    author: 'Anthropic',
  },
  {
    id: 'skills/mcp-builder',
    name: 'MCP Builder',
    description: 'Build MCP servers for extended AI capabilities.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Development',
    author: 'Anthropic',
  },
  {
    id: 'skills/webapp-testing',
    name: 'Webapp Testing',
    description: 'Test and audit web apps via Playwright.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Development',
    author: 'Anthropic',
  },
  {
    id: 'skills/web-artifacts-builder',
    name: 'Web Artifacts Builder',
    description: 'Build reusable web components and artifacts.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Development',
    author: 'Anthropic',
  },
  {
    id: 'skills/skill-creator',
    name: 'Skill Creator',
    description: 'Create custom skills for extended AI capabilities.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Development',
    author: 'Anthropic',
  },
  {
    id: 'skills/algorithmic-art',
    name: 'Algorithmic Art',
    description: 'Create generative art with p5.js.',
    source: 'anthropics/skills',
    branch: 'main',
    category: 'Creative',
    author: 'Anthropic',
  },
];

export function searchCurated(query: string): CuratedSkill[] {
  const words = query.toLowerCase().trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) return [...CURATED_SKILLS];

  return CURATED_SKILLS
    .map((skill) => {
      let score = 0;
      const name = skill.name.toLowerCase();
      const desc = skill.description.toLowerCase();
      const cat = skill.category.toLowerCase();
      const author = skill.author.toLowerCase();

      for (const word of words) {
        if (name.includes(word)) score += 3;
        if (desc.includes(word)) score += 2;
        if (cat.includes(word)) score += 1;
        if (author.includes(word)) score += 1;
      }
      return { skill, score };
    })
    .filter(({ score }) => score > 0)
    .sort((a, b) => b.score - a.score)
    .map(({ skill }) => skill);
}
