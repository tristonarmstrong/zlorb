build:
  cargo build --release
  # --------------
  rm /usr/local/bin/zlorbrs-ctl
  mv target/release/zlorbrs-ctl /usr/local/bin
  # --------------
  rm /usr/local/bin/zlorbrs-service
  mv target/release/zlorbrs-service /usr/local/bin
  # --------------
  cp zlorbrs.service /usr/lib/systemd/system/zlorbrs.service
  # --------------
  systemctl daemon-reload
