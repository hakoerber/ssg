FROM scratch

COPY ./server/target/x86_64-unknown-linux-musl/release/server /server

CMD ["/server", "3000"]
