#!/bin/bash

set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if Docker is installed and install it if not
if ! command_exists docker; then
    echo "Docker is not installed. Installing Docker..."
    sudo apt-get update
    sudo apt-get install -y apt-transport-https ca-certificates curl software-properties-common
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
    sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
    sudo apt-get update
    sudo apt-get install -y docker-ce
    sudo usermod -aG docker $USER
    newgrp docker
    echo "Docker installation completed."
else
    echo "Docker is already installed."
fi

# Check if Docker Compose is installed and install it if not
if ! command_exists docker-compose; then
    echo "Docker Compose is not installed. Installing Docker Compose..."
    sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
    echo "Docker Compose installation completed."
else
    echo "Docker Compose is already installed."
fi

echo "Starting Docker Compose..."
docker-compose up -d

echo "Building the Rust project in release mode..."
cargo build --release

echo "Running the Rust project..."
./target/release/$(basename $(pwd))
