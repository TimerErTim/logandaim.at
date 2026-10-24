# Graph Report - frontend  (2026-10-24)

## Corpus Check
- 66 files · ~14,379 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 438 nodes · 860 edges · 18 communities (16 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2ffd3e60`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- sections.tsx
- pfeil-table-section.tsx
- provider.tsx
- package.json
- navigation.ts
- dependencies
- token-swatches.tsx
- biome.json
- card.tsx
- compilerOptions
- components.json
- layout.tsx
- button.tsx
- dates.ts
- devDependencies
- Frontend
- postcss.config.mjs

## God Nodes (most connected - your core abstractions)
1. `cn()` - 86 edges
2. `react` - 35 edges
3. `compilerOptions` - 16 edges
4. `scripts` - 9 edges
5. `Wertungszone` - 9 edges
6. `Button()` - 9 edges
7. `HttpLogandaimApi` - 8 edges
8. `LogandaimApi` - 8 edges
9. `useLogandaimApi()` - 8 edges
10. `lucide-react` - 7 edges

## Surprising Connections (you probably didn't know these)
- `ThemeFrame()` --calls--> `cn()`  [EXTRACTED]
  src/components/dev-gallery/component-section.tsx → src/lib/cn.ts
- `TickRule()` --calls--> `cn()`  [EXTRACTED]
  src/components/dev-gallery/feldbuch-section.tsx → src/lib/cn.ts
- `Wegmarke()` --calls--> `cn()`  [EXTRACTED]
  src/components/dev-gallery/feldbuch-section.tsx → src/lib/cn.ts
- `Distanzband()` --calls--> `cn()`  [EXTRACTED]
  src/components/dev-gallery/feldbuch-section.tsx → src/lib/cn.ts
- `WegmarkeNav()` --calls--> `cn()`  [EXTRACTED]
  src/components/dev-gallery/feldbuch-section.tsx → src/lib/cn.ts

## Import Cycles
- None detected.

## Communities (18 total, 1 thin omitted)

### Community 0 - "sections.tsx"
Cohesion: 0.05
Nodes (61): clsx, lucide-react, @radix-ui/react-dropdown-menu, @radix-ui/react-progress, @radix-ui/react-radio-group, @radix-ui/react-separator, @radix-ui/react-slider, @radix-ui/react-switch (+53 more)

### Community 1 - "pfeil-table-section.tsx"
Cohesion: 0.07
Nodes (33): Wertungszone, Body, Kill, Miss, Spot, zonen, Distanzband(), FeldbuchSection() (+25 more)

### Community 2 - "provider.tsx"
Cohesion: 0.10
Nodes (21): openapi-fetch, @tanstack/react-query-devtools, ApiError, HttpLogandaimApi, JsonClient, unwrap(), createLogandaimApi(), LogandaimApi (+13 more)

### Community 3 - "package.json"
Cohesion: 0.06
Nodes (34): name, private, scripts, build, check:format, check:lint, check:ts, dev (+26 more)

### Community 4 - "navigation.ts"
Cohesion: 0.12
Nodes (23): AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+15 more)

### Community 5 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

### Community 6 - "token-swatches.tsx"
Cohesion: 0.11
Nodes (22): ComponentSection(), Example(), ThemeFrame(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour() (+14 more)

### Community 7 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 8 - "card.tsx"
Cohesion: 0.19
Nodes (14): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), Card() (+6 more)

### Community 9 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 10 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 11 - "layout.tsx"
Cohesion: 0.17
Nodes (9): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, next, @tanstack/react-query, geistMono, metadata, nunitoSans, Providers() (+1 more)

### Community 12 - "button.tsx"
Cohesion: 0.21
Nodes (9): @radix-ui/react-slot, GallerySections(), TypeSection(), Alert(), AlertProps, alertVariants, Button(), ButtonProps (+1 more)

### Community 13 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 14 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 15 - "Frontend"
Cohesion: 0.33
Nodes (5): Building, Configuration, Design system, Development, Frontend

## Knowledge Gaps
- **167 isolated node(s):** `$schema`, `enabled`, `clientKind`, `useIgnoreFile`, `ignoreUnknown` (+162 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 201 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `react` connect `sections.tsx` to `pfeil-table-section.tsx`, `provider.tsx`, `package.json`, `navigation.ts`, `token-swatches.tsx`, `card.tsx`, `layout.tsx`, `button.tsx`?**
  _High betweenness centrality (0.163) - this node is a cross-community bridge._
- **Why does `dependencies` connect `dependencies` to `package.json`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._
- **Why does `cn()` connect `sections.tsx` to `pfeil-table-section.tsx`, `package.json`, `navigation.ts`, `token-swatches.tsx`, `card.tsx`, `button.tsx`?**
  _High betweenness centrality (0.077) - this node is a cross-community bridge._
- **What connects `$schema`, `enabled`, `clientKind` to the rest of the system?**
  _167 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `sections.tsx` be split into smaller, more focused modules?**
  _Cohesion score 0.05381400208986416 - nodes in this community are weakly interconnected._
- **Should `pfeil-table-section.tsx` be split into smaller, more focused modules?**
  _Cohesion score 0.06767676767676768 - nodes in this community are weakly interconnected._
- **Should `provider.tsx` be split into smaller, more focused modules?**
  _Cohesion score 0.0953058321479374 - nodes in this community are weakly interconnected._