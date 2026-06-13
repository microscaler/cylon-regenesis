# Observability

**Flintlock proposal 13**

## Trace propagation

```
Portal HTTP
  → regenesis-hub axum (span: hub.http)
    → OpenRaft commit (span: hub.raft)
      → tonic gRPC to cylon host (W3C traceparent)
        → Firecracker UDS (span: fc.api)
          → vsock guest (span: guest.vsock)
```

Implementation: OpenTelemetry SDK in hub + cylon host ([RESURRECTION-HUB-PRD](../../cylon/docs/RESURRECTION-HUB-PRD.md) 1.1).

## Metrics

### Hub (Prometheus)

| Metric | Type | Labels |
|---|---|---|
| `regenesis_agents_total` | gauge | state |
| `regenesis_nodes_online` | gauge | site |
| `regenesis_bid_duration_seconds` | histogram | |
| `regenesis_raft_commit_latency_seconds` | histogram | |
| `regenesis_proxy_requests_total` | counter | endpoint, status |

### Host (cylon extension)

| Metric | Type |
|---|---|
| `cylon_running_vms` | gauge |
| `cylon_oci_pull_duration_seconds` | histogram |
| `cylon_detached_mode` | gauge 0/1 |

## Logging

- Structured JSON (`tracing` subscriber)
- Correlation ID = trace_id

## Dashboards

Grafana boards (future):

- CRP fleet overview
- Scheduling latency
- Regenesis boot success rate (from regenesis-agent)

## regenesis-agent events

| Event | Level |
|---|---|
| `regenesis.first_boot.start` | info |
| `regenesis.register.success` | info |
| `regenesis.ghcr.pull_failed` | error |

## References

- `cylon/crates/resurrection-hub/`
- DCops observability (separate)
