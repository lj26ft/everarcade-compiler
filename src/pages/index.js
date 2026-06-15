import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const values = [
  ['Build Worlds', 'Create persistent digital worlds rather than one-off sessions.'],
  ['Own Worlds', 'World packages, history, and operations remain portable and sovereign.'],
  ['Verify Worlds', "Anyone can independently verify a world's history."],
  ['Operate Worlds', 'Infrastructure can be decentralized across operators and ecosystems.'],
];
const paths = [
  ['Developers', '/developers', 'Create your first world, learn World Contracts, and use RustRigs.'],
  ['Operators', '/operators', 'Host worlds, coordinate federation, and keep history recoverable.'],
  ['Players', '/players', 'Understand continuity, ownership, and persistent world communities.'],
  ['Contributors', '/contributors', 'Find the repository map, local build path, and contribution workflow.'],
];
export default function Home() {
  return <Layout title="Build Sovereign Worlds" description="EverArcade is a sovereign game runtime for persistent worlds.">
    <header className="hero heroBanner"><div className="container text--center">
      <p className="kicker">EverArcade</p><h1 className="hero__title">Build Sovereign Worlds</h1>
      <p className="hero__subtitle">EverArcade is a sovereign game runtime for persistent worlds. Developers build worlds. Operators run worlds. Players inhabit worlds.</p>
      <p>Worlds accumulate history, ownership, economies, and communities over time.</p>
      <div className="heroActions"><Link className="button button--primary button--lg" to="/docs/getting-started/">Get Started</Link><Link className="button button--secondary button--lg" to="/docs">Explore Documentation</Link><Link className="button button--outline button--secondary button--lg" to="/architecture">View Architecture</Link><Link className="button button--outline button--secondary button--lg" href="https://github.com/everarcade/everarcade-compiler">GitHub Repository</Link></div>
    </div></header>
    <main><section className="sectionBand"><div className="container"><h2>What is EverArcade?</h2><p>EverArcade gives teams a way to build worlds that keep going: history is preserved, state can move between operators, and communities can verify what happened without trusting a single server forever.</p><div className="valueGrid">{values.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div></div></section>
    <section className="sectionBand"><div className="container"><h2>Choose your path</h2><p>No protocol background required. Start with the role closest to you.</p><div className="pathGrid">{paths.map(([title, to, body]) => <Link className="cardPanel" to={to} key={title}><h3>{title}</h3><p>{body}</p></Link>)}</div></div></section></main>
  </Layout>;
}
