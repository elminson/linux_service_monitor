#!/bin/sh

# Start services
service nginx start
if [ $? -ne 0 ]; then
  echo "Failed to start nginx"
  exit 1
fi

service mariadb start
if [ $? -ne 0 ]; then
  echo "Failed to start mariadb"
  exit 1
fi

service postgresql start
if [ $? -ne 0 ]; then
  echo "Failed to start postgresql"
  exit 1
fi

# Run the Rust application
cargo run