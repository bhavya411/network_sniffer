# Network Sniffer API

## Description

The Network Sniffer API is a Rust application built using the Actix Web framework. It connects to a PostgreSQL database and fetches packet metadata captured by the network sniffer. The captured metadata is then provided to clients in JSON format through the API.

## Requirements

- Rust (Nightly)
- PostgreSQL database with the following credentials:

  - Database URL: `postgres://postgres:1234567@localhost:5432/demo`

## Installation

1. Clone the repository to your local machine:

git clone https://github.com/yourusername/network-sniffer-api.git

cd network-sniffer-api


2. Install the required dependencies:

cargo build

3. Set up the PostgreSQL database with the specified credentials.

## Usage

1. Start the Network Sniffer API:

cargo run


2. The API will start listening on `http://127.0.0.1:2000`.

## Endpoints

### GET /json

- Description: Fetches packet metadata from the database and returns it as JSON.
- Response: Array of packet metadata objects in JSON format.

### GET /

- Description: Serves a static HTML file (tarun.html) with a web-based interface to view the captured packet metadata.

## Database Schema

The database table "sniffer" should have the following schema:

| Column Name  | Data Type       |
|--------------|-----------------|
| src_ip       | VARCHAR (NULL)  |
| dst_ip       | VARCHAR (NULL)  |
| src_port     | INTEGER         |
| dst_port     | INTEGER         |
| seq_number   | INTEGER         |
| ack_number   | INTEGER         |
| flags        | INTEGER         |
| window_size  | INTEGER         |

## Acknowledgments

Special thanks to the Actix Web community and the Tokio contributors for their excellent work in developing the Actix Web framework and the Tokio runtime, making it possible to build high-performance web applications with Rust.

## Contributing

Contributions to the Network Sniffer API project are welcome. If you find a bug, have a feature request, or want to contribute code, please open an issue or submit a pull request on the GitHub repository.



