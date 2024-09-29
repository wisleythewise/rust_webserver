# This is an image specifically good to run rust binaries
FROM messense/rust-musl-cross:x86_64-musl

# Change to the current working directory
WORKDIR /rust/webserver/

# Copy all the code to the docker image
COPY . .

# Now we can build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Check if the path is correct by printing
RUN ls -la /rust/webserver/target/x86_64-unknown-linux-musl/release/

# Set the entrypoint to the compiled webserver binary
ENTRYPOINT ["/rust/webserver/target/x86_64-unknown-linux-musl/release/webserver"]

# Expose the port for the webserver
EXPOSE 7878
