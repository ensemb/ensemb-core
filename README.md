# Dome.Red Oracle

How to run:

cargo run --release -- --socket-path ./o.sock run-server

Issue this command from another terminal:

echo "reload-lib ./target/release/libsolana_broker.so" | nc -U ./o.sock
