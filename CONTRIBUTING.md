# Contributing to SPECTRA

Thank you for your interest in contributing to the SPECTRA display engine.

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Run lints: `cargo clippy --workspace && cargo fmt --all`
6. Submit a pull request

## Code Standards

- All Rust code must pass `clippy` with zero warnings
- All code must be formatted with `rustfmt`
- New public APIs require documentation
- All new features require tests

## Architecture

See [docs/architecture.md](docs/architecture.md) for system design details.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
