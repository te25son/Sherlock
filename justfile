[private]
alias f := fix
# Run linter
fix:
    cargo +nightly fmt --verbose

[private]
alias r := run
# run command with args
run +args="--help":
    cargo run -- {{ args }}
