# Quake Log Parser in Rust

Quake Log Parser writen in Rust language;

It processes Quake game logs to extract and create a report file with each match information.


## Requirements

- Rust `1.70.0` or higher. 

## Building the Project

Before building the project, ensure you have Rust and Cargo installed on your system.

To build the project in release mode, run the following command:

```sh
cargo build --release
```

## Running the Project

To run the Quake Log Parser, you need a Quake log file as input. A Quake log example is provided at the root of this project (`.quake.log`).

The output file is created at the current directory with the name `quake-report.json` and the following pattern:

```json
[
    {
        "round_number": 19,
        "total_kills": 95,
        "players": [
            "Oootsimo",
            "Dono da Bola",
        ],
        "kills": {
            "Oootsimo": 10,
            "Dono da Bola": 10,
        },
        "kills_by_means": {
            "MOD_ROCKET_SPLASH": 32,
            "MOD_TRIGGER_HURT": 12,
            "MOD_ROCKET": 27
        }
    }
]

```

### Using Cargo

If you have Cargo installed, you can run the project directly from the source code. From the project's root directory, execute:

```sh
cargo run -- <path_to_quake_log>
```

### Tests

Some tests can be executed for the parser using:

```sh
cargo test
```

## Author
Kelvin César de Andrade - kelvincandrade@gmail.com