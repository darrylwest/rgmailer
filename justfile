
project := "rgmailer"

alias t := test
alias ta := test-all
alias b := build
alias rel := release
alias r := run

# run the standard tests
test:
    clear
    cargo test

# run the standard tests + clippy and fmt
test-all:
    clear
    cargo test && cargo fmt && cargo clippy

# build the debug target
build:
    clear
    cargo build

# build the docs
docs:
    cargo doc --no-deps --open

# run the key-service back-end; start the db if necessary
run:
    cargo run --bin rgmailer

# build the release
release:
    clear
    cargo build --release --bins

# install
install:
    just release
    cp target/release/rgmailer ~/.cargo/bin/

# watch the current folders and run tests when a file is changed
watch:
    watchexec -d 500 -c -e rs cargo test && cargo fmt && cargo clippy

# cover - runs code test coverage report and writes to coverage folder
cover:
  cargo tarpaulin --out html --output-dir coverage && scp coverage/tarpaulin-report.html dpw@raincitysoftware.com:raincitysoftware.com/rxkv/index.html

# start a http server in the coverage folder
serve-cover:
  cd coverage && mv tarpaulin-report.html index.html && python3 -m http.server 8080

# merge the develop branch to main
merge:
    git push && git checkout main && git pull && git merge develop && git push && git checkout develop

