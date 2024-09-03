# Datto Monitoring CLI

This Rust project is a command-line interface (CLI) application designed to monitor backup histories for various SaaS customers using the Datto API. It checks the backup status of specific applications and sends email notifications when certain criteria are met.

## Features

- **Authentication**: Uses Basic Authentication with the Datto API to retrieve information about SaaS customers.
- **Backup Monitoring**: Filters and monitors backup histories for specific time windows and statuses.
- **Customizable Alerts**: Sends alerts based on criteria defined in a JSON configuration file.
- **Concurrency**: Uses asynchronous tasks to handle multiple SaaS customers simultaneously.

## Prerequisites

Before running the tool, ensure you have the following:

- Rust installed on your system.
- A `.env` file in the current directory with the following environment variables:
  - `CLIENT_ID`: Your Datto API client ID.
  - `CLIENT_SECRET`: Your Datto API client secret.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/datto_tools.git
    cd datto_tools/datto_monitoring_cli/
    ```

2. **Build the Project**:
    Make sure you have Rust installed on your machine. You can build the project with Cargo:
    ```bash
    cargo build --release
    ```

## Configuration

### JSON Configuration

Create a `datto_monitoring.json` file in the root directory of the project. This file should contain a list of companies you want to monitor. The structure of the JSON file should look like this:

```json
{
    "companies": [
        {
            "name": "Company1",
            "sending_email": "alert@company1.com",
            "receiving_email": "admin@company1.com"
        },
        {
            "name": "Company2",
            "sending_email": "alert@company2.com",
            "receiving_email": "admin@company2.com"
        }
    ]
}
```

### .env File Setup

Create a `.env` file in the root directory of the project with the following contents. This file will store your Datto API credentials:

```dotenv
CLIENT_ID=your_client_id_here
CLIENT_SECRET=your_client_secret_here
```

Replace your_client_id_here and your_client_secret_here with your actual Datto API credentials.

## Usage

To run the tool, use the following command:

```sh
./target/release/datto_monitoring_cli
```

The tool will read the configuration from the datto_monitoring.json file and the environment variables from the .env file. It will then authenticate with the Datto API, retrieve backup histories for the specified SaaS customers, and output relevant information to the terminal.


## Output

The tool will read the configuration from the `datto_monitoring.json` file and the environment variables from the `.env` file. It will then authenticate with the Datto API, retrieve backup histories for the specified SaaS customers, and output relevant information to the terminal.

## Output

If a company's backup history has three non-perfect outcomes within specific time windows, the tool will print the following details:

- Customer Name
- Application Type
- Time Window and Status for each relevant backup
- Sending and Receiving Email addresses for the alert

Example output:

```plaintext
Customer Name: COMPANY1
App Type: Office 365
  Time Window: Between0dAnd1d
  Status: Warning
  Time Window: Between1dAnd2d
  Status: Warning
  Time Window: Between2dAnd3d
  Status: Warning
Sending Email: alert@company1.com
Receiving Email: admin@company1.com
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/datto_tools/blob/main/LICENSE) file for details.
