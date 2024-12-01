FROM rust:1.81

# Install zsh
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.5/zsh-in-docker.sh)" -- -t agnoster

# Make zsh default
RUN chsh -s /bin/zsh

# Install rustfmt
RUN rustup component add rustfmt

# Install bacon
RUN cargo install --locked bacon

# Install clippy
RUN rustup component add clippy

# Install watch
RUN cargo install cargo-watch --locked

# Additional components required to compute LLVM code coverage
RUN rustup component add llvm-tools-preview

RUN cargo install cargo-llvm-cov

# Cargo audit
RUN cargo install cargo-audit

# Cargo expand
RUN cargo install cargo-expand