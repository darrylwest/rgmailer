
project := "rgmailer"

alias t := test
alias b := build
alias rel := release
alias r := run

# run the standard tests
test:
    clear
    tar xzf test-setup.tgz
    cargo test

# build the debug target
build:
    clear
    cargo build

# build the docs
docs:
    cargo doc --no-deps --open

# send a simple email
run:
    cargo run --bin rgmailer home/queue/7mNdj105Ch0c.toml

# build the release
release:
    clear
    cargo build --release --bins

# install
install:
    just release
    cp target/release/rgmailer ~/.cargo/bin/

# format the code
format:
    cargo fmt

# run clippy
clippy:
    cargo clippy

# watch the current folders and run tests when a file is changed
watch:
    watchexec -d 500 -c -e rs cargo test && cargo fmt && cargo clippy

# pre-commit
pre:
    just test format clippy

# cover - runs code test coverage report and writes to coverage folder
cover:
    tar xzf test-setup.tgz
    cargo tarpaulin --out html --output-dir coverage

# start a http server in the coverage folder
serve-cover:
    cd coverage && mv tarpaulin-report.html index.html && python3 -m http.server 8080

# merge the develop branch to main
merge:
    git push && git checkout main && git pull && git merge develop && git push && git checkout develop

