import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const values = [
  ['Worlds', 'Persistent places with memory, economies, communities, and ownership.'],
  ['Continuity', 'History stays legible as worlds grow, recover, and move between operators.'],
  ['Ownership', 'World packages, history, and operations are designed to remain portable.'],
  ['Verification', 'Communities can independently check what happened without trusting one server forever.'],
];

const traditional = [
  'Worlds die when servers shut down.',
  'Communities depend on platform owners.',
  'History resets between products.',
  'Infrastructure and game rules are tightly coupled.',
];

const everarcade = ['Portable', 'Persistent', 'Verifiable', 'Operator-hosted', 'Community-driven'];

const sovereigntyStack = [
  ['World Contracts', 'Developers define world rules, metadata, upgrade boundaries, and continuity expectations outside one host.'],
  ['Deterministic Runtime', 'Portable execution makes the same accepted inputs produce the same world transitions.'],
  ['Replay Verification', 'Communities can independently check history windows instead of trusting one server narrative.'],
  ['Checkpoint Restore', 'Operators can recover or migrate from known state without pretending history began again.'],
  ['Portable World Packages', 'Rules, content references, and operating expectations travel together across distribution paths.'],
  ['Operator Hosting', 'World operation can move beyond a single platform owner or permanent infrastructure provider.'],
  ['XRPL / Xahau Anchoring', 'Ownership, settlement, and external boundary records can anchor outside the runtime platform.'],
  ['Evernode Compute', 'Independent compute gives operators another path for running verifiable world infrastructure.'],
];

const sovereigntyOutcomes = [
  'developer-owned worlds',
  'portable execution',
  'independent verification',
  'operator independence',
  'less platform lock-in',
  'long-lived world continuity',
];

const enables = [
  ['Persistent Civilizations', 'Societies that remember institutions, borders, resources, and player decisions.'],
  ['Collaborative Sandbox Worlds', 'Shared construction and discovery spaces where the world keeps its state.'],
  ['Sovereign MMORPGs', 'Role-playing worlds with portable packages, recoverable history, and operator choice.'],
  ['Living Economies', 'Markets shaped by scarcity, production, trust, and long-running player behavior.'],
  ['Community-Governed Worlds', 'Places where rules, stewardship, and upgrades can become public community processes.'],
  ['Historical Simulations', 'Worlds where events accumulate into a legible, replayable record.'],
  ['Incrementally Evolving Worlds', 'Games that grow through modules, seasons, and migrations without erasing continuity.'],
];

const trust = [
  ['Deterministic WASM Runtime', 'World logic runs inside a constrained runtime designed for repeatable execution.'],
  ['Replay Verification', 'History windows can be checked after the fact instead of accepted on faith.'],
  ['Checkpoint Restore', 'Recovery material lets operators continue worlds after incidents or migrations.'],
  ['Federation', 'Operational responsibility can move beyond a single permanent host.'],
  ['Open Architecture', 'Design details live in public docs for review and contribution.'],
  ['Portable Worlds', 'World packages separate rules and continuity from one distribution channel.'],
];

const paths = [
  ['Explore Worlds', '/worlds', 'Understand what makes a world different from a temporary game session.'],
  ['Developers', '/developers', 'Create, deploy, verify, operate, and scale your first world.'],
  ['Technical Overview', '/developers/technical-overview', 'Follow the engineering path into contracts, replay, packaging, and continuity.'],
  ['Compare Platforms', '/compare', 'See how EverArcade differs from existing platform and hosting models.'],
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
        <Link className="button button--secondary button--lg" to="/developers/capabilities">Capabilities</Link>
        <Link className="button button--outline button--secondary button--lg" to="/open-source">Open Source Repo — Coming Soon</Link>
      </div>
    </div></header>
    <main>
      <section className="sectionBand"><div className="container"><h2>What is EverArcade?</h2><p>EverArcade helps teams create worlds that keep going: history is preserved, state can move between operators, and communities can verify what happened without trusting a single server forever.</p><div className="valueGrid">{values.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div></div></section>
      <section className="sectionBand identityBand"><div className="container"><h2>Why worlds matter</h2><p>Traditional games usually bind community, rules, state, and hosting into one product lifecycle. EverArcade separates the world from a single server or storefront so continuity can become the foundation instead of a temporary feature.</p><div className="comparisonGrid"><div className="cardPanel"><h3>Traditional Games</h3><ul>{traditional.map((item) => <li key={item}>{item}</li>)}</ul></div><div className="cardPanel accentPanel"><h3>EverArcade Worlds</h3><ul className="tagList">{everarcade.map((item) => <li key={item}>{item}</li>)}</ul></div></div></div></section>
      <section className="sectionBand sovereigntyBand"><div className="container"><p className="kicker">Ownership, operation, verification</p><h2>Why Sovereignty Matters</h2><p className="sovereigntyLead">Traditional platforms own the server, the rules, the distribution, and the continuity of the game. EverArcade separates those powers.</p><div className="sovereigntyStatement"><p>Developers define the world.</p><p>Operators run the world.</p><p>Players inhabit the world.</p><p>Verification protects the history.</p><p>Ownership anchors outside the platform.</p></div><div className="valueGrid">{sovereigntyStack.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div><p>Combined, these pieces support <strong>developer-owned worlds</strong>, portable execution, independent verification, operator independence, less platform lock-in, and long-lived world continuity.</p><ul className="tagList outcomeTags">{sovereigntyOutcomes.map((item) => <li key={item}>{item}</li>)}</ul><div className="heroActions"><Link className="button button--primary" to="/docs/concepts/sovereign-worlds">Read the sovereignty concept</Link></div></div></section>
      <section className="sectionBand"><div className="container"><h2>What EverArcade enables</h2><p>EverArcade is for developers who want the world itself to be the durable product: its rules, packages, history, operators, and community memory.</p><div className="valueGrid">{enables.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div></div></section>
      <section className="sectionBand"><div className="container"><h2>Why developers trust EverArcade</h2><p>Capability is visible in the architecture, not hidden behind marketing language.</p><div className="valueGrid">{trust.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div><div className="heroActions"><Link className="button button--primary" to="/developers/technical-overview">Technical Overview</Link><Link className="button button--secondary" to="/developers/capabilities">Capability Matrix</Link></div></div></section>
      <section className="sectionBand"><div className="container"><h2>Choose your path</h2><p>No protocol background required. Start with the role closest to you.</p><div className="pathGrid">{paths.map(([title, to, body]) => <Link className="cardPanel" to={to} key={title}><h3>{title}</h3><p>{body}</p></Link>)}</div></div></section>
    </main>
  </Layout>;
}
