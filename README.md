# Wire Util

A command-line tool to calculate voltage drop across different wire gauges based on voltage, current, and distance.

## Features

- Calculate voltage drop for 19 different wire gauges (28 AWG to 0000 AWG)
- Filter results to specific gauges using the `--gauges` argument
- Configurable maximum acceptable voltage drop percentage
- Clear formatted output with voltage drop analysis
- Automatic recommendation of the smallest gauge that meets requirements
- Resistance values based on copper wire at 75°C

## Building

Requirements:
- Rust 1.70 or later
- Cargo

```bash
cargo build --release
```

The compiled binary will be in `target/release/wire-util`

## Usage

### Basic usage

Calculate voltage drop for a 120V circuit with 20A current at 100 feet distance:

```bash
cargo run -- --voltage 120 --current 20 --distance 100
```

### With specific gauges

Show only 10, 12, and 14 AWG gauges:

```bash
cargo run -- --voltage 120 --current 20 --distance 100 --gauges 10,12,14
```

### Custom voltage drop threshold

Set maximum acceptable voltage drop to 5% instead of the default 3%:

```bash
cargo run -- --voltage 240 --current 15 --distance 150 --max-drop 5
```

## Command Line Arguments

| Argument | Short | Type | Description |
|----------|-------|------|-------------|
| `--voltage` | `-v` | float | Voltage in volts (required) |
| `--current` | `-c` | float | Current in amps (required) |
| `--distance` | `-d` | float | One-way distance in feet (required) |
| `--max-drop` | `-m` | float | Maximum acceptable voltage drop percentage (default: 3.0) |
| `--gauges` | | integers | Comma-separated wire gauges to show (e.g., 10,12,14). If omitted, shows all gauges |

## Supported Wire Gauges

28, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2, 1, 0, 00, 000, 0000 AWG

## Examples

### Example 1: 12V automotive circuit

```bash
cargo run -- --voltage 12 --current 20 --distance 10 --gauges 8,10,12,14
```

### Example 2: 240V sub-panel

```bash
cargo run -- --voltage 240 --current 30 --distance 50 --max-drop 2
```

### Example 3: Low voltage lighting

```bash
cargo run -- --voltage 24 --current 10 --distance 100 --gauges 14,12,10,8,6
```

### Example 4: DC 14.5V circuit with filtered gauges

```bash
cargo run -- --voltage 14.5 --current 8 --distance 10 --gauges 8,10,12,14,16,18,22
```

Output:
```
=== Wire Gauge Voltage Drop Calculator ===

Input Parameters:
  Voltage: 14.5 V
  Current: 8 A
  Distance: 10 ft (one way)
  Max Acceptable Drop: 3%
  Filtered Gauges: [8, 10, 12, 14, 16, 18, 22]

+------------+----------------+------------------+----------+-----------------+
| Wire Gauge | Resistance (Ω) | Voltage Drop (V) | Drop (%) | Status          |
+------------+----------------+------------------+----------+-----------------+
| 22 AWG     | 0.3228         | 2.582            | 17.81    | ✗ Too much drop |
+------------+----------------+------------------+----------+-----------------+
| 18 AWG     | 0.1277         | 1.022            | 7.05     | ✗ Too much drop |
+------------+----------------+------------------+----------+-----------------+
| 16 AWG     | 0.0803         | 0.643            | 4.43     | ✗ Too much drop |
+------------+----------------+------------------+----------+-----------------+
| 14 AWG     | 0.0502         | 0.402            | 2.77     | ✓ OK            |
+------------+----------------+------------------+----------+-----------------+
| 12 AWG     | 0.0318         | 0.254            | 1.75     | ✓ OK            |
+------------+----------------+------------------+----------+-----------------+
| 10 AWG     | 0.0200         | 0.160            | 1.10     | ✓ OK            |
+------------+----------------+------------------+----------+-----------------+
| 8 AWG      | 0.0126         | 0.100            | 0.69     | ✓ OK            |
+------------+----------------+------------------+----------+-----------------+

Recommended gauge: 14 AWG
  Voltage drop: 0.402 V (2.77%)
```

## Output

The tool displays:
- Input parameters (voltage, current, distance, max drop)
- A table with wire gauge results showing:
  - Wire gauge
  - Resistance in ohms
  - Voltage drop in volts
  - Voltage drop as a percentage
  - Status (✓ OK or ✗ Too much drop)
- Recommended gauge (smallest gauge that meets your voltage drop requirement)

## License

MIT License - see LICENSE file for details

## Notes

- Distance is specified as one-way; the tool automatically calculates round-trip distance
- Resistance values are for copper wire at 75°C ambient temperature
- Always follow local electrical codes and regulations when designing circuits
