#!/usr/bin/env bash
set -euo pipefail

: "${SOROBAN_RPC_URL:=https://soroban-testnet.stellar.org}"
: "${SOROBAN_NETWORK_PASSPHRASE:=Test SDF Network ; September 2015}"
: "${WAVE_SOURCE_ACCOUNT:?Set WAVE_SOURCE_ACCOUNT (funded testnet account)}"
: "${WAVE_ADMIN_ADDRESS:?Set WAVE_ADMIN_ADDRESS}"

echo "==> Building contract"
cargo build --target wasm32-unknown-unknown --release -p wave-contract

WASM_PATH="target/wasm32-unknown-unknown/release/wave_contract.wasm"

echo "==> Deploying contract to testnet"
CONTRACT_ID="$(soroban contract deploy \
  --wasm "${WASM_PATH}" \
  --source "${WAVE_SOURCE_ACCOUNT}" \
  --rpc-url "${SOROBAN_RPC_URL}" \
  --network-passphrase "${SOROBAN_NETWORK_PASSPHRASE}")"

echo "Contract deployed: ${CONTRACT_ID}"

echo "==> Initializing contract admin"
soroban contract invoke \
  --id "${CONTRACT_ID}" \
  --source "${WAVE_SOURCE_ACCOUNT}" \
  --rpc-url "${SOROBAN_RPC_URL}" \
  --network-passphrase "${SOROBAN_NETWORK_PASSPHRASE}" \
  -- \
  init \
  --admin "${WAVE_ADMIN_ADDRESS}"

echo "Deployment complete. Export:"
echo "export WAVE_CONTRACT_ID=${CONTRACT_ID}"

