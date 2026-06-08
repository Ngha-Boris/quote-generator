import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const responseTime = new Trend('response_time');
const requestsTotal = new Counter('requests_total');

// Test configuration
export const options = {
  stages: [
    // Ramp up
    { duration: '30s', target: 10 },    // 10 users
    { duration: '30s', target: 50 },    // 50 users
    // Steady state
    { duration: '1m', target: 50 },       // Hold at 50 users
    { duration: '2m', target: 100 },     // Ramp to 100 users
    { duration: '2m', target: 100 },     // Hold at 100 users
    // Ramp down
    { duration: '30s', target: 0 },       // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'],     // 95% of requests under 500ms
    http_req_failed: ['rate<0.1'],        // Less than 10% errors
    errors: ['rate<0.1'],
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://nginx:80';

export default function () {
  group('Health Check', () => {
    const res = http.get(`${BASE_URL}/api/health`);
    const success = check(res, {
      'health status is 200': (r) => r.status === 200,
      'health response is healthy': (r) => r.json('status') === 'healthy',
    });
    errorRate.add(!success);
    responseTime.add(res.timings.duration);
    requestsTotal.add(1);
    sleep(1);
  });

  group('Get Random Quote', () => {
    const res = http.get(`${BASE_URL}/api/quotes/random`);
    const success = check(res, {
      'random quote status is 200': (r) => r.status === 200,
      'random quote has id': (r) => r.json('id') !== undefined,
      'random quote has quote text': (r) => r.json('quote') !== undefined,
      'random quote has author': (r) => r.json('author') !== undefined,
    });
    errorRate.add(!success);
    responseTime.add(res.timings.duration);
    requestsTotal.add(1);
    sleep(1);
  });

  group('Get All Quotes', () => {
    const res = http.get(`${BASE_URL}/api/quotes`);
    const success = check(res, {
      'all quotes status is 200': (r) => r.status === 200,
      'all quotes returns array': (r) => Array.isArray(r.json()),
    });
    errorRate.add(!success);
    responseTime.add(res.timings.duration);
    requestsTotal.add(1);
    sleep(1);
  });
}
