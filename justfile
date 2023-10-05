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
     target/release/example-l2 --sequencer-url http://localhost:8083 \
     --l1-http-provider http://localhost:8545 \
     --l1-ws-provider ws://localhost:8546 \
     --hotshot-address 0x5fbdb2315678afecb367f032d93f642f64180aa3 \
     --rollup-mnemonic "test test test test test test test test test test test junk"
