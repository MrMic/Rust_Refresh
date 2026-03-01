# To load .env file(s)
set dotenv-load
set dotenv-filename := ".env"


# Use bash for better scripting
set shell := ["bash", "-cu"]

# Workspace members (crate names)
memory_management_09 := "memory_management_09"

# Default recipe
default: build

# -----------------------------
# Workspace‑level commands
# -----------------------------

# Build the workspace project
[group('dev')]
build:
    cargo build --workspace

release:
    cargo build --workspace --release

run-all:
    cargo run --workspace

test:
    cargo test --workspace

clean:
    cargo clean

fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

watch:
    cargo watch -x "check --workspace"

# -----------------------------
# App‑specific commands
# -----------------------------

# run the user_input_01 crate
[group('dev')]
run-memory_management:
    cargo run -p {{memory_management_09}}

build-memory_management:
    cargo build -p {{memory_management_09}}

release-memory_management:
    cargo build -p {{memory_management_09}} --release

test-memory_management:
    cargo test -p {{memory_management_09}}


# -----------------------------
# Utility
# -----------------------------

info:
    echo "Workspace crates:"
    echo " - {{memory_management_09}}"
    echo ""
    echo "Rust version:"
    rustc --version
