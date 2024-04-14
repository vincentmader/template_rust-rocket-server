develop:
	cargo run
release:
	cargo run --release
docs:
	cargo doc --open
up:
	docker-compose up -d --build
down:
	docker-compose down
logs:
	docker-compose logs -f -t
