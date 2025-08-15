#!/usr/bin/env bash

echo "rustc client.rs --cfg 'feature=\"debug\"' -o client"
rustc client.rs --cfg 'feature="debug"' -o client
echo 'RUNNING IN DEBUG MODE'

# echo "rustc client.rs -o client"
# rustc client.rs  -o client

echo './client 127.0.0.1 4242'
./client 127.0.0.1 4242
