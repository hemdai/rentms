install:
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --geatures derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add rand_core --features "std"
	cargo install sqlx-cli

build:
	cargo build

docker_build_restart:
	docker-compose up --build

docker_stop_all:
	docker stop $(docker ps -q)

docker_restart: docker_build_restart