use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::manifest::validation::ManifestValidator;
use crate::utils::paths::MANIFEST_FILENAME;

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
    /// Scaffold a new capability directory.
    Init {
        /// Directory to initialize. Defaults to the parent of the current working directory.
        directory: Option<PathBuf>,
    },
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
        Commands::Init { directory } => {
            init(directory)?;
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

fn init(target_dir: Option<PathBuf>) -> crate::error::Result<()> {
    let cwd = env::current_dir()?;
    let target_dir = match target_dir {
        Some(directory) => directory,
        None => cwd
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| cwd.clone()),
    };

    let capabilities_dir = target_dir.join("capabilities");
    fs::create_dir_all(&capabilities_dir)?;

    let workspace_name = target_dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("epistem-workspace");

    let manifest_path = target_dir.join(MANIFEST_FILENAME);
    let workspace_doc_path = target_dir.join("EPISTEM.md");

    let manifest = format!(
        concat!(
            "{{\n",
            "  \"name\": \"{}\",\n",
            "  \"version\": \"0.1.0\",\n",
            "  \"description\": \"Capability scaffold initialized by epistem init\"\n",
            "}}\n"
        ),
        workspace_name
    );

    let workspace_doc = format!(
        concat!(
            "# Epistem Workspace\n\n",
            "This directory was initialized by `epistem init`.\n\n",
            "Installed capabilities live under `capabilities/`.\n\n",
            "## Next Steps\n\n",
            "- Edit `{}` to describe the workspace-level capability.\n",
            "- Add installed capabilities under `capabilities/`.\n"
        ),
        MANIFEST_FILENAME
    );

    write_if_missing(&manifest_path, &manifest)?;
    write_if_missing(&workspace_doc_path, &workspace_doc)?;

    println!(
        "initialized capability scaffold in {}",
        target_dir.display()
    );
    Ok(())
}

fn write_if_missing(path: &Path, contents: &str) -> crate::error::Result<()> {
    if path.exists() {
        return Ok(());
    }

    fs::write(path, contents)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    struct CwdGuard {
        previous: PathBuf,
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.previous);
        }
    }

    #[test]
    fn init_scaffolds_an_explicit_directory() {
        let tempdir = tempdir().expect("temp dir");

        init(Some(tempdir.path().to_path_buf())).expect("init should succeed");

        assert_scaffold(tempdir.path());
    }

    #[test]
    fn init_defaults_to_the_parent_directory() {
        let tempdir = tempdir().expect("temp dir");
        let nested_dir = tempdir.path().join("agent");
        fs::create_dir_all(&nested_dir).expect("nested dir");

        let previous = env::current_dir().expect("current dir");
        let _guard = CwdGuard { previous };
        env::set_current_dir(&nested_dir).expect("switch cwd");

        init(None).expect("init should succeed");

        assert_scaffold(tempdir.path());
    }

    fn assert_scaffold(target_dir: &Path) {
        let manifest_path = target_dir.join(MANIFEST_FILENAME);
        let workspace_doc_path = target_dir.join("EPISTEM.md");
        let capability_dir = target_dir.join("capabilities");

        assert!(manifest_path.exists());
        assert!(workspace_doc_path.exists());
        assert!(capability_dir.exists());
        assert!(
            fs::read_dir(&capability_dir)
                .expect("capabilities dir")
                .next()
                .is_none()
        );

        let manifest = fs::read_to_string(manifest_path).expect("manifest");
        assert!(manifest.contains("\"version\": \"0.1.0\""));

        let workspace_doc = fs::read_to_string(workspace_doc_path).expect("workspace doc");
        assert!(workspace_doc.contains("Installed capabilities live under `capabilities/`"));
    }
}
