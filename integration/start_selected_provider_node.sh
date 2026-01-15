#!/usr/bin/env bash

set -euo pipefail

YAGNA_EXEC=$1
PROVIDER_EXEC=$2

# require .env and load it; no fallback
if [[ ! -f .env ]]; then
  echo "ERROR: .env not found" >&2
  exit 1
fi
set -o allexport; source .env; set +o allexport
if [[ -z "${YAGNA_API_URL:-}" ]]; then
  echo "ERROR: YAGNA_API_URL not set in .env" >&2
  exit 1
fi
if [[ -z "${YAGNA_APPKEY:-}" ]]; then
  echo "ERROR: YAGNA_APPKEY not set in .env" >&2
  exit 1
fi

ENDPOINT="${YAGNA_API_URL}"

TIMEOUT=60
INTERVAL=1

"${YAGNA_EXEC}" service run >/dev/null 2>&1 &

start_time=$(date +%s)

echo "Waiting for yagna to become ready..."
echo "Endpoint: $ENDPOINT"
echo "Timeout: ${TIMEOUT} seconds"

while true; do
  # Check HTTP status code
  status=$(curl -s -o /dev/null -w "%{http_code}" -H "Authorization: Bearer ${YAGNA_APPKEY}" "${ENDPOINT}"/me || true)

  if [[ "$status" == "200" ]]; then
    echo "yagna is ready (HTTP 200)"
    exit 0
  fi

  current_time=$(date +%s)
  elapsed=$((current_time - start_time))

  if (( elapsed >= TIMEOUT )); then
    echo "ERROR: yagna did not become ready within ${TIMEOUT} seconds." >&2
    echo "Last HTTP status: $status" >&2
    exit 1
  fi

  sleep "$INTERVAL"
done

"${PROVIDER_EXEC}" run --no-file-monitors >/dev/null 2>&1 &
