use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeRef {
    pub file: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScenarioRef {
    pub name: String,
    #[serde(default)]
    pub kind: Option<ScenarioKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScenarioKind {
    Formal,
    Test,
    Manual,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyRef {
    pub file: String,
    #[serde(default)]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Requirement {
    pub id: String,
    pub capability: String,
    pub requirement: String,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub property_tested: bool,
    #[serde(default)]
    pub property: Option<PropertyRef>,
    #[serde(default)]
    pub code: Option<CodeRef>,
    #[serde(default)]
    pub contract: Option<String>,
    #[serde(default)]
    pub claimcheck: Option<bool>,
    #[serde(default)]
    pub scenarios: Option<Vec<ScenarioRef>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mapping {
    pub version: u32,
    pub requirements: Vec<Requirement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}

impl IssueSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueSeverity::Critical => "CRITICAL",
            IssueSeverity::Warning => "WARNING",
            IssueSeverity::Info => "INFO",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhaseStatus {
    Pass,
    Fail,
    Skip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbtRunMeta {
    pub seed: u64,
    pub examples_run: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyIssue {
    pub severity: IssueSeverity,
    pub phase: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_id: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterexample: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbt: Option<PbtRunMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationSummary {
    pub total_requirements: usize,
    pub proved_requirements: usize,
    pub property_tested_requirements: usize,
    pub spec_tracked_requirements: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReport {
    pub ok: bool,
    pub strict: bool,
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<String>,
    pub phases: std::collections::BTreeMap<String, PhaseStatus>,
    pub issues: Vec<VerifyIssue>,
    pub verified_files: Vec<String>,
    pub verification_summary: VerificationSummary,
}

pub fn summarize_verification(requirements: &[Requirement]) -> VerificationSummary {
    let proved_requirements = requirements.iter().filter(|r| r.verified).count();
    let property_tested_requirements = requirements.iter().filter(|r| r.property_tested).count();
    let spec_tracked_requirements = requirements
        .iter()
        .filter(|r| !r.verified && !r.property_tested)
        .count();
    VerificationSummary {
        total_requirements: requirements.len(),
        proved_requirements,
        property_tested_requirements,
        spec_tracked_requirements,
    }
}

pub fn validate_mapping(mapping: &Mapping) -> Result<(), String> {
    if mapping.version != 1 {
        return Err(format!("mapping version must be 1, got {}", mapping.version));
    }
    for req in &mapping.requirements {
        if req.id.is_empty() {
            return Err("requirement id must not be empty".to_string());
        }
        if req.capability.is_empty() {
            return Err("requirement capability must not be empty".to_string());
        }
        if req.requirement.is_empty() {
            return Err("requirement text must not be empty".to_string());
        }
        if req.verified && req.property_tested {
            return Err(format!(
                "requirement '{}' cannot set both verified and property_tested (v1: mutually exclusive)",
                req.id
            ));
        }
        if req.property_tested {
            if let Some(prop) = &req.property {
                if prop.file.is_empty() {
                    return Err(format!(
                        "requirement '{}' property.file must not be empty",
                        req.id
                    ));
                }
            }
        }
        if let Some(code) = &req.code {
            if code.file.is_empty() {
                return Err("code.file must not be empty".to_string());
            }
            if code.symbols.is_empty() {
                return Err("code.symbols must not be empty".to_string());
            }
        }
    }
    Ok(())
}
