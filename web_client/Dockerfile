# Use a base image like Ubuntu
FROM ubuntu:20.04

# Set the non-interactive mode to avoid prompts during apt-get install
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
  tzdata \
  git \
  curl \
  unzip \
  xz-utils \
  zip \
  libglu1-mesa \
  openjdk-8-jdk

# Clean up the apt cache to reduce image size
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Download and install Flutter SDK
RUN git clone https://github.com/flutter/flutter.git -b stable /flutter

# Set environment variables
ENV PATH="$PATH:/flutter/bin"

# Pre-download Flutter dependencies
RUN flutter precache

# Accept Android licenses
RUN yes | flutter doctor --android-licenses

# Set the working directory
WORKDIR /app

# Copy your app files into the container
COPY . .

# Run flutter pub get
RUN flutter pub get

# Expose the appropriate port
EXPOSE 80

# Default command to run the web app
CMD ["flutter", "run", "-d", "web-server", "--web-port", "80", "--web-hostname", "0.0.0.0"]

