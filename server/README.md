# Collaborative Text Editor

This is a collaborative text editor application that allows multiple users to edit the same document in real-time. The application consists of:

- **Server**: A Rust-based WebSocket server that manages document state and synchronizes edits between clients.
- **Web Client**: A Flutter web application that connects to the server and provides a user interface for editing the document.
- **Rust Client**: An alternative client written in Rust for command-line interaction with the document.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Directory Structure](#directory-structure)
- [Setup and Run with Docker Compose](#setup-and-run-with-docker-compose)
  - [1. Clone the Repository](#1-clone-the-repository)
  - [2. Build and Run the Services](#2-build-and-run-the-services)
- [Using the Application](#using-the-application)
  - [Accessing the Web Client](#accessing-the-web-client)
  - [Interacting with the Rust Client](#interacting-with-the-rust-client)
- [Services Details](#services-details)
  - [Server](#server)
  - [Web Client](#web-client)
  - [Rust Client](#rust-client)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **Docker Compose**: [Install Docker Compose](https://docs.docker.com/compose/install/)

## Directory Structure

The repository is organized as follows:

```plaintext
.
├── docker-compose.yml
├── README.md
├── server
│   ├── Dockerfile
│   └── src
│       └── main.rs
├── web_client
│   ├── Dockerfile
│   └── lib
│       └── main.dart
└── client
    ├── Dockerfile
    └── src
        └── main.rs
```

## Setup and Run with Docker Compose

### Build and Run the Services

```
docker-compose up --build
```

This command will:

Build the Docker images for the server, web client, and Rust client.
Start the services and connect them via Docker Compose.
Note: The first build may take several minutes to complete.

## Using the Application

### Accessing the Web Client

- Open your web browser.
- Navigate to `http://localhost`.
- You should see the collaborative text editor interface. You can open this URL in multiple browser windows or on different devices connected to the same network to test real-time collaboration.

### Interacting with the Rust Client

The Rust client runs as a service in the Docker Compose setup. To interact with it:

- Open a terminal.

- Attach to the Rust client container:

```
docker-compose exec client bash
```

- Once inside the container, you can interact with the client via the command line.

Example Usage:

- To enter an edit:

```
Enter an edit (position,insert/delete):
```

- Follow the prompt to input your edits in the format:

```
For insertion: position,text_to_insert
For deletion: position,deleteN (where N is the number of characters to delete)
```

Note: The Rust client logs will display information about edits sent and received.

## Services Details

### Server
Description: A Rust-based WebSocket server that manages the collaborative editing session.
Dockerfile Location: `./server/Dockerfile`
Port Mapping: Exposes port `8080` to the host.
Build Context: `./server`

### Web Client
Description: A Flutter web application that connects to the server and provides a user interface.
Dockerfile Location: `./web_client/Dockerfile`
Port Mapping: Exposes port `80` to the host.
Build Context: `./web_client`
Access URL: `http://localhost`

### Rust Client

Description: A command-line client written in Rust for interacting with the server.
Dockerfile Location: `./client/Dockerfile`
Build Context: `./client`
Interaction: Via terminal after attaching to the container.

## Troubleshooting

Docker Permission Issues: If you encounter permission issues during the build, ensure you are not running Docker commands as root and that your user has the appropriate permissions.

Port Conflicts: Ensure that ports 80 and 8080 are not in use by other applications on your host machine.

WebSocket Connection Errors: If the web client cannot connect to the server, verify that the server is running and accessible at `ws://localhost:8080`.

Rebuilding Containers: If you make changes to the code, rebuild the Docker images:

```bash
docker-compose build
docker-compose up
```
