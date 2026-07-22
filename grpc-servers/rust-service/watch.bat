@echo off
setlocal

watchexec --shell=cmd -e rs,toml -w src -w Cargo.toml -i target -r --clear -- "cargo run"