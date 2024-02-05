#!/usr/bin/env fish
if set -q argv[1]
    cargo run $argv[1]
    code $argv[1]
else
    echo "Parameter missing!"
end
