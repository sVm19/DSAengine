#!/usr/bin/env bash
set -euo pipefail

IMAGE="${IMAGE:-dsaengine:render}"
PORT="${PORT:-10000}"
HOST_PORT="${HOST_PORT:-10000}"
MASTER_API_2026="${MASTER_API_2026:?MASTER_API_2026 is required}"
CONTAINER_NAME="${CONTAINER_NAME:-dsaengine_memcheck}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"
MEM_LIMIT="${MEM_LIMIT:-512m}"
RISK_THRESHOLD_MB="${RISK_THRESHOLD_MB:-450}"

BASE_URL="http://127.0.0.1:${HOST_PORT}"

cleanup() {
  docker rm -f "${CONTAINER_NAME}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "Starting container '${CONTAINER_NAME}' from image '${IMAGE}' with memory=${MEM_LIMIT}"
cleanup
docker run -d \
  --name "${CONTAINER_NAME}" \
  --memory="${MEM_LIMIT}" \
  --memory-swap="${MEM_LIMIT}" \
  -e PORT="${PORT}" \
  -e MASTER_API_2026="${MASTER_API_2026}" \
  -p "${HOST_PORT}:${PORT}" \
  "${IMAGE}" >/dev/null

echo "Waiting for /health..."
for _ in {1..30}; do
  code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" "${BASE_URL}/health" || true)"
  if [[ "${code}" == "200" ]]; then
    break
  fi
  sleep 1
done

code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" "${BASE_URL}/health" || true)"
if [[ "${code}" != "200" ]]; then
  echo "FAIL: service did not become healthy in time (HTTP ${code})"
  exit 1
fi

mem_usage_mb() {
  local raw
  raw="$(docker stats --no-stream --format "{{.MemUsage}}" "${CONTAINER_NAME}" | awk '{print $1}')"
  # expected examples: 123.4MiB, 0.95GiB
  if [[ "${raw}" == *GiB ]]; then
    awk -v v="${raw%GiB}" 'BEGIN { printf "%.0f", v * 1024 }'
  else
    awk -v v="${raw%MiB}" 'BEGIN { printf "%.0f", v }'
  fi
}

echo "Memory snapshot: cold start"
cold_mb="$(mem_usage_mb)"
echo "cold_start_mem_mb=${cold_mb}"

echo "Warm-up health checks"
for _ in {1..20}; do
  curl -sS --max-time "${TIMEOUT_SECONDS}" "${BASE_URL}/health" >/dev/null
done

echo "Skill-load checks (representative endpoints)"
post_json() {
  local ep="$1"
  local payload="$2"
  curl -sS -o /dev/null -w "%{http_code}" \
    --max-time "${TIMEOUT_SECONDS}" \
    -X POST "${BASE_URL}${ep}" \
    -H "X-API-KEY: ${MASTER_API_2026}" \
    -H "Content-Type: application/json" \
    --data "${payload}" || true
}

# Bloom filter
code="$(post_json "/api/v1/advanced/bloom_filter" '{"bit_count":1024,"hash_count":4,"insert_items":["a","b","c","d"],"query_items":["a","z"]}')"
echo "bloom_filter_http=${code}"

# LFU cache
code="$(post_json "/api/v1/advanced/lfu_cache" '{"capacity":64,"operations":[{"type":"put","key":"k1","value":"v1"},{"type":"put","key":"k2","value":"v2"},{"type":"get","key":"k1"}]}')"
echo "lfu_cache_http=${code}"

# Dijkstra
code="$(post_json "/api/v1/graphs/dijkstra" '{"num_nodes":6,"source":0,"destination":5,"edges":[[0,1,2],[1,2,3],[2,5,1],[0,3,4],[3,4,1],[4,5,2]]}')"
echo "dijkstra_http=${code}"

# Array/KMP
code="$(post_json "/api/v1/arrays/kmp_search" '{"text":"ababcabcababcabc","pattern":"abc"}')"
echo "kmp_http=${code}"

after_mb="$(mem_usage_mb)"
echo "post_load_mem_mb=${after_mb}"

if (( after_mb > RISK_THRESHOLD_MB )); then
  echo "RISK: memory ${after_mb}MB exceeds threshold ${RISK_THRESHOLD_MB}MB (close to 512MB cap)"
  exit 2
fi

echo "PASS: memory stayed within threshold (${after_mb}MB <= ${RISK_THRESHOLD_MB}MB)"
