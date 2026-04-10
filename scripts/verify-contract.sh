#!/usr/bin/env bash
set -euo pipefail

: "${SOROBAN_RPC_URL:=https://soroban-testnet.stellar.org}"
: "${SOROBAN_NETWORK_PASSPHRASE:=Test SDF Network ; September 2015}"
: "${WAVE_CONTRACT_ID:?Set WAVE_CONTRACT_ID}"
: "${WAVE_SOURCE_ACCOUNT:?Set WAVE_SOURCE_ACCOUNT}"
: "${WAVE_RECIPIENT_ADDRESS:?Set WAVE_RECIPIENT_ADDRESS}"

echo "==> Checking contract reachability"
soroban contract invoke \
  --id "${WAVE_CONTRACT_ID}" \
  --source "${WAVE_SOURCE_ACCOUNT}" \
  --rpc-url "${SOROBAN_RPC_URL}" \
  --network-passphrase "${SOROBAN_NETWORK_PASSPHRASE}" \
  -- \
  claimable \
  --stream_id 1 >/tmp/wave_claimable.out 2>/tmp/wave_claimable.err || true

if grep -qi "StreamNotFound" /tmp/wave_claimable.err; then
  echo "OK: contract responded with expected error for missing stream"
else
  echo "NOTE: claimable output"
  cat /tmp/wave_claimable.out
  cat /tmp/wave_claimable.err
fi

echo "==> Creating deterministic verification stream"
soroban contract invoke \
  --id "${WAVE_CONTRACT_ID}" \
  --source "${WAVE_SOURCE_ACCOUNT}" \
  --rpc-url "${SOROBAN_RPC_URL}" \
  --network-passphrase "${SOROBAN_NETWORK_PASSPHRASE}" \
  -- \
  create_stream \
  --stream_id 1001 \
  --recipient "${WAVE_RECIPIENT_ADDRESS}" \
  --total_amount 1000000 \
  --start_ledger 100 \
  --end_ledger 1000

echo "==> Reading stream state"
soroban contract invoke \
  --id "${WAVE_CONTRACT_ID}" \
  --source "${WAVE_SOURCE_ACCOUNT}" \
  --rpc-url "${SOROBAN_RPC_URL}" \
  --network-passphrase "${SOROBAN_NETWORK_PASSPHRASE}" \
  -- \
  get_stream \
  --stream_id 1001

echo "Verification completed."

