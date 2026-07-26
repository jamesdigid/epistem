use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::manifest::validation::ManifestValidator;

#[derive(Debug, Clone)]
struct ValidationRow {
    field: String,
    value: String,
    status: String,
}

#[derive(Debug, Parser)]
#[command(
    name = "epistem",
    about = "Open capability registry and capability manager for autonomous agents",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    Validate {
        path: PathBuf,
    },
    Install,
    /// Learn a capability into the current agent directory.
    Learn {
        capability: String,
    },
    Graph,
    Search,
}

pub fn run() -> crate::error::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            println!("init is not implemented yet");
        }
        Commands::Validate { path } => {
            validate(path);
        }
        Commands::Install => {
            println!("install is not implemented yet");
        }
        Commands::Learn { capability } => {
            println!("learn is not implemented yet (requested capability: {capability})");
        }
        Commands::Graph => {
            println!("graph is not implemented yet");
        }
        Commands::Search => {
            println!("search is not implemented yet");
        }
    }

    Ok(())
}

fn validate(path: PathBuf) {
    let validator = ManifestValidator::default();
    let report = validator.validate_path(&path);
    let mut rows = vec![ValidationRow {
        field: "path".to_string(),
        value: report.path.display().to_string(),
        status: status(report.valid).to_string(),
    }];

    if let Some(manifest) = report.manifest.as_ref() {
        rows.push(ValidationRow {
            field: "name".to_string(),
            value: manifest.name.clone(),
            status: status(report.valid).to_string(),
        });
        rows.push(ValidationRow {
            field: "version".to_string(),
            value: manifest.version.clone(),
            status: status(report.valid).to_string(),
        });
        rows.push(ValidationRow {
            field: "description".to_string(),
            value: manifest.description.as_deref().unwrap_or("").to_string(),
            status: status(report.valid).to_string(),
        });
        rows.push(ValidationRow {
            field: "contracts".to_string(),
            value: contracts_summary(&manifest.contracts),
            status: status(report.valid).to_string(),
        });
        rows.push(ValidationRow {
            field: "dependencies".to_string(),
            value: dependencies_summary(&manifest.dependencies),
            status: status(report.valid).to_string(),
        });
        rows.push(ValidationRow {
            field: "keywords".to_string(),
            value: joined(&manifest.keywords),
            status: status(report.valid).to_string(),
        });
        if let Some(artifacts) = manifest.artifacts.as_ref() {
            rows.push(ValidationRow {
                field: "artifacts".to_string(),
                value: format!(
                    "guide={:?}, examples={:?}, tests={:?}",
                    artifacts.guide, artifacts.examples, artifacts.tests
                ),
                status: status(report.valid).to_string(),
            });
        }
    }

    if !report.issues.is_empty() {
        for issue in &report.issues {
            rows.push(ValidationRow {
                field: format!("issue: {}", issue.field),
                value: issue.message.clone(),
                status: "invalid".to_string(),
            });
        }
    }

    print_validation_rows(&rows);
}

fn joined(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(", ")
    }
}

fn contracts_summary(contracts: &[crate::manifest::models::ContractSpec]) -> String {
    if contracts.is_empty() {
        return "-".to_string();
    }

    contracts
        .iter()
        .map(|contract| format!("{}@{}", contract.id, contract.version))
        .collect::<Vec<_>>()
        .join(", ")
}

fn dependencies_summary(dependencies: &[crate::manifest::models::DependencySpec]) -> String {
    if dependencies.is_empty() {
        return "-".to_string();
    }

    dependencies
        .iter()
        .map(|dependency| dependency.contract.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn status(valid: bool) -> &'static str {
    if valid { "valid" } else { "invalid" }
}

fn print_validation_rows(rows: &[ValidationRow]) {
    let field_width = rows
        .iter()
        .map(|row| row.field.len())
        .chain(["Field".len()])
        .max()
        .unwrap_or(5);
    let value_width = rows
        .iter()
        .map(|row| row.value.len())
        .chain(["Value".len()])
        .max()
        .unwrap_or(5);
    let status_width = rows
        .iter()
        .map(|row| row.status.len())
        .chain(["Status".len()])
        .max()
        .unwrap_or(6);

    let separator = format!(
        "+-{:-<field_width$}-+-{:-<value_width$}-+-{:-<status_width$}-+",
        "",
        "",
        "",
        field_width = field_width,
        value_width = value_width,
        status_width = status_width,
    );

    println!("{separator}");
    println!(
        "| {:<field_width$} | {:<value_width$} | {:<status_width$} |",
        "Field",
        "Value",
        "Status",
        field_width = field_width,
        value_width = value_width,
        status_width = status_width,
    );
    println!("{separator}");
    for row in rows {
        println!(
            "| {:<field_width$} | {:<value_width$} | {:<status_width$} |",
            row.field,
            row.value,
            row.status,
            field_width = field_width,
            value_width = value_width,
            status_width = status_width,
        );
    }
    println!("{separator}");
}
