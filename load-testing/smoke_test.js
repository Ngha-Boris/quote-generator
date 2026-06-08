import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Rate } from 'k6/metrics';

// Smoke test - minimal load to verify system works
export const options = {
  vus: 2,           // 2 virtual users
  duration: '30s',  // Run for 30 seconds
  thresholds: {
    http_req_duration: ['p(95)<200'],   // 95% under 200ms
    http_req_failed: ['rate<0.01'],     // Less than 1% errors
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://nginx:80';

const errorRate = new Rate('errors');

export default function () {
  group('Smoke Test - All Endpoints', () => {
    // Health check
    let res = http.get(`${BASE_URL}/api/health`);
    errorRate.add(!check(res, {
      'health is 200': (r) => r.status === 200,
    }));

    // Random quote
    res = http.get(`${BASE_URL}/api/quotes/random`);
    errorRate.add(!check(res, {
      'random quote is 200': (r) => r.status === 200,
    }));

    // All quotes
    res = http.get(`${BASE_URL}/api/quotes`);
    errorRate.add(!check(res, {
      'all quotes is 200': (r) => r.status === 200,
    }));
  });

  sleep(1);
}
