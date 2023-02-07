sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid'
sudo sysctl -w kernel.perf_event_paranoid=1
cargo build --release 
RUST_BACKTRACE=1 perf record ./target/release/voxel_game
