# Rust Web Applications

## Prometheus Metrics Example

- Axum framework.
- Online docker image project: https://cartcart.coding.net/p/rustlog

## Debugging PodMonitor Metrics Collecting

```bash
# Get prometheus-stack operator configuration, search podMonitorSelector, release=monitoring
kubectl get prometheus -n monitoring -o yaml | less

# Grafana web port forward
kubectl -n monitoring port-forward svc/monitoring-grafana 8080:80

# Prometheus web port forward, Top bar -> Status -> Service Discovery
kubectl -n monitoring port-forward svc/monitoring-kube-prometheus-prometheus 9090:9090

# Example app for collecting metrics, port forward
kubectl -n default port-forward svc/prometheus-metrics-example 3000:3000 3001:3001
```

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PodMonitor
metadata:
  name: prometheus-metrics-example
  namespace: monitoring
  labels:
    release: monitoring
    app.kubernetes.io/name: web-apps
    app.kubernetes.io/component: prometheus-metrics-example
spec:
  namespaceSelector:
    matchNames:
    - default
  selector:
    matchLabels:
      app.kubernetes.io/component: prometheus-metrics-example
  podMetricsEndpoints:
  - port: metrics
    path: /metrics
```

## Querying pod monitor metrics

Grafana -> Explore -> Query by PromQL 

Query by job:`{job="monitoring/prometheus-metrics-example"}`

Query by metric name and job: `http_request_duration_microseconds_bucket{job="monitoring/prometheus-metrics-example"}`

Request latency P99: `histogram_quantile(0.99, http_request_duration_microseconds_bucket)`

histogram_quantile(0.99, sum(rate(http_request_duration_microseconds_bucket[30s])) by (le, instance))

### Pod/Container CPU Usage

```PromQL
rate(container_cpu_usage_seconds_total{pod=~"prometheus-metrics-example-.*", container="user-container"}[5m])
```


## Running stress tests

```bash
k6 run ./k6_test/stress.js

# or by predefined make target
make stress_test
```

### [Running distributed tests](https://grafana.com/docs/k6/latest/testing-guides/running-distributed-tests/)


Deploy k6 operator on Kubernetes cluster

```
git clone https://github.com/grafana/k6-operator && cd k6-operator
make deploy
kubectl get pod -n k6-operator-system
```

Add k6 test as ConfigMap

```bash
kubectl create configmap stress-test --from-file ./k6_test/stress.js

# run test
kubectl apply -f ./k6_test/k8s_stress.yaml

# watch test running
kubectl get testrun -w

# cleanup
kubectl delete -f ./k6_test/k8s_stress.yaml
```
