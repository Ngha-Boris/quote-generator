# Load Testing for Quote Generator

This directory contains load testing scripts using [k6](https://k6.io/) to test the performance and reliability of the Quote Generator API. Results are automatically stored in InfluxDB and visualized in Grafana.

## Test Types

| Test | Script | Purpose | Duration |
|------|--------|---------|----------|
| Smoke | `smoke_test.js` | Verify basic functionality works | 30s |
| Load | `load_test.js` | Normal expected traffic simulation | ~6min |
| Stress | `stress_test.js` | Find system breaking points | ~11min |
| Spike | `spike_test.js` | Sudden traffic spikes | ~3min |
| Soak | `soak_test.js` | Long-term stability testing | ~40min |

## Quick Start

### 1. Start all services (including InfluxDB for test results)

```bash
docker compose up -d
```

This starts:
- PostgreSQL database
- Rust backend API
- Frontend (Nginx)
- Prometheus (metrics)
- **InfluxDB** (k6 test results)
- Grafana (dashboards)

### 2. Run a load test

```bash
# Quick smoke test (30 seconds)
docker compose run --rm k6 run /scripts/smoke_test.js

# Full load test (~6 minutes)
docker compose run --rm k6 run /scripts/load_test.js

# Stress test to find breaking point (~11 minutes)
docker compose run --rm k6 run /scripts/stress_test.js

# Spike test (~3 minutes)
docker compose run --rm k6 run /scripts/spike_test.js

# Soak test for stability (~40 minutes)
docker compose run --rm k6 run /scripts/soak_test.js
```

### 3. View results in Grafana

Open http://localhost:3000 (admin/admin) → **"k6 Load Testing Results"** dashboard

**Results appear automatically** as the test runs. No manual setup needed!

## Dashboard Overview

The pre-configured dashboard shows:
- **Virtual Users** - Current active users
- **Requests/sec** - Request rate
- **Error Rate** - Failed requests percentage
- **p95/p99 Latency** - Response time percentiles
- **Test Duration** - How long the test ran
- **Time series graphs** - VUs, RPS, latencies over time
- **HTTP Status Codes** - Response code distribution

## Using Makefile (Recommended)

```bash
# Start all services
make up

# Run tests
make test-smoke      # Quick verification
make test-load       # Normal traffic simulation
make test-stress     # Find breaking point
make test-spike      # Sudden traffic spike
make test-soak       # Long-term stability

# Open dashboards
make grafana         # Open Grafana (admin/admin)
make prometheus      # Open Prometheus
```

## How It Works

```
┌─────────┐    HTTP API     ┌──────────┐
│   k6    │ ───────────────►│  Nginx   │
│ (tests) │                 │ (app)    │
└────┬────┘                 └──────────┘
     │
     │ writes metrics
     ▼
┌──────────┐    queries    ┌──────────┐
│ InfluxDB │ ◄──────────────│ Grafana  │
│  (k6 db) │               │(dashboard│
└──────────┘               └──────────┘
```

1. **k6** runs tests against the API
2. **k6** writes real-time metrics to **InfluxDB**
3. **Grafana** queries InfluxDB and displays the dashboard
4. Dashboard auto-refreshes every 5 seconds during tests

## Alternative: Local k6 Installation

Install k6 locally:
```bash
# macOS
brew install k6

# Linux
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D786D77FF9D7444F0E470
sudo bash -c 'echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" > /etc/apt/sources.list.d/k6.list'
sudo apt-get update && sudo apt-get install k6
```

Run tests locally (outputs to console only):
```bash
export BASE_URL=http://localhost:8000
k6 run smoke_test.js
```

To send local test results to InfluxDB/Grafana:
```bash
export K6_OUT=influxdb=http://localhost:8086/k6
k6 run load_test.js
```

## Test Configuration

All tests support environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `BASE_URL` | `http://nginx:80` | Target API base URL |

## Interpreting Results

### Key Metrics

| Metric | Good | Warning | Critical |
|--------|------|---------|----------|
| p95 Latency | < 200ms | 200-500ms | > 500ms |
| Error Rate | < 1% | 1-5% | > 5% |
| RPS | Matches target | 10% below | > 20% below |

### Test Thresholds

Each test defines success criteria:
- **Smoke:** p95 < 200ms, errors < 1%
- **Load:** p95 < 500ms, errors < 10%
- **Stress:** Used to find breaking point (higher error tolerance)
- **Spike:** Quick recovery verification
- **Soak:** Consistent performance over 40 minutes

## API Endpoints Tested

- `GET /api/health` - Health check
- `GET /api/quotes/random` - Random quote
- `GET /api/quotes` - All quotes

## Troubleshooting

### Connection refused errors
Ensure the API is accessible:
```bash
curl http://localhost:8000/api/health
```

### Dashboard not showing data
1. Check InfluxDB is running:
   ```bash
   docker compose ps influxdb
   ```

2. Verify data is being written:
   ```bash
   docker compose exec influxdb influx -database k6 -execute "SHOW MEASUREMENTS"
   ```

3. Check Grafana data source:
   - Configuration → Data Sources → InfluxDB-k6
   - URL: `http://influxdb:8086`
   - Database: `k6`

### Out of memory during stress tests
Increase container limits in docker-compose.yml:
```yaml
backend:
  deploy:
    resources:
      limits:
        memory: 512M
```

### k6 container can't reach backend
Verify network connectivity:
```bash
docker compose run --rm k6 wget -O- http://nginx:80/api/health
```
