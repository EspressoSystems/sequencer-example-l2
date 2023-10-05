default:
    just --list

demo *args:
    docker compose up {{args}}

down *args:
    docker compose down {{args}}

docker-cli *cmd:
    docker exec -it espresso-sequencer-example-rollup-1 bin/cli {{cmd}}

cli *cmd:
    target/release/cli {{cmd}}

pull:
    docker compose pull

bindings *args:
    forge bind --bindings-path contract-bindings --select Example --crate-name "contract-bindings" {{args}}

docker-stop-rm:
    docker stop $(docker ps -aq); docker rm $(docker ps -aq)

anvil *args:
    docker run -p 127.0.0.1:8545/8545 ghcr.io/foundry-rs/foundry:latest "anvil {{args}}"

test:
    cargo test --release --all-features

dev-demo:
     cargo run --bin example-l2
