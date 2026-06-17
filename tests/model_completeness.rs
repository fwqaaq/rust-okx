//! Source-level regression test for request/response model completeness.

use std::fs;
use std::path::Path;

fn visit_rust_files(path: &Path, findings: &mut Vec<String>) {
    for entry in fs::read_dir(path).expect("read source directory") {
        let entry = entry.expect("read directory entry");
        let path = entry.path();
        if path.is_dir() {
            visit_rust_files(&path, findings);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let source = fs::read_to_string(&path).expect("read Rust source");
            for (line_index, line) in source.lines().enumerate() {
                if line
                    .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                    .any(|token| token == "RequestParams" || token == "RestRow")
                {
                    findings.push(format!(
                        "{}:{}: {}",
                        path.display(),
                        line_index + 1,
                        line.trim()
                    ));
                }
            }
        }
    }
}

#[test]
fn api_modules_do_not_fall_back_to_generic_maps() {
    let mut findings = Vec::new();
    visit_rust_files(Path::new("src/api"), &mut findings);
    assert!(
        findings.is_empty(),
        "replace generic request/response maps with endpoint-specific models:\n{}",
        findings.join("\n")
    );
}

fn request_struct_names(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let rest = line.strip_prefix("pub struct ")?;
            let name = rest
                .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                .next()?;
            name.ends_with("Request").then_some(name)
        })
        .collect()
}

#[test]
fn migrated_request_structs_implement_validation() {
    let request_dirs = [
        "src/api/account/requests",
        "src/api/finance/requests",
        "src/api/public_data/requests",
        "src/api/trade/requests",
    ];
    let mut missing = Vec::new();

    for directory in request_dirs {
        for entry in fs::read_dir(directory).expect("read migrated request directory") {
            let path = entry.expect("read request source entry").path();
            if path.extension().is_none_or(|extension| extension != "rs") {
                continue;
            }
            let source = fs::read_to_string(&path).expect("read migrated request source");
            for name in request_struct_names(&source) {
                let implementation = format!("impl ValidateRequest for {name}");
                if !source.contains(&implementation) {
                    missing.push(format!("{}: {name}", path.display()));
                }
            }
        }
    }

    assert!(
        missing.is_empty(),
        "every migrated public request struct must implement ValidateRequest:\n{}",
        missing.join("\n")
    );
}

#[test]
fn migrated_accessors_validate_before_transport() {
    let request_dirs = [
        "src/api/account/requests",
        "src/api/finance/requests",
        "src/api/public_data/requests",
        "src/api/trade/requests",
    ];
    let mut request_names = Vec::new();
    for directory in request_dirs {
        for entry in fs::read_dir(directory).expect("read migrated request directory") {
            let path = entry.expect("read request source entry").path();
            if path.extension().is_none_or(|extension| extension != "rs") {
                continue;
            }
            let source = fs::read_to_string(path).expect("read migrated request source");
            request_names.extend(request_struct_names(&source).into_iter().map(str::to_owned));
        }
    }

    let api_files = [
        "src/api/account/api.rs",
        "src/api/finance/api.rs",
        "src/api/public_data/api.rs",
        "src/api/trade/api.rs",
    ];
    let mut missing = Vec::new();
    for api_file in api_files {
        let source = fs::read_to_string(api_file).expect("read accessor source");
        for function in source.split("\n    pub async fn ").skip(1) {
            let name = function.split('(').next().unwrap_or("<unknown>").trim();
            let header = function.split('{').next().unwrap_or(function);
            let represented = request_names
                .iter()
                .any(|request_name| header.contains(request_name));
            if represented && !function.contains("request.validate()?") {
                missing.push(format!("{api_file}: {name}"));
            }
        }
    }

    assert!(
        missing.is_empty(),
        "migrated accessors must validate requests before transport:\n{}",
        missing.join("\n")
    );
}
