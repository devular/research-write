# Use an official Rust image based on Debian (adjust version as needed)
FROM rust:1.84-bookworm

# Enable multiarch to install x86_64 libraries
RUN dpkg --add-architecture amd64 && apt-get update

# Optional: Disable HTTP pipelining to help avoid hash sum mismatches
RUN echo 'Acquire::http::Pipeline-Depth "0";' > /etc/apt/apt.conf.d/99disable-pipeline

# Install x86_64 dependencies for cross-compiling (note the :amd64 suffix)
RUN apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  apt-get update --fix-missing && \
  apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev:amd64 \
  libgtk-3-dev:amd64 \
  libglib2.0-dev:amd64 \
  libglib2.0-0:amd64 \
  libwebkit2gtk-4.1-dev:amd64 \
  python3 \
  make && \
  apt-get install -y build-essential:amd64 && \
  apt-get install -y python3-mako python3-markdown && \
  rm -rf /var/lib/apt/lists/*

# Configure pkg-config for cross-compilation
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-linux-gnu
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
  apt-get update && \
  apt-get install -y nodejs

# Add the Linux target for cross-compilation
RUN rustup target add x86_64-unknown-linux-gnu

# Set the working directory in the container
WORKDIR /app

# Copy your project files into the container
COPY . .

# Install JavaScript dependencies (uses your package.json)
RUN npm install

# Run the Tauri build command – this builds the x86_64 Linux artifact
RUN npm run tauri build -- --target x86_64-unknown-linux-gnu