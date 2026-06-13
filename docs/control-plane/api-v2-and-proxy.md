# API v2 and proxy routing

**Flintlock proposal 03**

## Base URL

| Environment | URL |
|---|---|
| Kind dev | `http://127.0.0.1:14000` (Tilt forward) |
| Production | `https://regenesis-hub.mgmt.example/v2` |

No Flintlock `/api/v1` compatibility.

## Endpoints (target)

### Nodes

| Method | Path | Description |
|---|---|---|
| POST | `/v2/register` | New resurrection node |
| POST | `/v2/nodes/rejoin` | Partition recovery |
| GET | `/v2/nodes` | Fleet capacity |
| GET | `/v2/nodes/{id}` | Node detail |
| POST | `/v2/nodes/{id}/drain` | Begin migration drain |

### Agents

| Method | Path | Description |
|---|---|---|
| POST | `/v2/agents` | Create agent microVM |
| GET | `/v2/agents` | List registry |
| GET | `/v2/agents/{id}` | Hub registry view |
| GET | `/v2/agents/{id}/status` | **Proxy** to host |
| POST | `/v2/agents/{id}/hibernate` | CryoSleep |
| POST | `/v2/agents/{id}/resurrect` | Restore from snapshot |

## Proxy routing

```
GET /v2/agents/{id}/status
  1. Hub lookup agent.node_id
  2. Hub mTLS gRPC → node GetCylonVm / metrics
  3. Return merged response
```

Latency budget: +20ms p99 vs direct host query.

## Auth

| Caller | Auth |
|---|---|
| Platform daemon | mTLS client cert (hub-client) |
| Resurrection node | mTLS + node identity |
| Human admin | OIDC gateway (future) |

## OpenAPI

Future: `cylon-regenesis/openapi/regenesis-v2.yaml` generated from axum handlers.

## References

- `tiffany/crates/resurrection-hub/src/api.rs`
- `tiffany/crates/cylon/proto/cylon.proto`
