import React from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const problemCards = [
  ['Disposable worlds', 'MMOs shut down. Private servers disappear. Communities fracture. The maps, economies, guilds, wars, jokes, rituals, and records that made a world meaningful vanish with the infrastructure that hosted them.'],
  ['Platform dependence', 'Worlds depend on studios, publishers, storefronts, platforms, and servers. When the owner leaves, the world dies—even if the community is still alive.'],
  ['Creator extraction', 'Creators build value. Platforms capture value. Artists, modders, operators, lore writers, economists, tool builders, and community leaders rarely own the institutions they help create.'],
  ['Rebuilding the same systems', 'Every game rebuilds housing, guilds, crafting, economies, permissions, reputation, and governance. Knowledge is trapped inside product cycles instead of compounding across worlds.'],
];

const worldActions = ['Move', 'Survive', 'Verify', 'Replay', 'Restore', 'Govern', 'Grow'];
const continuityExamples = ['History', 'Economies', 'Governments', 'Guilds', 'Wars', 'Events', 'Institutions'];
const contributorActions = ['Build', 'Merge', 'Collaborate', 'Earn', 'Govern'];
const contributorAccumulation = ['Builders', 'History', 'Capabilities'];
const beyondGaming = ['Education', 'Simulation', 'Communities', 'Research', 'Governance', 'Digital Nations', 'Persistent AI Systems'];
const joinRoles = ['Operators', 'Developers', 'Artists', 'Designers', 'Writers', 'Economists', 'Governance Builders', 'AI Builders'];

const manifestoNav = [
  ['Home', '#home'],
  ['The Problem', '#problem'],
  ['world.evr', '#world-evr'],
  ['Continuity', '#continuity'],
  ['World Git', '#world-git'],
  ['Governance', '#governance'],
  ['Founding Worlds', '#founding-worlds'],
  ['Join Us', '#join-us'],
];

function PillList({items}) {
  return <ul className="tagList visionPills">{items.map((item) => <li key={item}>{item}</li>)}</ul>;
}

function ArrowModel({title, items}) {
  return <div className="cardPanel visionModel"><h3>{title}</h3>{items.map((item, index) => <React.Fragment key={item}><div>{item}</div>{index < items.length - 1 && <span aria-hidden="true">↓</span>}</React.Fragment>)}</div>;
}

export default function Vision() {
  return <Layout title="The EverArcade Vision" description="The public manifesto for persistent, sovereign, portable worlds.">
    <header id="home" className="hero heroBanner visionHero"><div className="container text--center">
      <p className="kicker">The Public Thesis of EverArcade</p>
      <h1 className="hero__title">Worlds should not be disposable.</h1>
      <p className="hero__subtitle">EverArcade exists to make persistent, sovereign, portable worlds possible.</p>
      <p className="heroStatement">Most game platforms are designed around products. EverArcade is designed around worlds.</p>
      <div className="heroActions">
        <a className="button button--primary button--lg" href="#world-evr">What is world.evr?</a>
        <a className="button button--secondary button--lg" href="#join-us">Come build worlds</a>
      </div>
    </div></header>

    <main className="visionPage">
      <nav className="visionNav" aria-label="Manifesto sections"><div className="container">{manifestoNav.map(([label, to]) => <a href={to} key={to}>{label}</a>)}</div></nav>

      <section className="sectionBand visionStatement"><div className="container">
        <p>Games are launched. Games are sold. Games are abandoned. Games disappear.</p>
        <p><strong>Worlds persist.</strong> Worlds accumulate history. Worlds accumulate culture. Worlds accumulate contributors. Worlds become institutions.</p>
      </div></section>

      <section id="problem" className="sectionBand"><div className="container">
        <p className="kicker">The Problem</p><h2>Digital worlds are treated like temporary products.</h2>
        <p>Players and creators pour years of memory, labor, identity, and trust into worlds that can be deleted by business decisions, infrastructure failures, licensing changes, or platform strategy.</p>
        <div className="valueGrid">{problemCards.map(([title, body]) => <div className="cardPanel" key={title}><h3>{title}</h3><p>{body}</p></div>)}</div>
      </div></section>

      <section id="world-evr" className="sectionBand identityBand"><div className="container">
        <p className="kicker">A New Primitive</p><h2>world.evr</h2>
        <div className="visionDefinition"><code>world.evr</code><p>A portable, verifiable, persistent world.</p></div>
        <p>Not a game save. Not a server. Not a client. A world: the durable object that can outlive any single company, storefront, host, or launch cycle.</p>
        <PillList items={worldActions} />
      </div></section>

      <section className="sectionBand"><div className="container">
        <p className="kicker">The World-Centered Model</p><h2>The world becomes the institution.</h2>
        <div className="comparisonGrid"><ArrowModel title="Traditional Model" items={['Studio', 'Game', 'Players']} /><ArrowModel title="World Model" items={['World', 'Contributors', 'Operators', 'Players', 'Communities']} /></div>
        <p>In the world model, the world is the durable artifact. Studios can contribute. Operators can run infrastructure. Players can inhabit and govern. Communities can preserve continuity.</p>
      </div></section>

      <section id="continuity" className="sectionBand sovereigntyBand"><div className="container">
        <p className="kicker">Continuity Engine</p><h2>Most games reset. Worlds remember.</h2>
        <p className="sovereigntyLead">Time should be a first-class system: not a timer, not a seasonal reset, but a foundation for memory, restoration, migration, and trust.</p>
        <PillList items={continuityExamples} />
      </div></section>

      <section id="world-git" className="sectionBand"><div className="container">
        <p className="kicker">World Git</p><h2>Git changed software. World Git changes worlds.</h2>
        <p>Worlds should be able to accumulate builders over years. Contributors should be able to propose, merge, collaborate, earn, and govern without requiring permanent employment by a studio.</p>
        <div className="comparisonGrid"><div className="cardPanel"><h3>Contributors can</h3><PillList items={contributorActions} /></div><div className="cardPanel"><h3>Worlds can accumulate</h3><PillList items={contributorAccumulation} /></div></div>
      </div></section>

      <section id="governance" className="sectionBand identityBand"><div className="container">
        <p className="kicker">Digital Institutions</p><h2>Communities need governance. Worlds need governance.</h2>
        <p>Persistent worlds become communities, economies, civilizations, and institutions. They need ways to steward upgrades, resolve conflicts, recognize contributors, preserve legitimacy, and survive leadership changes.</p>
      </div></section>

      <section className="sectionBand"><div className="container">
        <p className="kicker">AI Changes Everything</p><h2>AI can create content. AI cannot create continuity.</h2>
        <p>Future worlds may contain AI agents, AI citizens, AI economies, and AI institutions. But the world itself—the shared memory, rules, provenance, governance, and continuity—remains the durable artifact.</p>
      </div></section>

      <section id="founding-worlds" className="sectionBand sovereigntyBand"><div className="container">
        <p className="kicker">Beyond Gaming</p><h2>The world is the primitive. Gaming is only the beginning.</h2>
        <p className="sovereigntyLead">Persistent, sovereign, portable worlds can become infrastructure for simulation, education, community coordination, research, governance, digital nations, and persistent AI systems.</p>
        <PillList items={beyondGaming} />
      </div></section>

      <section id="join-us" className="sectionBand visionFinal"><div className="container text--center">
        <p className="kicker">The Invitation</p><h2>Come build worlds with us.</h2>
        <p>EverArcade is for the first generation of builders who believe worlds should survive their launch windows, outgrow their original teams, and become institutions communities can carry forward.</p>
        <PillList items={joinRoles} />
        <div className="heroActions"><Link className="button button--primary button--lg" to="/founding-developers">Start building</Link><Link className="button button--secondary button--lg" to="/docs/concepts/what-is-a-world">Learn what a world is</Link></div>
      </div></section>
    </main>
  </Layout>;
}
