#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:${PORT:-10000}}"
MASTER_API_2026="${MASTER_API_2026:?MASTER_API_2026 is required}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"

health_url="${BASE_URL}/health"

post_json() {
  local endpoint="$1"
  local payload="$2"
  curl -sS --max-time "${TIMEOUT_SECONDS}" \
    -X POST "${BASE_URL}${endpoint}" \
    -H "X-API-KEY: ${MASTER_API_2026}" \
    -H "Content-Type: application/json" \
    --data "${payload}"
}

post_code() {
  local endpoint="$1"
  local payload="$2"
  curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" \
    -X POST "${BASE_URL}${endpoint}" \
    -H "X-API-KEY: ${MASTER_API_2026}" \
    -H "Content-Type: application/json" \
    --data "${payload}" || true
}

assert_health() {
  local code
  code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" "${health_url}" || true)"
  if [[ "${code}" != "200" ]]; then
    echo "FAIL: /health is down (HTTP ${code})"
    exit 1
  fi
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  local label="$3"
  if [[ "${haystack}" != *"${needle}"* ]]; then
    echo "FAIL: ${label} missing expected fragment '${needle}'"
    echo "Response: ${haystack}"
    exit 1
  fi
}

echo "Running golden skill suite against ${BASE_URL}"
assert_health

# 1) Bloom Filter - valid case
resp="$(post_json "/api/v1/advanced/bloom_filter" '{"bit_count":128,"hash_count":3,"insert_items":["cat","dog"],"query_items":["cat","bird"]}')"
assert_contains "${resp}" "\"status\":\"success\"" "bloom_filter(valid)"
assert_contains "${resp}" "\"queries\"" "bloom_filter(valid)"
assert_contains "${resp}" "\"estimated_fp_rate\"" "bloom_filter(valid)"
assert_health

# 2) Bloom Filter - edge case (defaults + no inserts/queries)
resp="$(post_json "/api/v1/advanced/bloom_filter" '{}')"
assert_contains "${resp}" "\"status\":\"success\"" "bloom_filter(edge)"
assert_health

# 3) Bloom Filter - invalid type
code="$(post_code "/api/v1/advanced/bloom_filter" '{"bit_count":"bad"}')"
if [[ "${code}" =~ ^5 ]]; then
  echo "FAIL: bloom_filter(invalid) returned 5xx (${code})"
  exit 1
fi
assert_health

# 4) LFU Cache - valid case
resp="$(post_json "/api/v1/advanced/lfu_cache" '{"capacity":2,"operations":[{"type":"put","key":"a","value":"1"},{"type":"put","key":"b","value":"2"},{"type":"get","key":"a"}]}')"
assert_contains "${resp}" "\"status\":\"success\"" "lfu_cache(valid)"
assert_contains "${resp}" "\"results\"" "lfu_cache(valid)"
assert_health

# 5) LFU Cache - invalid schema
code="$(post_code "/api/v1/advanced/lfu_cache" '{"capacity":2,"operations":"bad"}')"
if [[ "${code}" =~ ^5 ]]; then
  echo "FAIL: lfu_cache(invalid) returned 5xx (${code})"
  exit 1
fi
assert_health

# 6) Dijkstra - valid case
resp="$(post_json "/api/v1/graphs/dijkstra" '{"num_nodes":5,"source":0,"destination":4,"edges":[[0,1,4],[1,2,3],[0,3,2],[3,4,1],[2,4,5]]}')"
assert_contains "${resp}" "\"status\":\"success\"" "dijkstra(valid)"
assert_contains "${resp}" "\"distances\"" "dijkstra(valid)"
assert_contains "${resp}" "\"path_to_destination\"" "dijkstra(valid)"
assert_health

# 7) Dijkstra - invalid source node
code="$(post_code "/api/v1/graphs/dijkstra" '{"num_nodes":2,"source":9,"edges":[[0,1,1]]}')"
if [[ "${code}" =~ ^5 ]]; then
  echo "FAIL: dijkstra(invalid) returned 5xx (${code})"
  exit 1
fi
assert_health

# 8) KMP - valid case
resp="$(post_json "/api/v1/arrays/kmp_search" '{"text":"ababcabcab","pattern":"abc"}')"
assert_contains "${resp}" "\"status\":\"success\"" "kmp_search(valid)"
assert_contains "${resp}" "\"matches\"" "kmp_search(valid)"
assert_health

# 9) KMP - invalid type
code="$(post_code "/api/v1/arrays/kmp_search" '{"text":["not","string"],"pattern":"abc"}')"
if [[ "${code}" =~ ^5 ]]; then
  echo "FAIL: kmp_search(invalid) returned 5xx (${code})"
  exit 1
fi
assert_health

echo "PASS: golden suite passed (valid/edge/invalid checks)."
