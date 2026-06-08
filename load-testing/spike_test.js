import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

// Spike test - sudden massive load increase
export const options = {
  stages: [
    { duration: '10s', target: 10 },      // Normal load
    { duration: '10s', target: 10 },      // Hold
    { duration: '10s', target: 500 },     // SPIKE to 500 users
    { duration: '1m', target: 500 },       // Hold spike
    { duration: '10s', target: 10 },       // Drop back
    { duration: '1m', target: 10 },       // Recovery period
    { duration: '10s', target: 0 },         // Ramp down
  ],
  thresholds: {
    http_req_failed: ['rate<0.5'],         // Allow higher errors during spike
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://nginx:80';

const errorRate = new Rate('errors');

export default function () {
  const res = http.get(`${BASE_URL}/api/quotes/random`);
  errorRate.add(!check(res, {
    'status is 200 or 503': (r) => r.status === 200 || r.status === 503,
  }));
  sleep(0.1);
}
