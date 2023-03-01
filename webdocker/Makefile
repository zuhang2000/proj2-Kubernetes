install:
	cargo clean &&\
		cargo build -j 1

build:
	docker build -t todo .

rundocker:
	docker run -it --rm -p 8000:8000 todo

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run