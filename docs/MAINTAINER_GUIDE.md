# Maintainer Guide

## Release Process

1. Run CI-equivalent checks locally.
2. Tag release and publish changelog.
3. Publish deterministic build metadata and hashes.
4. Update roadmap and transparency report.

## Dependency Updates

- Rust and JS dependency updates must include:
  - changelog links
  - compatibility notes
  - risk assessment

## Audit Checklist

- Contract auth boundaries reviewed
- Storage layout changes documented
- Error paths tested
- Deployment script reviewed
- Security contact validated

## Funding Transparency

- Log Drips stream config changes in `FUNDING_TRANSPARENCY.md`.
- Publish quarterly report with milestone completion evidence.

## Succession Plan

- Add at least one backup maintainer.
- Transfer release keys/secrets using documented secure handoff.

