import http from 'k6/http';
import { check, sleep, group } from 'k6/metrics';
import { Rate, Trend } from 'k6/metrics';

// Stress test - find breaking point
export const options = {
  stages: [
    { duration: '1m', target: 100 },     // Ramp to 100
    { duration: '2m', target: 200 },     // Ramp to 200
    { duration: '2m', target: 400 },     // Ramp to 400
    { duration: '2m', target: 600 },     // Ramp to 600
    { duration: '2m', target: 800 },     // Ramp to 800
    { duration: '2m', target: 1000 },    // Ramp to 1000
    { duration: '2m', target: 0 },       // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<1000'],    // 95% under 1s under load
  },
  noConnectionReuse: true,               // Don't reuse connections for max stress
};

const BASE_URL = __ENV.BASE_URL || 'http://nginx:80';

const errorRate = new Rate('errors');
const responseTime = new Trend('response_time');

export default function () {
  group('Stress - Random Quote', () => {
    const res = http.get(`${BASE_URL}/api/quotes/random`);
    const success = check(res, {
      'status is 200': (r) => r.status === 200,
      'response time < 2s': (r) => r.timings.duration < 2000,
    });
    errorRate.add(!success);
    responseTime.add(res.timings.duration);
  });

  sleep(Math.random() * 0.5);  // Random sleep 0-500ms
}
