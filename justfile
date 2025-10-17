build:
  cargo build --release
  mv target/release/zlorbrs-ctl /usr/local/bin
  mv target/release/zlorbrs-service /usr/local/bin
  cp zlorbrs.service /usr/lib/systemd/system/zlorbrs.service
