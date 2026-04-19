#!/usr/bin/env bash
set -euo pipefail

PORT="${PORT:-10000}"
HOST="${HOST:-127.0.0.1}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"
URL="http://${HOST}:${PORT}/health"

result="$(
  curl -sS -o /dev/null \
    -w "code=%{http_code} time=%{time_total}" \
    --max-time "${TIMEOUT_SECONDS}" \
    "${URL}"
)"

code="$(echo "${result}" | sed -n 's/.*code=\([0-9][0-9][0-9]\).*/\1/p')"
time_total="$(echo "${result}" | sed -n 's/.*time=\([0-9.]*\).*/\1/p')"

if [[ "${code}" != "200" ]]; then
  echo "FAIL: /health returned HTTP ${code} at ${URL}"
  exit 1
fi

awk -v t="${time_total}" -v limit="${TIMEOUT_SECONDS}" 'BEGIN { exit !(t <= limit) }'
if [[ $? -ne 0 ]]; then
  echo "FAIL: /health exceeded ${TIMEOUT_SECONDS}s (actual ${time_total}s)"
  exit 1
fi

echo "PASS: /health returned 200 in ${time_total}s (limit ${TIMEOUT_SECONDS}s)"
