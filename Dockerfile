FROM debian:latest

# Install necessary tools and dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    nginx \
    mariadb-server \
    postgresql

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/myapp
COPY . .

# Build the Rust application
RUN cargo build --release

# Copy the start_services.sh script
COPY start_services.sh /usr/src/myapp/start_services.sh
RUN chmod 777 /usr/src/myapp/start_services.sh

# Start services and run the Rust application
CMD ["/usr/src/myapp/start_services.sh"]