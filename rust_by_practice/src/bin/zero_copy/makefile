.PHONY: *

tmpfs_large_file:
	sudo mount -t tmpfs -o size=20G tmpfs $(HOME)/zero_copy_sink
	dd if=/dev/urandom of=$(HOME)/zero_copy_sink/large_file bs=1G count=8

clean_tmpfs:
	rm -rf $(HOME)/zero_copy_sink/*
	sudo umount $(HOME)/zero_copy_sink

nc_unix_socket_server:
	(nc -lU /tmp/zero_copy.sock >/dev/null && rm /tmp/zero_copy.sock &) && sleep 0.5

method=""

copy_command:
	cargo run --release -p rust_by_practice --bin zero_copy $(method) ~/zero_copy_sink/large_file /tmp/zero_copy.sock

method_std_io_copy:
	$(eval method := std_io_copy)
test_std_io_copy: nc_unix_socket_server method_std_io_copy copy_command

method_read_write:
	$(eval method := read_write)
test_read_write: nc_unix_socket_server method_read_write copy_command

method_nix_sendfile:
	$(eval method := nix_sendfile)
test_nix_sendfile: nc_unix_socket_server method_nix_sendfile copy_command

libc_sendfile:
	$(eval method := libc_sendfile)
test_libc_sendfile: nc_unix_socket_server libc_sendfile copy_command

tokio_io_copy:
	$(eval method := tokio_io_copy)
test_tokio_io_copy: nc_unix_socket_server tokio_io_copy copy_command

