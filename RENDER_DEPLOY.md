# Render Docker Deployment Guide (`dsaengine`)

This guide deploys `dsaengine` to Render using a prebuilt Docker image.

## 1. Pre-Deploy Local Checks

Run these from repo root:

```bash
cargo check
```

Optional full gate (recommended):

```bash
export MASTER_API_2026="replace-with-strong-secret"
chmod +x scripts/*.sh
./scripts/predeploy_check.sh
```

## 2. Build and Push `linux/amd64` Image

Render requires `linux/amd64`.

```bash
docker buildx build \
  --platform linux/amd64 \
  -t <dockerhub-user>/dsaengine:latest \
  --push .
```

Verify pushed image architecture:

```bash
docker buildx imagetools inspect <dockerhub-user>/dsaengine:latest
```

Expected to include: `linux/amd64`

## 3. Create Render Web Service

In Render Dashboard:

1. `New` -> `Web Service`
2. Choose `Deploy an existing image from a registry`
3. Image URL:

```text
docker.io/<dockerhub-user>/dsaengine:latest
```

## 4. Render Service Settings

Set:

- Runtime: `Docker`
- Health Check Path: `/health`
- Instance type: `Free` (or as needed)

Notes:

- App binds using `PORT` automatically (already implemented in code).
- Do not hardcode port in Render; Render injects `PORT`.

## 5. Required Environment Variables

Add this key variable:

- `MASTER_API_2026=<strong-secret>`

`X-API-KEY` request header must match this value.

## 6. Post-Deploy Verification

Replace `<render-url>` and `<secret>` below.

Health check:

```bash
curl -sS -o /dev/null -w "code=%{http_code} time=%{time_total}s\n" \
  --max-time 5 \
  "https://<render-url>/health"
```

Secured endpoint check:

```bash
curl -X POST "https://<render-url>/api/v1/graphs/dijkstra" \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: <secret>" \
  -d '{"num_nodes":5,"source":0,"destination":4,"edges":[[0,1,4],[1,2,3],[0,3,2],[3,4,1],[2,4,5]]}'
```

Expected:

- `/health` -> HTTP `200` within `5s`
- Dijkstra endpoint -> HTTP `200` with JSON `status: "success"`

## 7. Update Flow (New Release)

1. Build and push new image tag:

```bash
docker buildx build --platform linux/amd64 -t <dockerhub-user>/dsaengine:vX.Y.Z --push .
```

2. In Render service, update image to `vX.Y.Z`
3. Deploy
4. Re-run verification commands

## 8. Troubleshooting

### Service fails to start / no incoming traffic

- Confirm listener is `0.0.0.0:$PORT` (already implemented in `src/web_server.rs`).
- Ensure image is `linux/amd64`.

### 401 Unauthorized on API routes

- Ensure Render env var `MASTER_API_2026` is set.
- Ensure request has exact header:

```text
X-API-KEY: <same-secret>
```

### Health check timeout

- Check logs for startup delays.
- Ensure health path is exactly `/health`.
- Consider reducing startup work and testing cold starts locally with:

```bash
./scripts/memory_stress.sh
```
