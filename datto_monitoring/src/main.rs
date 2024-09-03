use base64::encode;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tokio::task;

const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";

#[derive(Deserialize, Debug)]
struct Domain {
    #[serde(rename = "saasCustomerId")]
    saas_customer_id: Option<u64>,
    #[serde(rename = "saasCustomerName")]
    #[allow(dead_code)]
    saas_customer_name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ApplicationsResponse {
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Item {
    #[serde(rename = "customerName")]
    customer_name: String,
    suites: Vec<Suite>,
}

#[derive(Deserialize, Debug)]
struct Suite {
    #[serde(rename = "appTypes")]
    app_types: Vec<AppType>,
}

#[derive(Deserialize, Debug)]
struct AppType {
    #[serde(rename = "appType")]
    app_type: String,
    #[serde(rename = "backupHistory")]
    backup_history: Vec<BackupHistory>,
}

#[derive(Deserialize, Debug)]
struct BackupHistory {
    #[serde(rename = "timeWindow")]
    time_window: String,
    status: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Company {
    // Added Clone here
    name: String,
    sending_email: String,
    receiving_email: String,
}

#[derive(Deserialize, Debug)]
struct MonitoringConfig {
    companies: Vec<Company>,
}

async fn fetch_applications(
    client: &Client,
    encoded_credentials: &str,
    saas_customer_id: u64,
    companies: &[Company],
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://api.datto.com/v1/saas/{}/applications",
        saas_customer_id
    );
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {}", encoded_credentials))
        .header("Accept", "application/json")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    if status.is_success() {
        let applications_response: ApplicationsResponse = serde_json::from_str(&text)?;

        for item in applications_response.items {
            let customer_name = item.customer_name.to_uppercase(); 
            if let Some(company) = companies
                .iter()
                .find(|company| customer_name.contains(&company.name))
            {
                for suite in item.suites {
                    for app_type in suite.app_types {
                        let mut relevant_history = Vec::new();
                        let mut failure_count = 0;

                        for backup_history in &app_type.backup_history {
                            if ["Between0dAnd1d", "Between1dAnd2d", "Between2dAnd3d"]
                                .contains(&backup_history.time_window.as_str())
                            {
                                relevant_history.push(backup_history);
                                if backup_history.status != "Perfect" {
                                    failure_count += 1;
                                }
                            }
                        }

                        if failure_count == 3 {
                            println!("Customer Name: {}", customer_name);
                            println!("App Type: {}", app_type.app_type);
                            for history in relevant_history {
                                println!("  Time Window: {}", history.time_window);
                                println!("  Status: {}", history.status);
                            }
                            println!("Sending Email: {}", company.sending_email);
                            println!("Receiving Email: {}", company.receiving_email);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!(
            "Error response from server for SaaS Customer ID {}: {}",
            saas_customer_id, text
        );
    }

    Ok(())
}

async fn make_request() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let credentials = format!("{}:{}", CLIENT_ID, CLIENT_SECRET);
    let encoded_credentials = encode(credentials);

    let file = File::open("datto_monitoring.json")?;
    let reader = BufReader::new(file);
    let config: MonitoringConfig = serde_json::from_reader(reader)?;

    println!(
        "Successfully imported JSON file with {} companies",
        config.companies.len()
    );

    let companies: Vec<Company> = config.companies;

    let res = client
        .get("https://api.datto.com/v1/saas/domains")
        .header("Authorization", format!("Basic {}", encoded_credentials))
        .header("Accept", "application/json")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    if status.is_success() {
        let domains: Vec<Domain> = serde_json::from_str(&text)?;
        let mut tasks = vec![];

        for domain in domains {
            if let Some(saas_customer_id) = domain.saas_customer_id {
                let client = client.clone();
                let encoded_credentials = encoded_credentials.to_string();
                let companies = companies.clone();

                let task = task::spawn(async move {
                    fetch_applications(&client, &encoded_credentials, saas_customer_id, &companies)
                        .await
                        .unwrap();
                });

                tasks.push(task);
            } else {
                println!("Incomplete domain information: {:?}", domain);
            }
        }

        for task in tasks {
            task.await?;
        }

        Ok(())
    } else {
        eprintln!("Error response from server: {}", text);
        Err(format!("Error response from server: {}", text).into())
    }
}

#[tokio::main]
async fn main() {
    match make_request().await {
        Ok(_) => println!("Request succeeded"),
        Err(e) => eprintln!("Error making request: {}", e),
    }
}
