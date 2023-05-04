.PHONY: test
test: src
	@cargo test --features=async_api