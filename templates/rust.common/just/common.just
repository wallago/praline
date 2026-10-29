# {if:just}
# Run the app with `config.toml`. Usage: just run -- [ARGS]
[group('dev')]
run *ARGS:
    cargo run -- --config config.toml {{ ARGS }}

# Build the optimized release binary
[group('dev')]
build:
    cargo build --release

# Type-check the whole workspace (fast feedback, no binary)
[group('dev')]
check:
    cargo check --all-targets
# {endif:just}
