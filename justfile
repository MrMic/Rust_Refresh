# To load .env file(s)
set dotenv-load
set dotenv-filename := ".env"


# Use bash for better scripting
set shell := ["bash", "-cu"]

# Workspace members (crate names)
user_input := "user_input"

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

# run the user_input crate
[group('dev')]
run-user_input:
    cargo run -p {{user_input}}

build-user_input:
    cargo build -p {{user_input}}

release-user_input:
    cargo build -p {{user_input}} --release

test-user_input:
    cargo test -p {{user_input}}


# -----------------------------
# Utility
# -----------------------------

info:
    echo "Workspace crates:"
    echo " - {{user_input}}"
    echo ""
    echo "Rust version:"
    rustc --version
