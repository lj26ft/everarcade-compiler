# Theme Contrast Report

## Affected components found

The audit focused on `src/css/custom.css` and Docusaurus-rendered documentation components. The site intentionally forces both light and dark themes toward dark EverArcade surfaces, but several generated components could inherit white or low-contrast foregrounds against white/default Docusaurus backgrounds. Risk areas included badges, pills, tags, inline code, admonitions, table headers/cells, details/summary blocks, tabs, table-of-contents links, Mermaid SVG labels, buttons, cards, navbar links, sidebar links, and footer links.

## Root cause

The base theme customized background and font colors but did not set a complete contrast system for all Docusaurus variables and generated component classes. Components with default Docusaurus backgrounds could therefore render with inherited foreground colors that were tuned for the custom dark page background rather than the generated component surface.

## CSS variables changed

The update defines or reinforces content, heading, link, navbar, footer, table, code, blockquote, TOC, menu, tab, and highlighted-code variables for `:root`, `html[data-theme='light']`, and `[data-theme='dark']`.

## Pages checked

- `docs/canonical-package-format.md`, especially `#round-trip-guarantees`.
- New technical architecture, operator, developer, capability, concept, and continuity pages.
- Sidebar, navbar, tables, inline code, Mermaid diagrams, cards, and footer areas through the Docusaurus build.

## Dark mode result

Dark mode now uses explicit readable text colors for documentation body text, headings, links, tables, inline code, badges, generated tags/pills, details, tabs, admonition-like blocks, Mermaid text, buttons, cards, sidebar items, and footer links.

## Light mode result

The project's light theme remains a dark EverArcade visual theme, but now receives the same contrast-safe component variables and generated-class overrides as dark mode.

## Remaining known visual issues

No remaining white-on-white or dark-on-dark documentation component issue is known from the source audit. A browser-based visual pass is still recommended after deployment because third-party plugin output can add class names not present in source CSS.
