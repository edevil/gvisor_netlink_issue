# Use the official Rust image as the base image
FROM rust:latest AS build

# Set the working directory inside the container
WORKDIR /usr/src/whatsmyip

# Copy the source code into the container
COPY . .

# Build the application (release mode)
RUN cargo build --release

# Create a new stage for the final image
FROM gcr.io/distroless/cc-debian12:nonroot

# Set the working directory for the final image
WORKDIR /usr/src/whatsmyip

# Copy the statically-linked binary from the build stage
COPY --from=build /usr/src/whatsmyip/target/release/whatsmyip .

# Specify the command to run your Rust application
CMD ["./whatsmyip"]

