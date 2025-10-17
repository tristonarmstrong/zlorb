build:
  cargo build --release
  cp zlorbrs.service /usr/lib/systemd/system/zlorbrs.service
