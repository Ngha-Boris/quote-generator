.PHONY: help up down test-smoke test-load test-stress test-spike test-soak test-all clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

up: ## Start all services with docker compose
	docker compose up -d

down: ## Stop all services
	docker compose down

logs: ## View logs from all services
	docker compose logs -f

test-smoke: ## Run smoke test (30s, minimal load)
	docker compose run --rm k6 run /scripts/smoke_test.js

test-load: ## Run load test (~6min, simulates normal traffic)
	docker compose run --rm k6 run /scripts/load_test.js

test-stress: ## Run stress test (~11min, finds breaking point)
	docker compose run --rm k6 run /scripts/stress_test.js

test-spike: ## Run spike test (~3min, sudden traffic spike)
	docker compose run --rm k6 run /scripts/spike_test.js

test-soak: ## Run soak test (~40min, long-term stability)
	docker compose run --rm k6 run /scripts/soak_test.js

test-all: ## Run all tests sequentially (smoke, load, stress, spike)
	@echo "=== Running Smoke Test ==="
	docker compose run --rm k6 run /scripts/smoke_test.js
	@echo "=== Running Load Test ==="
	docker compose run --rm k6 run /scripts/load_test.js
	@echo "=== Running Stress Test ==="
	docker compose run --rm k6 run /scripts/stress_test.js
	@echo "=== Running Spike Test ==="
	docker compose run --rm k6 run /scripts/spike_test.js
	@echo "=== All tests complete! View results at http://localhost:3000 ==="

clean: ## Clean up docker volumes and containers
	docker compose down -v
	docker system prune -f

ps: ## Show running containers
	docker compose ps

prometheus: ## Open Prometheus UI
	@echo "Opening http://localhost:9090"
	@xdg-open http://localhost:9090 2>/dev/null || open http://localhost:9090 2>/dev/null || echo "Open http://localhost:9090"

grafana: ## Open Grafana UI
	@echo "Opening http://localhost:3000 (admin/admin)"
	@xdg-open http://localhost:3000 2>/dev/null || open http://localhost:3000 2>/dev/null || echo "Open http://localhost:3000 (admin/admin)"
