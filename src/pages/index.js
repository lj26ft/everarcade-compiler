import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const values = [
  ['Worlds', 'Persistent places with memory, economies, communities, and ownership.'],
  ['Continuity', 'History stays legible as worlds grow, recover, and move between operators.'],
  ['Ownership', 'World packages, history, and operations are designed to remain portable.'],
  ['Verification', 'Communities can independently check what happened without trusting one server forever.'],
];

const paths = [
  ['Explore Worlds', '/worlds', 'Understand what makes a world different from a temporary game session.'],
  ['Developers', '/developers', 'Create, deploy, verify, operate, and scale your first world.'],
  ['Operators', '/operators', 'Host worlds and preserve continuity for communities.'],
  ['Players', '/players', 'See why persistent worlds change the meaning of play.'],
];

export default function Home() {
  return <Layout title="Build Worlds. Not Just Games." description="EverArcade is a sovereign runtime for persistent worlds.">
    <header className="hero heroBanner"><div className="container text--center">
      <p className="kicker">EverArcade</p>
      <h1 className="hero__title">Build Worlds.<br />Not Just Games.</h1>
      <p className="hero__subtitle">EverArcade is a sovereign runtime for persistent worlds.</p>
      <p className="heroStatement">Developers create them. Operators run them. Players give them life.</p>
      <div className="heroActions">
        <Link className="button button--primary button--lg" to="/worlds">Explore Worlds</Link>
        <Link className="button button--secondary button--lg" to="/docs">Documentation</Link>
        <Link className="button button--outline button--secondary button--lg" href="https://github.com/everarcade/everarcade-compiler">GitHub</Link>
      </div>
    </div></header>
    <main>
      <section className="sectionBand"><div className="container"><h2>What is EverArcade?</h2><p>EverArcade helps teams create worlds that keep going: history is preserved, state can move between operators, and communities can verify what happened without trusting a single server forever.</p><div className="valueGrid">{values.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div></div></section>
      <section className="sectionBand"><div className="container"><h2>Choose your path</h2><p>No protocol background required. Start with the role closest to you.</p><div className="pathGrid">{paths.map(([title, to, body]) => <Link className="cardPanel" to={to} key={title}><h3>{title}</h3><p>{body}</p></Link>)}</div></div></section>
      <section className="sectionBand"><div className="container"><h2>From vision to implementation</h2><p>The website introduces the public story and stakeholder journeys. The documentation portal remains the source for commands, formats, runtime details, and reference material.</p><div className="heroActions"><Link className="button button--primary" to="/founding-developers">Founding Developers</Link><Link className="button button--secondary" to="/roadmap">Roadmap</Link><Link className="button button--secondary" to="/community">Community</Link></div></div></section>
    </main>
  </Layout>;
}
