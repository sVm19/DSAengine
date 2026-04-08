#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${ROOT_DIR}"

IMAGE="${IMAGE:-dsaengine:render}"
PORT="${PORT:-10000}"
HOST="${HOST:-127.0.0.1}"
HOST_PORT="${HOST_PORT:-10000}"
MASTER_API_2026="${MASTER_API_2026:?MASTER_API_2026 is required}"
CONTAINER_NAME="${CONTAINER_NAME:-dsaengine_predeploy}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"
RUN_MEMORY="${RUN_MEMORY:-1}"

cleanup() {
  docker rm -f "${CONTAINER_NAME}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "==> [1/7] Cargo compile check"
cargo check

echo "==> [2/7] Build Docker image for linux/amd64"
docker buildx build --platform linux/amd64 -t "${IMAGE}" --load .

echo "==> [3/7] Verify image architecture"
arch="$(docker image inspect "${IMAGE}" --format '{{.Os}}/{{.Architecture}}')"
echo "image_arch=${arch}"
if [[ "${arch}" != "linux/amd64" ]]; then
  echo "FAIL: image architecture is '${arch}', expected 'linux/amd64'"
  exit 1
fi

echo "==> [4/7] Start container and wait for health"
cleanup
docker run -d \
  --name "${CONTAINER_NAME}" \
  -e PORT="${PORT}" \
  -e MASTER_API_2026="${MASTER_API_2026}" \
  -p "${HOST_PORT}:${PORT}" \
  "${IMAGE}" >/dev/null

for _ in {1..40}; do
  code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time "${TIMEOUT_SECONDS}" "http://${HOST}:${HOST_PORT}/health" || true)"
  [[ "${code}" == "200" ]] && break
  sleep 1
done

echo "==> [5/7] Health SLA check (<${TIMEOUT_SECONDS}s, HTTP 200)"
HOST="${HOST}" PORT="${HOST_PORT}" TIMEOUT_SECONDS="${TIMEOUT_SECONDS}" \
  "${SCRIPT_DIR}/check_health.sh"

echo "==> [6/7] Golden skill suite + malformed JSON fuzz"
BASE_URL="http://${HOST}:${HOST_PORT}" MASTER_API_2026="${MASTER_API_2026}" TIMEOUT_SECONDS="${TIMEOUT_SECONDS}" \
  "${SCRIPT_DIR}/run_skill_golden.sh"
BASE_URL="http://${HOST}:${HOST_PORT}" MASTER_API_2026="${MASTER_API_2026}" TIMEOUT_SECONDS="${TIMEOUT_SECONDS}" \
  "${SCRIPT_DIR}/fuzz_payloads.sh"

cleanup

if [[ "${RUN_MEMORY}" == "1" ]]; then
  echo "==> [7/7] 512MB memory stress check"
  IMAGE="${IMAGE}" PORT="${PORT}" HOST_PORT="${HOST_PORT}" MASTER_API_2026="${MASTER_API_2026}" TIMEOUT_SECONDS="${TIMEOUT_SECONDS}" \
    "${SCRIPT_DIR}/memory_stress.sh"
else
  echo "==> [7/7] Memory stress check skipped (RUN_MEMORY=${RUN_MEMORY})"
fi

echo "PASS: pre-deployment checks completed successfully."
