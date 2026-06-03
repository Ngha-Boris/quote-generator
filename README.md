A simple motivational quote generator application built to demonstrate a complete modern DevOps workflow.

The goal of this project is not only to build a small web application, but to showcase how modern software systems are developed, tested, containerized, deployed, monitored, and automated using DevOps best practices.

# Project Overview

The application allows users to generate and view random motivational quotes through a web interface.

When a user clicks the **"Generate New Quote"** button:

1. The frontend sends a request to the backend API
2. The backend fetches a random quote from the database
3. The quote is returned as JSON
4. The frontend displays the quote to the user

This project demonstrates:

* Frontend and Backend communication
* REST API development
* Database integration
* Docker containerization
* CI/CD automation
* Monitoring and observability
* Reverse proxy configuration
* Health checks and service monitoring

# Architecture Overview

```text
                ┌───────────────┐
                │     User      │
                └──────┬────────┘
                       │
                       ▼
                ┌───────────────┐
                │     Nginx     │
                │ Reverse Proxy │
                └──────┬────────┘
                       │
        ┌──────────────┴──────────────┐
        ▼                             ▼
┌───────────────┐            ┌────────────────┐
│   Frontend    │            │  Backend API   │
│ React / HTML  │◄──────────►│   Rust Axum    │
└───────────────┘            └──────┬─────────┘
                                    │
                                    ▼
                           ┌────────────────┐
                           │   PostgreSQL   │
                           │    Database    │
                           └────────────────┘

Monitoring Stack
────────────────────────────────────
Prometheus → Collect Metrics
Grafana   → Visualize Metrics
```

# Tech Stack

## Frontend

* React (or HTML/CSS/JavaScript)
* Axios / Fetch API

## Backend

* Rust
* Axum Framework
* Tokio Async Runtime
* Serde

## Database

* PostgreSQL

## DevOps & Infrastructure

* Docker
* Docker Compose
* Nginx
* GitHub Actions
* Ansible

## Monitoring

* Prometheus
* Grafana

# Features

## User Features

* View motivational quotes
* Generate random quotes
* Responsive UI
* Error handling for failed API requests

## Backend Features

* REST API
* Health check endpoint
* Random quote retrieval
* JSON responses
* Metrics exposure for monitoring

## DevOps Features

* Dockerized services
* Multi-container orchestration
* Automated CI/CD pipeline
* Monitoring dashboards
* Reverse proxy setup
* Health checks
* Environment consistency


# API Endpoints

## Get Random Quote

```http
GET /quotes/random
```

### Example Response

```json
{
  "quote": "Discipline beats motivation."
}
```

## Health Check

```http
GET /health
```

### Example Response

```json
{
  "status": "healthy"
}
```

# Docker Setup

All services run inside Docker containers.

This includes:

* Frontend
* Backend
* PostgreSQL
* Prometheus
* Grafana
* Nginx

## Start the Project

Before starting, copy the example environment file and configure it:
```bash
cp .env.example .env
```

Build and run all services:
```bash
docker compose up --build
```

Once running, you can access the application components:
- **Frontend App**: [http://localhost:8000](http://localhost:8000)
- **Random Quote API**: [http://localhost:8000/api/quotes/random](http://localhost:8000/api/quotes/random)
- **Health Check Endpoint**: [http://localhost:8000/api/health](http://localhost:8000/api/health)
- **Prometheus Dashboard**: [http://localhost:9090](http://localhost:9090)
- **Grafana Dashboard**: [http://localhost:3000](http://localhost:3000) (default credentials: `admin`/`admin`)

## Run in Detached Mode

```bash
docker compose up -d
```

## Stop Containers

```bash
docker compose down
```

# Ansible Deployment

We provide Ansible playbooks and roles for deploying and tearing down the application stack locally.

## Prerequisites

Ensure that you have Ansible installed:
```bash
# Check if Ansible is installed
ansible --version
```
*(If Ansible is not installed, you can install it using pip: `pip install ansible --user --break-system-packages`)*

## Directory Structure

The deployment files are organized in the `ansible/` directory:
- `ansible.cfg`: Configures default inventory, roles paths, and disables host key checking.
- `inventory/hosts.ini`: Specifies target hosts (configured to target `localhost` locally).
- `playbooks/deploy.yml`: Playbook to verify Docker, template `.env`, and start the stack.
- `playbooks/shutdown.yml`: Playbook to stop and remove all services, networks, and database volumes.
- `playbooks/group_vars/all.yml`: Stores configurable environment variables (like DB username, password, ports).

## Deploying the Stack

To build and start all containers, generate `.env`, and run health checks:
```bash
cd ansible
ansible-playbook playbooks/deploy.yml
```

This playbook will verify that Docker is running, dynamically template your `.env` file, spin up the Docker Compose stack, and block until all containers (Postgres, Backend, Nginx proxy) report a `healthy` state.

## Stopping the Stack

To stop all services and remove containers, networks, and database volumes (equivalent to `docker compose down -v`):
```bash
cd ansible
ansible-playbook playbooks/shutdown.yml
```


# Environment Variables

Example `.env` file (copy from `.env.example`):

```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=quotes_db

DATABASE_URL=postgres://postgres:postgres@postgres:5432/quotes_db

RUST_LOG=info
```

# CI/CD Pipeline

The project uses GitHub Actions for Continuous Integration and Continuous Deployment.

## Pipeline Structure

```text
Push / PR to main or develop
│
├──► backend-ci      ──┐
│    (rustfmt,        │
│     clippy,         │
│     test, build)    │
│                      ├──► docker-build ──┐
├──► frontend-ci     ──┤   (build+push    │
│    (lint, test,     │    Docker images  ├──► deploy (main only)
│     build)          │    to GHCR)      │    (SSH → snapshot images
│                      │                  │     → pull → zero-downtime
│                      ├──► integration  │     restart → health check
│                      │    test         │     → auto-rollback on fail)
│                      │    (Compose up, │
│                      │     health+API) │
│                      │                 │
│                      │      Manual ──── ├──► rollback workflow
│                      │      trigger     │    (pull SHA image →
                                       │     re-tag → restart →
                                       │     health check)
```

## Jobs

| Job | Trigger | Permissions | Purpose |
|-----|---------|-------------|---------|
| `backend-ci` | All events | `contents: read` | rustfmt, clippy, `cargo test`, release build |
| `frontend-ci` | All events | `contents: read` | npm lint, unit tests, production build |
| `docker-build` | Push only | `contents: read`, `packages: write` | Build & push images to GHCR (with `sha-` tags) |
| `integration-test` | Push only | `contents: read` | Full Compose stack health + API smoke test |
| `deploy` | Push to main | `{} ` (none) | SSH deploy with snapshot, health check & rollback |
| `rollback` | Manual dispatch | `{}` (none) | Revert to a specific SHA-tagged image |

## Security & Hardening

- **Least-privilege permissions**: each job declares only what it needs; deploy and rollback jobs have zero permissions (`{}`)
- **Workflow-level default**: `permissions: contents: read` — no write access unless explicitly granted
- **GitHub Environment**: `deploy` and `rollback` target the `production` environment — supports required reviewers and environment secrets
- **Secret handling**: `DEPLOY_HOST`, `DEPLOY_USER`, `DEPLOY_SSH_KEY`, `DEPLOY_PATH` stored as secrets — never logged; image hashes are truncated in logs
- **SSH host key verification**: `fingerprint` parameter uses `SSH_HOST_FINGERPRINT` secret to prevent MITM attacks
- **SSH key safety**: `passphrase` explicitly empty; `script_stop: true` aborts on first command failure; `command_timeout: 10m` prevents hangs
- **Concurrency group**: `deploy-production` shared between deploy and rollback — prevents simultaneous deploys; non-main branches cancel in-progress runs
- **No checkout in deploy**: the deploy job does not check out code — reduces attack surface and CI minutes
- **Image pruning**: dangling images pruned after successful deploy/rollback (24h filter) to prevent disk exhaustion

## Deployment Flow

1. SSH into production server (with host fingerprint verification)
2. **Snapshot current container image references**: record content hash + repo:tag for each service
3. `docker compose pull` — fetch latest images from GHCR
4. `docker compose up -d --remove-orphans` — zero-downtime restart (new containers replace old)
5. Health check with retry (24 attempts × 5 s = 120 s) against configurable `HEALTH_CHECK_URL`
6. On success: prune dangling images, write summary
7. **On failure — automatic rollback**:
   - Re-tag old images (still on disk, identified by content hash) back to their original repo:tag
   - `docker compose up -d --force-recreate --remove-orphans` — restart with previous images
   - Exit with failure code so GitHub marks the job as failed
8. Job summary written to `$GITHUB_STEP_SUMMARY` on both success and failure

## Manual Rollback

The `rollback.yml` workflow can be triggered manually from the GitHub Actions UI:

1. Provide the target commit SHA (7+ characters)
2. Pulls the SHA-tagged images from GHCR (`sha-<commit>`)
3. Re-tags them to match the docker-compose.yml image references
4. Restarts services and runs a health check

This is useful when:
- Automatic rollback failed or was skipped
- You need to revert to a specific earlier version
- You need to re-deploy a known-good commit

## Required Secrets

| Secret | Description |
|--------|-------------|
| `DEPLOY_HOST` | Production server hostname or IP |
| `DEPLOY_USER` | SSH username for deployment |
| `DEPLOY_SSH_KEY` | Private SSH key (ed25519 recommended, no passphrase) |
| `DEPLOY_PATH` | Absolute path to docker-compose project on server |
| `SSH_HOST_FINGERPRINT` | SSH host key fingerprint for server verification |

## Required Variables

| Variable | Description |
|----------|-------------|
| `DEPLOY_URL` | Public URL of the production environment (shown in GitHub Environment) |
| `HEALTH_CHECK_URL` | URL for post-deploy health verification (default: `http://localhost/api/health`) |

## Setting Up SSH_HOST_FINGERPRINT

```bash
ssh-keyscan -t ed25519 your-server.com | ssh-keygen -lf -
```

Copy the fingerprint string and add it as a repository secret.

## CI/CD Goals

* Automated testing
* Automated builds
* Automated deployment
* Faster feedback loops
* Deployment consistency
* Zero-downtime restarts
* Automatic rollback on failure (with manual fallback)
* Supply chain traceability (SHA-tagged images)

# Monitoring & Observability

## Prometheus

Prometheus collects metrics such as:

* CPU usage
* Memory usage
* API request count
* Response times
* Container health

## Grafana

Grafana visualizes metrics through dashboards.

Example dashboards:

* API Requests Per Minute
* CPU Usage
* RAM Usage
* Container Status
* Backend Response Time

# Reverse Proxy

Nginx acts as the reverse proxy for the platform.

Responsibilities include:

* Routing requests
* Forwarding traffic
* Improving security
* Serving frontend traffic
* API proxying
# Health Checks

The backend exposes a health endpoint:

```http
GET /health
```

This endpoint is used for:

* Container health checks
* CI/CD validation
* Monitoring systems
* Uptime monitoring

# Development Workflow

## 1. Local Development

Develop features locally.

## 2. Version Control

Push code to GitHub.

```bash
git push
```

## 3. CI/CD Pipeline

GitHub Actions automatically:

* Runs tests
* Builds Docker containers
* Deploys services

## 4. Runtime Environment

Docker Compose runs all services together.

## 5. Monitoring

Prometheus collects metrics and Grafana visualizes system health.

# Future Improvements

* User authentication
* Admin dashboard
* Quote categories
* Search functionality
* Kubernetes deployment
* HTTPS support
* Load balancing
* Caching with Redis
* Rate limiting
* Distributed tracing

# Learning Objectives

This project demonstrates practical experience with:

* Rust backend development
* REST APIs
* PostgreSQL integration
* Docker containerization
* Infrastructure automation
* CI/CD pipelines
* Monitoring and observability
* Reverse proxy configuration
* DevOps best practices

# Team Members

* Boris Ngha
* Valantine Fuh
* Victoire Motouom
