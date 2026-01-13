@echo off
cargo build --release ^
    --target x86_64-pc-windows-msvc ^
    -Z build-std=core,panic_abort ^
    -Z build-std-features=default,optimize_for_size