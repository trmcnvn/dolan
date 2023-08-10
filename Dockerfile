# builder
from rustlang/rust:nightly as builder
run rustup target add x86_64-unknown-linux-musl
run apt update && apt install -y musl-tools musl-dev
workdir /dolan
copy ./ .
run cargo build --target x86_64-unknown-linux-musl --release
# final
from scratch
copy --from=builder /dolan/target/x86_64-unknown-linux-musl/release/dolan ./
expose 10000
cmd ["./dolan"]
