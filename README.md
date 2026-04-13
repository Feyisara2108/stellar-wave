<div align="center">
  <img src="https://stellar.org/icons/stellar-icon-logo.svg" width="100" height="100" alt="Stellar Logo" />
  <h1>🌊 Stellar-Wave Protocol v2</h1>
  <p><strong>The Production-Ready, Full-Stack Streaming Protocol natively built for Soroban.</strong></p>

  <p>
    <a href="https://github.com/stellar/soroban-cli"><img src="https://img.shields.io/badge/Soroban-v22.0-blue" alt="Soroban version" /></a>
    <a href="https://nestjs.com/"><img src="https://img.shields.io/badge/NestJS-Backend-red" alt="NestJS version" /></a>
    <a href="https://nextjs.org/"><img src="https://img.shields.io/badge/Next.js-14.2+-black" alt="Next.js version" /></a>
    <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-green" alt="License" /></a>
  </p>
</div>

---

## 📖 Overview

Stellar-Wave v2 fundamentally reimagines continuous token streaming on the Stellar network. What began as a smart contract scaffold has evolved into a comprehensive, extensible, **Full-Stack Streaming Monorepo** designed explicitly for open-source contributors and dApp developers.

Whether you're allocating DAO treasuries, distributing payroll continuously over time, or releasing venture grants with strict cliffs, Stellar-Wave provides the architecture to compose streaming into any UI securely and natively.

---

## ⚡ Features

### 🔐 Advanced Soroban Smart Contract
- **Multi-Token Support**: Stream native XLM or any standard Stellar custom asset seamlessly.
- **Granular Scheduling**: Enforce time-locked cliffs before streaming begins natively at the ledger level.
- **Admin Control & Pausability**: Deep administrative mechanisms to freeze distributions during emergency scenarios.
- **Cancellation**: Ability for stream creators to instantly claw back unvested capital securely.

### 🌐 Scalable Backend Abstractions
- **NestJS Workers & API**: Pre-integrated backend mapping stream histories automatically for user exploration.
- **Soroban Indexer**: Headless background service constantly parsing Soroban event streams to mirror state into PostgreSQL immediately.

### 🎨 State-of-the-art Developer Experience
- **Next.js & shadcn/ui**: Plug-and-play modern dashboard interface handling Soroban RPC interactions under the hood.
- **Robust TypeScript SDK**: Unified abstract tooling. 
- **Dockerized Environment**: Spin up the backend, postgres DB, and frontend via `docker compose`.

---

## 🏗️ Monorepo Architecture

The repository enforces strict separation of concerns for unparalleled maintainability:

```text
stellar-wave/
├── contracts/          # 🦀 Rust-based Soroban contracts (wave-contract) and test suite.
├── sdk/                # 🛠️ TypeScript wrapper abstracting rpc.Server and transaction builders.
├── backend/            # 🟢 NestJS ecosystem (API endpoints, Postgres Indexer, and Cron Workers).
├── frontend/           # ⚛️ Next.js Web App tailored with shadcn/ui and Tailwind.
├── packages/           # 📦 Shared libs and design tokens (ui, config, types).
├── examples/           # 💡 Functional reference guides (payroll-app, dao-treasury, grants).
└── docs/               # 📚 Expanded technical specifications and Swagger API references.
```

---

## 🚀 Quickstart

1. **Prerequisites**: Ensure you have [Docker](https://www.docker.com/) and [pnpm](https://pnpm.io/) initialized globally on your machine.
2. **Setup Dependencies**:
   ```bash
   pnpm install
   ```
3. **Launch Backend**:
   Initialize PostgreSQL, the API service, and Event Indexer.
   ```bash
   docker compose up -d
   ```
4. **Boot App**:
   Spin up the interactive Next.js Dashboard.
   ```bash
   pnpm dev
   ```


---

## 🌍 Open Source & Future Strategy

Stellar-Wave is proudly community-centric. With the baseline V2 infrastructure now complete, we are pivoting significantly towards a distributed development roadmap focusing on broader ecosystem value:

### Community Roadmap

1. **Protocol Audits**: Organizing community bounties to iteratively discover and patch zero-day logic issues.
2. **SDK Expansion**: Expanding the SDK to support Python (fastapi/django) and GoLang abstractions natively.
3. **Stellar Anchor Integrations**: Implementing built-in Anchor withdrawal tracking, allowing streamed tokens to map directly to fiat APIs on completion.
4. **Enhanced Indexing**: Integrating Mercury / Horizon for off-chain metrics scaling.

### Get Involved & Good First Issues

The best way to start contributing is to look for issues labeled `good-first-issue`. If you're looking to contribute to Stellar-Wave, here are some beginner-friendly areas!

1. **Dashboard Tooltips**: Add minor hover tooltips to the Timeline component.
2. **SDK Type Enhancements**: Improve docstrings on the `WaveSDK` methods.
3. **Indexer Logging**: Make indexer error messages more verbose when Soroban RPC is down.

---

## 🤝 Contributing to Stellar-Wave

We welcome contributions to making Stellar-Wave the premier streaming protocol on Soroban!

**Development Setup**
1. Clone repo
2. Run `pnpm i`
3. Run `pnpm dev` to launch frontend, indexer and api.

**Pull Requests**
- Must pass all tests (`cargo test --workspace` and `pnpm test`).
- 100% smart contract code coverage is strictly enforced.
- Follow conventional commits.

---

## ⚖️ Code of Conduct

All contributors and participants of the Stellar-Wave project are expected to conform to the following Code of Conduct.

**Our Pledge**
We as members, contributors, and leaders pledge to make participation in our project and our community a harassment-free experience for everyone.

---

## 🛡️ Best Practices & Quality

We firmly believe in Zero-Shortcut delivery. 
- The Soroban `wave-contract` demands **100% test coverage** for all logic branches globally.
- E2E testing ensures our frontends map RPC state dynamically to UI updates reliably.

---

> Built with 🩵 by the Stellar ecosystem.
