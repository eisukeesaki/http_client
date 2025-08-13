#!/usr/bin/env bash
rustc client.rs -o client
./client 127.0.0.1 4242
