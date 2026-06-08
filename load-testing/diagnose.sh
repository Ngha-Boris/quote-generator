#!/bin/bash
# Diagnostic script for k6 + InfluxDB + Grafana troubleshooting

echo "=== k6 Load Testing Diagnostics ==="
echo

echo "1. Checking if InfluxDB is running..."
docker compose ps influxdb
echo

echo "2. Checking InfluxDB logs (last 20 lines)..."
docker compose logs --tail 20 influxdb
echo

echo "3. Testing Grafana -> InfluxDB connection..."
docker compose exec grafana wget -qO- http://influxdb:8086/ping 2>/dev/null || echo "Connection failed"
echo

echo "4. Checking if 'k6' database exists..."
docker compose exec influxdb influx -execute "SHOW DATABASES"
echo

echo "5. Checking measurements in k6 database..."
docker compose exec influxdb influx -database k6 -execute "SHOW MEASUREMENTS"
echo

echo "6. Checking recent data in http_reqs..."
docker compose exec influxdb influx -database k6 -execute "SELECT * FROM http_reqs LIMIT 5"
echo

echo "7. Testing k6 can reach backend..."
docker compose run --rm k6 wget -qO- http://nginx:80/api/health 2>/dev/null || echo "Cannot reach backend"
echo

echo "=== Diagnostics complete ==="
echo
echo "If you see 'k6' database and measurements above, data should appear in Grafana after running a test."
