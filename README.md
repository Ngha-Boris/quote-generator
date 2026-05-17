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

```bash
docker compose up --build
```

## Run in Detached Mode

```bash
docker compose up -d
```

## Stop Containers

```bash
docker compose down
```

# Environment Variables

Example `.env` file:

```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=quotes_db

DATABASE_URL=postgres://postgres:postgres@postgres:5432/quotes_db

RUST_LOG=info
```
# CI/CD Pipeline

The project uses GitHub Actions for Continuous Integration and Continuous Deployment.

## Pipeline Workflow

Whenever code is pushed:

1. Dependencies are installed
2. Tests are executed
3. Docker images are built
4. Application is deployed

## CI/CD Goals

* Automated testing
* Automated builds
* Automated deployment
* Faster feedback loops
* Deployment consistency

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
* Valentine Fuh
* Victoire Motouom
