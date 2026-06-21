const config = {
  title: 'EverArcade',
  tagline: 'Build Sovereign Worlds',
  url: 'https://everarcade.games',
  baseUrl: '/',
  favicon: 'img/favicon.ico',
  organizationName: 'everarcade',
  projectName: 'everarcade-compiler',
  onBrokenLinks: 'throw',
  markdown: {
    mermaid: true,
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },
  themes: ['@docusaurus/theme-mermaid'],
  presets: [
    [
      'classic',
      {
        docs: {
          path: 'docs',
          routeBasePath: 'docs',
          sidebarPath: require.resolve('./sidebars.js'),
          showLastUpdateTime: true,
          showLastUpdateAuthor: true,
        },
        blog: false,
        theme: { customCss: require.resolve('./src/css/custom.css') },
      },
    ],
  ],
  plugins: [
    [
      '@easyops-cn/docusaurus-search-local',
      { hashed: true, docsRouteBasePath: '/docs', indexBlog: false },
    ],
  ],
  themeConfig: {
    image: 'img/everarcade-social-card.png',
    navbar: {
      title: 'EverArcade',
      items: [
        { to: '/', label: 'Home', position: 'left' },
        { to: '/vision', label: 'Vision', position: 'left' },
        { to: '/worlds', label: 'Worlds', position: 'left' },
        { to: '/developers', label: 'Developers', position: 'left' },
        { to: '/operators', label: 'Operators', position: 'left' },
        { to: '/players', label: 'Players', position: 'left' },
        { to: '/continuity-engine', label: 'Continuity Engine', position: 'left' },
        { to: '/docs', label: 'Documentation', position: 'left' },
        { to: '/roadmap', label: 'Roadmap', position: 'left' },
        { to: '/open-source', label: 'Open Source Repo — Coming Soon', position: 'right' },
        { to: '/community', label: 'Community', position: 'right' },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        { title: 'Start', items: [{ label: 'Vision Manifesto', to: '/vision' }, { label: 'Explore Worlds', to: '/worlds' }, { label: 'Founding Developers', to: '/founding-developers' }] },
        { title: 'Paths', items: [{ label: 'Developers', to: '/developers' }, { label: 'Operators', to: '/operators' }, { label: 'Players', to: '/players' }, { label: 'Contributors', to: '/contributors' }] },
        { title: 'Reference', items: [{ label: 'Continuity Engine', to: '/continuity-engine' }, { label: 'Roadmap', to: '/roadmap' }, { label: 'Open Source Repo — Coming Soon', to: '/open-source' }] },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} EverArcade. Built as source-controlled documentation.`,
    }
  },
};

module.exports = config;
