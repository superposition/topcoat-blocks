---
name: topcoat-blocks
description: Build, review, and release accessible copy-to-own UI blocks for Topcoat with the Topcoat Blocks registry and Coatcheck. Use when creating shadcn-style blocks for Topcoat, adding or changing registry components, building a Coatcheck workbench or scene matrix, validating a block in a real consuming application, publishing topcoat-blocks, or deciding whether a proven block should be proposed upstream to Topcoat.
---

# Topcoat Blocks

Develop blocks in `superposition/topcoat-blocks`, not in Coatcheck or the
upstream Topcoat repository. Keep Coatcheck product-neutral and use upstream
Topcoat only for mature primitive-level promotions.

## Ground the work

1. Locate a clean `topcoat-blocks` checkout and verify its remote.
2. Read `CONTRIBUTING.md`, `docs/roadmap.md`, and the closest existing registry
   component before editing.
3. Confirm a real consuming application and the public Topcoat primitives the
   block can compose.
4. State the host-owned contract: actions, routes, data, errors, CSRF,
   authentication, persistence, and product policy must stay outside the block.

Do not start a block whose primitive dependencies or server-side contract are
still speculative.

## Build one canonical block

1. Add the canonical source under
   `crates/topcoat-blocks/registry/components/`.
2. Register its source and Topcoat registry dependencies in
   `registry/registry.toml`.
3. Prefer native HTML behavior and server-owned state. Add enhancement only
   after the unenhanced semantics are correct.
4. Forward root attributes and use child slots for host-owned hidden fields or
   composition when appropriate.
5. Document the public props and install contract beside the component.

Never treat the workbench copy as canonical. `topcoat ui add` regenerates that
copy from the registry.

## Review through Coatcheck

1. Run `./scripts/bootstrap` to install the registry component into the real
   Topcoat workbench.
2. Add catalog metadata and explicit scenes for meaningful viewport, surface,
   theme, and state combinations.
3. Add project-specific Playwright tests under `coatcheck/` for form methods,
   names, labels, links, keyboard behavior, and other browser semantics.
4. Run `./scripts/check`.
5. Inspect the generated PNGs intentionally. Treat them as review evidence,
   never pixel-baseline assertions.

The automatic gate must cover frame readiness, navigation, browser errors,
horizontal overflow, configured Axe impacts, package isolation, Rust tests,
formatting, and clippy.

## Preserve the release boundary

Run `./scripts/release-smoke` before publication. The published crate may
contain registry and library source only. Exclude:

- Coatcheck configuration, generated files, and tests;
- Playwright, Bun, and Node dependencies;
- the workbench and selected review images;
- repository scripts and Codex skills.

Consumers receive copy-to-own Rust. They must not inherit the review harness.

## Commit and publish

Keep infrastructure, block behavior, review evidence, and release policy in
independently understandable commits. Publish through a pull request with the
host contract, installed dependencies, reviewed scenes, semantic results, and
package-isolation result.

Propose a component upstream only after it has real consumer evidence, no
Topcoat Blocks branding or product policy, and a scope appropriate for a
framework primitive.
