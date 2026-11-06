# Linux System Dashboard

A dependency-free terminal dashboard for CPU, memory, network, block I/O, thermal readings, history, and threshold alerts.

**Language:** Rust stable, edition 2021  
**Platform:** Linux only  
**Status:** First-release learning project. See implemented scope and limitations below; this is not a production-completeness claim.

## Features

- Samples aggregate CPU load and available memory every second.
- Shows receive/transmit and block read/write rates.
- Rolling Unicode sparkline and 20-cell utilization bars.
- Reads exposed thermal-zone temperatures, if available.
- `--once` supports scripted smoke checks; `--alert NUMBER` sets the CPU/RAM threshold.

## Requirements and quick start

Clone the repository, enter its folder, and install the language/platform tools listed above. Commands run from the repository root. No credentials are required except for Backup Vault's explicitly generated secrets.

```sh
git clone https://github.com/netcodermike/linux-system-dashboard.git
cd linux-system-dashboard
cargo run --release
cargo run --release -- --once
cargo run --release -- --alert 80
```

## Verification

```sh
cargo test
```

Run `--once`, compare memory totals with `free -m`, and generate some CPU or network traffic. Run with `--alert 0` to see an alert. Parser unit tests use synthetic procfs records and do not require live hardware. Linux CI runs the tests and a live one-sample smoke check.

See [validation notes](docs/VALIDATION.md) for what was actually executed during authoring. A committed CI workflow is a future check until its run is observed successful.

## Architecture and data

`src/main.rs` contains parsers and a one-second sampling loop. `/proc/stat` supplies cumulative CPU ticks, `/proc/meminfo` supplies memory totals, `/proc/net/dev` supplies interface counters, `/proc/diskstats` supplies block sectors, and `/sys/class/thermal/*/temp` supplies optional temperatures. Deltas use saturating subtraction and actual elapsed time. Guest CPU ticks are not double-counted. Partitions, RAM disks, loop devices, and loopback networking are excluded.

No files, credentials, or external services are used. Most readings work as an ordinary user. The terminal is redrawn with ANSI sequences; use a Unicode terminal for the sparkline.

## Limitations and next steps

CPU and memory are system totals; there is no process list. Disk aggregation can double-count stacked devices such as device-mapper plus underlying disks. Network aggregation can double-count bridged traffic. Thermal sensors may be unavailable or meaningless in VMs. Alerts print inside the dashboard, without notification delivery or history persistence. Missing optional files display zero/omit readings. No macOS or Windows collector is included.

## Troubleshooting

On a non-Linux system the program exits with a platform explanation. Missing `/proc` inside a container requires an appropriately mounted procfs. Zero temperatures usually mean the host exposes no thermal zones. If the output contains escape sequences, use an ANSI-capable terminal. Stop with Ctrl+C.

## Development

Read the source entry point described above, make a focused change, run the verification command, and exercise the relevant manual scenario. [Contributing](CONTRIBUTING.md) explains bug reports and pull requests; [security notes](SECURITY.md) describe the trust assumptions. Runtime/generated files are ignored. No telemetry or paid API dependencies are included.

## License

[MIT](LICENSE), copyright 2026 netcodermike.
