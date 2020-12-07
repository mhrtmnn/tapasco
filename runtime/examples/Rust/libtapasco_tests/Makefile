HOST = tapasco@192.168.2.137
TEST = target/aarch64-unknown-linux-gnu/debug/libtapasco_tests

compile:
	cargo build --target aarch64-unknown-linux-gnu

deploy:
	scp $(TEST) $(HOST):~/myArraySum/
