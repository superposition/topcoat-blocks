# Topcoat Blocks

Topcoat Blocks is an independent collection of accessible, copy-to-own UI
blocks for [Topcoat](https://github.com/tokio-rs/topcoat). Every block is
developed in a real Topcoat host, reviewed in
[Coatcheck](https://github.com/superposition/coatcheck), and distributed
through a custom Topcoat UI registry.

The registry is the product. Upstream contributions are optional promotions
of components that have already survived the local visual, semantic, and
consumer-install feedback loop.

Coatcheck, Playwright, and the workbench are repository-only review tooling.
They are not dependencies of `topcoat-blocks` and are excluded from the
published crate. `./scripts/release-smoke` enforces that boundary.

## Status

The first block is `sign_in_form`: a portable server POST form with optional
recovery and account-creation paths, a hidden-field slot, and no authentication
provider assumptions.

## Install

Add the registry carrier and Topcoat to the consuming package:

```toml
[dependencies]
topcoat = { version = "0.4.0", features = ["tailwind", "ui"] }
topcoat-blocks = { git = "https://github.com/superposition/topcoat-blocks" }
```

Initialize Topcoat UI once, then copy the block and its Topcoat primitives into
the application:

```sh
topcoat ui init --theme neutral
topcoat ui add sign_in_form --registry topcoat-blocks
```

The installed Rust belongs to the application. The registry dependency remains
useful when checking for or installing later revisions.

## Use

```rust
use crate::components::sign_in_form::sign_in_form;
use topcoat::view::{attributes, view};

view! {
    sign_in_form(
        action: "/session",
        forgot_password_href: Some("/forgot-password".to_owned()),
        sign_up_href: Some("/sign-up".to_owned()),
        attrs: attributes! { class="w-full max-w-sm" },
        <input type="hidden" name="csrf_token" value=(csrf_token)>
    )
}
```

The form submits `email` and `password` by POST. Pass `id_prefix` when a page
renders multiple instances.

## Develop and review

Bootstrap the repository-local tools and open the visual workbench:

```sh
./scripts/bootstrap
./.tools/bin/coatcheck dev
```

The full validation gate is:

```sh
./scripts/check
```

## Repository structure

- `crates/topcoat-blocks` is the custom component registry.
- `apps/workbench` renders installed registry source in Topcoat.
- `coatcheck` contains project-specific semantic browser checks.
- `reviews` records selected, approved visual evidence.
- `skills/topcoat-blocks` is the reusable Codex development workflow.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the acceptance gate and
[docs/roadmap.md](docs/roadmap.md) for the block sequence.

## Codex skill

The versioned `$topcoat-blocks` skill keeps future blocks on the same registry,
workbench, Coatcheck, package-isolation, and release path. Install it into a
local Codex environment by linking `skills/topcoat-blocks` from this checkout
under `~/.codex/skills/topcoat-blocks`.

## License

MIT
