# Rust Web Applications

## Prometheus Metrics Example

- Axum framework.
- [Online docker image project](https://cartcart.coding.net/p/rustlog)

## Debugging PodMonitor Metrics Collecting

```bash
# Install prometheus by operator
helm install --namespace monitoring monitoring prometheus-community/kube-prometheus-stack

# Get prometheus-stack operator configuration, search podMonitorSelector, release=monitoring
kubectl get prometheus -n monitoring -o yaml | less

# Grafana web port forward
kubectl -n monitoring port-forward svc/monitoring-grafana 8080:80

# Prometheus web port forward, Top bar -> Status -> Service Discovery
kubectl -n monitoring port-forward svc/monitoring-kube-prometheus-prometheus 9090:9090

# Example app for collecting metrics, port forward
kubectl -n default port-forward svc/prometheus-metrics-example 3000:3000 3001:3001
```

Grafana & Prometheus service node port

```yaml
apiVersion: v1
kind: Service
metadata:
  annotations:
    meta.helm.sh/release-name: monitoring
    meta.helm.sh/release-namespace: monitoring
  labels:
    app.kubernetes.io/instance: monitoring
    app.kubernetes.io/managed-by: Helm
    app.kubernetes.io/name: grafana
    app.kubernetes.io/version: 11.2.0
    helm.sh/chart: grafana-8.5.1
  name: monitoring-grafana-external
  namespace: monitoring
spec:
  type: NodePort
  internalTrafficPolicy: Cluster
  ipFamilies:
  - IPv4
  ipFamilyPolicy: SingleStack
  ports:
  - name: http-web
    port: 80
    protocol: TCP
    targetPort: 3000
    # default range: 30000-32767
    nodePort: 31472
  selector:
    app.kubernetes.io/instance: monitoring
    app.kubernetes.io/name: grafana
  sessionAffinity: None

---

apiVersion: v1
kind: Service
metadata:
  annotations:
    meta.helm.sh/release-name: monitoring
    meta.helm.sh/release-namespace: monitoring
  labels:
    app: kube-prometheus-stack-prometheus
    app.kubernetes.io/instance: monitoring
    app.kubernetes.io/managed-by: Helm
    app.kubernetes.io/part-of: kube-prometheus-stack
    app.kubernetes.io/version: 62.7.0
    chart: kube-prometheus-stack-62.7.0
    heritage: Helm
    release: monitoring
    self-monitor: "true"
  name: monitoring-kube-prometheus-prometheus-external
  namespace: monitoring
spec:
  type: NodePort
  internalTrafficPolicy: Cluster
  ipFamilies:
  - IPv4
  ipFamilyPolicy: SingleStack
  ports:
  - name: http-web
    port: 9090
    protocol: TCP
    targetPort: 9090
    nodePort: 31776
  - appProtocol: http
    name: reloader-web
    port: 8080
    protocol: TCP
    targetPort: reloader-web
    nodePort: 31766
  selector:
    app.kubernetes.io/name: prometheus
    operator.prometheus.io/name: monitoring-kube-prometheus-prometheus
  sessionAffinity: None
```

### Enable prometheus remote write receiver

Edit resource `prometheus`

```bash
kubectl -n monitoring edit prometheus
```

Add additional arguments

```yaml
name: monitoring-kube-prometheus-prometheus
namespace: monitoring
spec:
  additionalArgs:
  - name: web.enable-remote-write-receiver
    value: ""
```

```bash
curl -X POST http://k8s0.devin.lan:31776/api/v1/write
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

```bash
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

### Instrument k6 write metrics to prometheus

[Example](https://github.com/keptn-sandbox/k6-service/blob/main/docs/k6-prometheus-example/README.md)

```yaml
apiVersion: k6.io/v1alpha1
kind: TestRun
metadata:
  name: stress-test
spec:
  runner:
    image: docker.io/jvenom/k6-prometheus:latest
    imagePullPolicy: IfNotPresent
    resources:
      limits:
        cpu: "500m"
        memory: "512Mi"
      requests:
        cpu: "250m"
        memory: "256Mi"
    env:
    - name: "K6_PROMETHEUS_REMOTE_URL"
      value: "http://monitoring-kube-prometheus-prometheus.monitoring.svc.smalkube.lan:9090/api/v1/write"
  parallelism: 8
  script:
    configMap:
      name: stress-test
      file: stress.js
```
