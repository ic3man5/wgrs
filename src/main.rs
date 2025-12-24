use clap::Parser;
use prettytable::{Table, row};

/// Wire gauge voltage drop calculator
/// 
/// Calculates voltage drop across different wire gauges based on voltage, current, and distance
#[derive(Parser, Debug)]
#[command(name = "Wire Util")]
#[command(about = "Calculate voltage drop for common wire gauges", long_about = None)]
struct Args {
    /// Voltage in volts
    #[arg(short, long)]
    voltage: f64,

    /// Current in amps
    #[arg(short, long)]
    current: f64,

    /// One-way distance in feet
    #[arg(short, long)]
    distance: f64,

    /// Maximum acceptable voltage drop percentage (default: 3%)
    #[arg(short = 'm', long, default_value = "3.0")]
    max_drop: f64,

    /// Wire gauges to show (comma-separated integers, e.g., 10,12,14)
    #[arg(long, value_delimiter = ',')]
    gauges: Option<Vec<i32>>,
}

// Wire AWG sizes with their resistances in ohms per 1000 feet at 75°C copper
// Format: (gauge_number, display_name, resistance)
// Note: Multi-zero gauges use negative numbers for internal representation
const WIRE_GAUGES: &[(i32, &str, f64)] = &[
    (28, "28 AWG", 64.90),
    (26, "26 AWG", 40.81),
    (24, "24 AWG", 25.67),
    (22, "22 AWG", 16.14),
    (20, "20 AWG", 10.15),
    (18, "18 AWG", 6.385),
    (16, "16 AWG", 4.016),
    (14, "14 AWG", 2.51),
    (12, "12 AWG", 1.588),
    (10, "10 AWG", 0.999),
    (8, "8 AWG", 0.628),
    (6, "6 AWG", 0.395),
    (4, "4 AWG", 0.248),
    (2, "2 AWG", 0.156),
    (1, "1 AWG", 0.123),
    (0, "0 AWG", 0.0983),
    (-2, "00 AWG", 0.0780),
    (-3, "000 AWG", 0.0619),
    (-4, "0000 AWG", 0.0491),
];

fn main() {
    let args = Args::parse();

    // Validate gauges argument if provided
    if let Some(ref requested_gauges) = args.gauges {
        let valid_gauges: Vec<i32> = WIRE_GAUGES.iter().map(|(num, _, _)| *num).collect();
        for gauge in requested_gauges {
            if !valid_gauges.contains(gauge) {
                eprintln!("Error: Invalid gauge number: {}. Valid gauges are: {:?}", gauge, 
                    valid_gauges.iter()
                        .filter(|&&g| g > 0)
                        .collect::<Vec<_>>());
                std::process::exit(1);
            }
        }
    }

    // Total distance (round trip)
    let total_distance = args.distance * 2.0;

    // Create results table
    let mut table = Table::new();
    table.add_row(row!["Wire Gauge", "Resistance (Ω)", "Voltage Drop (V)", "Drop (%)", "Status"]);

    let mut recommended = None;

    for (gauge_num, display_name, resistance_per_1000) in WIRE_GAUGES {
        // Skip if gauges filter is applied and this gauge is not in the list
        if let Some(ref requested_gauges) = args.gauges {
            if !requested_gauges.contains(gauge_num) {
                continue;
            }
        }

        // Calculate total resistance for the wire run
        let total_resistance = (resistance_per_1000 * total_distance) / 1000.0;

        // Calculate voltage drop using Ohm's law: V = I * R
        let voltage_drop = args.current * total_resistance;

        // Calculate percentage drop
        let drop_percentage = (voltage_drop / args.voltage) * 100.0;

        // Determine status
        let status = if drop_percentage <= args.max_drop {
            if recommended.is_none() {
                recommended = Some((*display_name, voltage_drop, drop_percentage));
            }
            "✓ OK"
        } else {
            "✗ Too much drop"
        };

        table.add_row(row![
            display_name,
            format!("{:.4}", total_resistance),
            format!("{:.3}", voltage_drop),
            format!("{:.2}", drop_percentage),
            status
        ]);
    }

    println!("\n=== Wire Gauge Voltage Drop Calculator ===\n");
    println!("Input Parameters:");
    println!("  Voltage: {} V", args.voltage);
    println!("  Current: {} A", args.current);
    println!("  Distance: {} ft (one way)", args.distance);
    println!("  Max Acceptable Drop: {}%", args.max_drop);
    if let Some(ref gauges) = args.gauges {
        println!("  Filtered Gauges: {:?}", gauges);
    }
    println!();

    table.printstd();

    println!();
    if let Some((gauge, drop, percentage)) = recommended {
        println!("Recommended gauge: {}", gauge);
        println!("  Voltage drop: {:.3} V ({:.2}%)", drop, percentage);
    } else {
        println!("WARNING: Even the largest gauge exceeds acceptable voltage drop!");
    }
}
