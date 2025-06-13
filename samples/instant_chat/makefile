run_server:
	cargo run --package instant_chat --bin instant-chat-server -- \
		--valkey-password="${VALKEY_PASSWORD}" \
		--tls-cert="./devin.lan.crt" --tls-key="./devin.lan.key" \
		--log-level="instant_chat=trace"

username ?= devin
chatroom ?= public
run_client:
	cargo run --package instant_chat --bin instant-chat-client -- \
		--addr https://arc.devin.lan:50051 \
		--tls-ca="./devin-rootCA.pem" \
		--username $(username) --chatroom $(chatroom)

grpcurl_list:
	grpcurl -v -cacert "./devin-rootCA.pem" -plaintext arc.devin.lan:50051 list

run_valkey:
	podman run -d --rm --name valkey \
		-p 6379:6379 \
		docker.io/valkey/valkey:8-alpine \
		valkey-server --requirepass "${VALKEY_PASSWORD}"

connect_valkey:
	podman exec -it valkey valkey-cli -a "${VALKEY_PASSWORD}"

release:
	cargo build --release --package instant_chat
