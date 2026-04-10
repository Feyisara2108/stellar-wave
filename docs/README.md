# wave (Stellar/Soroban Public Goods Scaffold)

`wave` is a maintainer-first Soroban project scaffold designed for Drips.wave Wave 4.
It provides an auditable contract, SDK shell, CI baseline, and governance/funding
documentation required for transparent public-goods operation.

## Quickstart

1. Install toolchains:
   - Rust stable + `wasm32-unknown-unknown`
   - `soroban` CLI
   - Node.js 20+
2. Run local checks:
   - `cargo test --workspace`
   - `cargo build --target wasm32-unknown-unknown --release -p wave-contract`
   - `npm install --prefix sdk`
   - `npm run --prefix sdk build`
3. Deploy to testnet:
   - `bash scripts/deploy-testnet.sh`

## Stellar Alignment

- Smart contract built on `soroban-sdk`.
- Testnet-first deployment model with explicit mainnet-readiness path.
- Funding transparency and governance artifacts for maintainer accountability.

