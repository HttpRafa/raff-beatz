# Use the official ArchLinux image with a AUR helper installed
FROM ghcr.io/greyltc-org/archlinux-aur:paru

RUN pacman-key --init
RUN pacman -Syu --noconfirm

# Install required packages
RUN pacman -S base-devel git openssh nano rustup --noconfirm

# Install Rustup and set default toolchains
RUN rustup default stable

# Install the wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Add $HOME/.cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-generate
RUN cargo binstall cargo-generate trunk wasm-bindgen-cli