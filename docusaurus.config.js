const config = {
  title: 'EverArcade',
  tagline: 'Build Sovereign Worlds',
  url: 'https://everarcade.games',
  baseUrl: '/',
  favicon: 'img/favicon.ico',
  organizationName: 'everarcade',
  projectName: 'everarcade-compiler',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  markdown: { mermaid: true },
  themes: ['@docusaurus/theme-mermaid'],
  presets: [
    [
      'classic',
      {
        docs: {
          path: 'docs',
          routeBasePath: 'docs',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/everarcade/everarcade-compiler/tree/main/',
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
        { to: '/concepts', label: 'Concepts', position: 'left' },
        { to: '/developers', label: 'Developers', position: 'left' },
        { to: '/operators', label: 'Operators', position: 'left' },
        { to: '/players', label: 'Players', position: 'left' },
        { to: '/architecture', label: 'Architecture', position: 'left' },
        { to: '/docs', label: 'Documentation', position: 'left' },
        { to: '/docs/architecture/roadmap/v1-roadmap', label: 'Roadmap', position: 'left' },
        { href: 'https://github.com/everarcade/everarcade-compiler', label: 'GitHub', position: 'right' },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        { title: 'Start', items: [{ label: 'Get Started', to: '/docs/getting-started/' }, { label: 'Concepts', to: '/concepts' }] },
        { title: 'Paths', items: [{ label: 'Developers', to: '/developers' }, { label: 'Operators', to: '/operators' }, { label: 'Players', to: '/players' }, { label: 'Contributors', to: '/contributors' }] },
        { title: 'Reference', items: [{ label: 'Architecture', to: '/architecture' }, { label: 'Whitepaper Archive', to: '/docs/archive/whitepaper/' }, { label: 'GitHub', href: 'https://github.com/everarcade/everarcade-compiler' }] },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} EverArcade. Built as source-controlled documentation.`,
    }
  },
};

module.exports = config;
