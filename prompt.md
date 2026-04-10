You are an expert Stellar ecosystem engineer, Soroban smart contract developer, and open-source public-goods maintainer. Scaffold a complete, production-ready project named "wave" for the Stellar network. This project will be used as the technical foundation for a Drips.wave Wave 4 maintainer funding application.

🎯 OBJECTIVE
Generate every file in the project structure with full, production-grade content. Do not summarize, skip files, or use placeholders like `// ... rest of code`. Output each file completely.

🛠️ TECHNICAL STACK

- Smart Contracts: Soroban (Rust) + `soroban-sdk`
- Client/SDK: TypeScript (`@stellar/stellar-sdk`, `@stellar/freighter` compatible)
- Tooling: `soroban-cli`, `cargo`, `npm`, `ts-node`
- CI/CD: GitHub Actions (lint, test, build, Soroban compliance checks)
- License: MIT or Apache-2.0
- Network: Stellar Testnet (with Mainnet readiness flags)

📁 REQUIRED FILES & CONTENTS

1. `contracts/wave-contract/Cargo.toml` & `contracts/wave-contract/src/lib.rs`
   - Implement a minimal, auditable Soroban contract aligned with public goods/streaming mechanics (e.g., time-based distribution, transparent treasury, or modular utility).
   - Include comprehensive unit tests, error handling, and CAP-compliant storage patterns.

2. `contracts/wave-contract/src/test.rs`
   - Property-based or scenario-driven tests covering edge cases, authorization, and resource limits.

3. `sdk/package.json`, `sdk/tsconfig.json`, `sdk/src/index.ts`, `sdk/src/wave-client.ts`
   - TypeScript SDK wrapping contract interactions, transaction building, and Soroban auth flows.
   - Include JSDoc, type safety, and Freighter/wallet adapter compatibility.

4. `scripts/deploy-testnet.sh` & `scripts/verify-contract.sh`
   - Bash scripts using `soroban contract deploy`, `contract invoke`, and verification steps with clear output parsing.
 with public goods/streaming mechanics (e.g., time-based distribution, transparent treasury, or modular utility).
   - Include comprehensive unit tests, error handling, and CAP-compliant storage patterns.

2. `contracts/wave-contract/src/test.rs`
5. `.github/workflows/ci.yml`
   - Lint (clippy, rustfmt), test (cargo test), SDK build (tsc, npm run test), and Soroban compilation checks.
   - Cache dependencies, fail-fast on errors, generate coverage reports.

6. `docs/README.md`, `docs/ARCHITECTURE.md`, `docs/CONTRIBUTING.md`, `docs/MAINTAINER_GUIDE.md`, `docs/ROADMAP.md`
   - `README.md`: Project purpose, quickstart, testnet usage, Stellar alignment.
   - `ARCHITECTURE.md`: Diagram (Mermaid), data flow, security model, SEP alignment.
   - `CONTRIBUTING.md`: PR workflow, coding standards, DCO, issue triage SLAs.
   - `MAINTAINER_GUIDE.md`: Release process, dependency updates, audit checklist, funding transparency, succession plan.
   - `ROADMAP.md`: Quarterly milestones mapped to Drips.wave Wave 4 evaluation criteria (public goods, streaming readiness, community impact).

7. `GOVERNANCE.md` & `FUNDING_TRANSPARENCY.md`
   - Decision-making process, maintainer roles, budget allocation breakdown, Drips stream configuration placeholder, and quarterly reporting cadence.

8. `.github/ISSUE_TEMPLATE/bug.md`, `.github/ISSUE_TEMPLATE/feature.md`, `.github/PULL_REQUEST_TEMPLATE.md`
   - Structured templates enforcing reproducibility, security considerations, and maintainer review standards.

9. `drips-stream-config.json` & `GRANT_APPLICATION_CHECKLIST.md`
   - JSON placeholder for Drips protocol streaming setup (recipient, duration, milestone triggers).
   - Checklist mapping project artifacts to Drips.wave Wave 4 reviewer requirements (open-source, transparent funding, CI passing, audit readiness, community governance).

10. Root files: `LICENSE`, `.gitignore`, `Cargo.toml` (workspace), `package.json` (monorepo/root), `.editorconfig`, `SECURITY.md`

⚖️ CONSTRAINTS & QUALITY STANDARDS

- All code must compile with current Soroban toolchain conventions.
- Follow Stellar SEP guidelines where applicable (e.g., SEP-6/24/41 if relevant to streaming/distribution).
- Documentation must explicitly address long-term maintenance, public goods impact, and verifiable milestones.
- Include clear comments on security boundaries, authorization models, and upgrade paths.
- Output format: For every file, print exactly:
  --- FILE: <relative/path> ---
  ```<language>
  <full file content>
  ```
