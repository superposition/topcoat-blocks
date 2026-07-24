# Topcoat Blocks

Topcoat Blocks is an independent collection of accessible, copy-to-own UI
blocks for [Topcoat](https://github.com/tokio-rs/topcoat). Every block is
developed in a real Topcoat host, reviewed in
[Coatcheck](https://github.com/superposition/coatcheck), and distributed
through a custom Topcoat UI registry.

The registry is the product. Upstream contributions are optional promotions
of components that have already survived the local visual, semantic, and
consumer-install feedback loop.

## Status

The first release is developing a portable sign-in form. Run the workbench
after the repository bootstrap is available:

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

## License

MIT

