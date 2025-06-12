run_server:
	cargo run --package instant_chat --bin instant-chat-server -- --valkey-password="${VALKEY_PASSWORD}"

username ?= devin
run_client:
	cargo run --package instant_chat --bin instant-chat-client -- --username $(username)

grpcurl_list:
	grpcurl -v -plaintext localhost:50051 list

run_valkey:
	podman run -d --rm --name valkey \
		-p 6379:6379 \
		docker.io/valkey/valkey:8-alpine \
		valkey-server --requirepass "${VALKEY_PASSWORD}"

connect_valkey:
	podman exec -it valkey valkey-cli -a "${VALKEY_PASSWORD}"
