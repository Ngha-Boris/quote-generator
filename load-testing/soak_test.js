import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Rate, Counter } from 'k6/metrics';

// Soak test - extended duration to catch memory leaks or degradation
export const options = {
  stages: [
    { duration: '5m', target: 50 },      // Ramp up
    { duration: '30m', target: 50 },     // Hold for 30 minutes
    { duration: '5m', target: 0 },       // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'],     // 95% under 500ms
    http_req_failed: ['rate<0.05'],       // Less than 5% errors
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://nginx:80';

const errorRate = new Rate('errors');
const requestsTotal = new Counter('requests_total');

export default function () {
  group('Soak - API Endpoints', () => {
    // Random quote (most used endpoint)
    let res = http.get(`${BASE_URL}/api/quotes/random`);
    errorRate.add(!check(res, {
      'random quote is 200': (r) => r.status === 200,
    }));
    requestsTotal.add(1);

    sleep(0.5);

    // Health check every 10th iteration (approx)
    if (__ITER % 10 === 0) {
      res = http.get(`${BASE_URL}/api/health`);
      errorRate.add(!check(res, {
        'health is 200': (r) => r.status === 200,
      }));
    }
  });

  sleep(0.5);
}
