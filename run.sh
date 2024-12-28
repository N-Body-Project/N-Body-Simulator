#!/bin/bash

# Define color codes
RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
RESET="\e[0m"

# Check if rustfmt is installed
echo -e "${YELLOW}||Checking if rustfmt is installed...${RESET}"
if ! command -v rustfmt &> /dev/null; then
    echo -e "${RED}||rustfmt is not installed. Please run 'rustup component add rustfmt' to install it.${RESET}"
    exit 1
else
    echo -e "${GREEN}||rustfmt is installed.${RESET}"
fi

# Format Rust code
echo -e "${YELLOW}||Formatting Rust code...${RESET}"
cargo fmt
if [ $? -eq 0 ]; then
    echo -e "${GREEN}||Formatting successful.${RESET}"
else
    echo -e "${RED}||Formatting failed.${RESET}"
    exit 1
fi

# Clippy
# MEMO: Unknown behavior, adjust as we go.
echo -e "${YELLOW}||Running clippy...${RESET}"
cargo clippy

# Build the project
echo -e "${YELLOW}||Building debug build...${RESET}"
cargo build
if [ $? -eq 0 ]; then
    echo -e "${GREEN}||Build successful. Running the project...${RESET}"
    ./target/debug/n-body-simulator
else
    echo -e "${RED}||Build failed.${RESET}"
    exit 1
fi
