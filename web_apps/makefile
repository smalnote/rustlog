run_metrics:
	cargo run -p web_apps --bin prometheus_metrics_example
docker_image:
	docker build . -t prometheus_metrics_example:v0.1
docker_run:
	docker run --rm --name metrics -p 3000:3000 -p 3001:3001 -dit prometheus_metrics_example:v0.1
stress_test:
	k6 run ./k6_test/stress.js
