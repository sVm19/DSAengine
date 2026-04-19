#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:${PORT:-10000}}"
MASTER_API_2026="${MASTER_API_2026:?MASTER_API_2026 is required}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"

health_url="${BASE_URL}/health"
auth_header="X-API-KEY: ${MASTER_API_2026}"
json_header="Content-Type: application/json"

endpoints=(
  "/api/v1/advanced/bloom_filter"
  "/api/v1/advanced/lfu_cache"
  "/api/v1/graphs/dijkstra"
)

# Includes empty body, malformed JSON, and wrong-shape JSON.
payloads=(
  ""
  "{}"
  "[]"
  "{\"oops\":true}"
  "{\"operations\":42}"
  "{\"bad\":"
)

assert_health() {
  local code
  code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" "${health_url}")"
  if [[ "${code}" != "200" ]]; then
    echo "FAIL: /health is not healthy after fuzz case (HTTP ${code})"
    exit 1
  fi
}

echo "Running payload fuzz checks against ${BASE_URL}"
assert_health

for endpoint in "${endpoints[@]}"; do
  for payload in "${payloads[@]}"; do
    code="$(
      curl -sS -o /dev/null -w "%{http_code}" \
        --max-time "${TIMEOUT_SECONDS}" \
        -X POST "${BASE_URL}${endpoint}" \
        -H "${auth_header}" \
        -H "${json_header}" \
        --data "${payload}" || true
    )"

    echo "endpoint=${endpoint} payload='${payload:0:24}' http=${code}"

    if [[ "${code}" =~ ^5 ]]; then
      echo "FAIL: endpoint ${endpoint} returned 5xx (${code}) for malformed/invalid payload"
      exit 1
    fi

    assert_health
  done
done

echo "PASS: malformed/empty JSON did not trigger 5xx or server crash."
