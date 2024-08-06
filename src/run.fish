#!/usr/bin/env fish
if set -q argv[1]
    cargo run $argv[1]
    firefox $argv[1]
else
    echo "Output directory is missing. Set to ./output/image.jpg by default."
    cargo run ../output/image.jpg
    firefox ../output/image.jpg
end
