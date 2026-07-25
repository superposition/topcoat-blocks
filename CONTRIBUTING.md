# Contributing to Topcoat Blocks

Topcoat Blocks ships copy-to-own product UI, not screenshots or framework
adapters. A contribution is ready when a maintainer can install it into a
normal Topcoat package, understand its public contract, and verify its behavior
without trusting a hand-built demo.

## Acceptance gate

Every block must:

1. Live as one canonical source file in the custom registry.
2. Compose public Topcoat primitives through registry dependencies.
3. Render in the workbench from a fresh `topcoat ui add`.
4. Preserve native HTML semantics before adding enhancement.
5. Pass keyboard, serious/critical Axe, console, light/dark, and responsive
   review.
6. Document host-owned concerns such as actions, data, CSRF, and routing.
7. Stay out of the published crate's Coatcheck and Playwright dependency
   graph.
8. Pass `./scripts/check` and `./scripts/release-smoke`.

Selected PNGs are review evidence, not pixel-locking test baselines. Semantic
gates fail automatically; visual approval remains a deliberate human step.

## Commit shape

Keep commits independently understandable:

1. Registry or infrastructure boundary.
2. Production block and install contract.
3. Behavioral tests and selected visual evidence.
4. Documentation or release policy.

Do not mix unrelated blocks, generated workbench installs, or opportunistic
refactors into a block commit. Commit messages should name the product result,
for example `feat: add portable sign-in form`.

## Pull request evidence

Describe:

- the user task the block solves;
- the host application contract;
- the installed Topcoat dependencies;
- viewports and surfaces reviewed;
- keyboard and semantic behavior checked;
- package-isolation output;
- what is intentionally deferred.

## Upstream promotion

This repository does not use the Topcoat project as its staging area. A mature
piece may be proposed upstream only when it is broadly reusable, fits
Topcoat's primitive-level scope, has no library-specific branding or product
policy, and already has evidence from real use here.
