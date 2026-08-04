# builder
from rust:1-alpine as builder
run apk add --no-cache musl-dev
workdir /dolan
copy ./ .
run cargo build --locked --release

# final
from alpine
run apk add --no-cache ca-certificates
copy --from=builder /dolan/target/release/dolan /usr/local/bin/dolan
expose 10000
cmd ["dolan"]
