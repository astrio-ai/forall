use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use rmcp::ErrorData as McpError;
use rmcp::ServiceExt;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, JsonObject, ListToolsResult, PaginatedRequestParams,
    ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{RequestContext, RoleServer};
use schemars::{JsonSchema, schema_for};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use forall_authoring::authoring::{
    self, CanonicalRoot, ContractTarget, MutationMode, MutationOptions,
};
use forall_authoring::mapping::schema::{CodeRef, Requirement};

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApplyMode {
    Preview,
    Apply,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyRequest {}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InitRequest {
    pub mode: ApplyMode,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DiscoverRequest {
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub languages: Vec<LanguageInput>,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LanguageInput {
    TypeScript,
    Rust,
    Java,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CodeRefInput {
    pub file: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct RequirementInput {
    pub id: String,
    pub capability: String,
    pub requirement: String,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub property_tested: bool,
    pub code: Option<CodeRefInput>,
    pub contract: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpsertRequirementsRequest {
    pub mode: ApplyMode,
    pub expected_mapping_sha256: Option<String>,
    pub requirements: Vec<RequirementInput>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ContractInput {
    pub file: String,
    pub symbol: String,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub ensures: Vec<String>,
    pub expected_sha256: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScaffoldContractsRequest {
    pub mode: ApplyMode,
    pub contracts: Vec<ContractInput>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScaffoldPropertyRequest {
    pub mode: ApplyMode,
    pub requirement_id: String,
    pub body: String,
    pub symbol: Option<String>,
    pub expected_sha256: Option<String>,
    pub expected_mapping_sha256: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct StructuredOutput {
    ok: bool,
    result: serde_json::Value,
}

#[derive(Clone)]
pub struct AuthorMcpServer {
    root: Arc<CanonicalRoot>,
    tools: Arc<Vec<Tool>>,
}

impl AuthorMcpServer {
    pub fn new(root: &Path) -> anyhow::Result<Self> {
        let root =
            CanonicalRoot::new(root).map_err(|error| anyhow::anyhow!("{}", error.message))?;
        Ok(Self {
            root: Arc::new(root),
            tools: Arc::new(vec![
                tool::<EmptyRequest>("forall_author_status", "Inspect the local Forall project.")?,
                tool::<InitRequest>(
                    "forall_author_init",
                    "Preview or initialize the local Forall project layout.",
                )?,
                tool::<DiscoverRequest>(
                    "forall_author_discover",
                    "Discover candidate TypeScript, Rust, and Java symbols without writing files.",
                )?,
                tool::<UpsertRequirementsRequest>(
                    "forall_author_upsert_requirements",
                    "Preview or merge explicit requirements into the project mapping.",
                )?,
                tool::<ScaffoldContractsRequest>(
                    "forall_author_scaffold_contracts",
                    "Preview or insert caller-supplied Forall contracts at mapped symbols.",
                )?,
                tool::<EmptyRequest>(
                    "forall_author_validate",
                    "Validate local mapping, paths, and mapped symbols without running verification.",
                )?,
                tool::<ScaffoldPropertyRequest>(
                    "forall_author_scaffold_property",
                    "Preview or create a caller-supplied property test and mapping link.",
                )?,
            ]),
        })
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    fn success<T: Serialize>(&self, result: T) -> Result<CallToolResult, McpError> {
        let result = serde_json::to_value(result)
            .map_err(|error| McpError::internal_error(error.to_string(), None))?;
        Ok(CallToolResult::structured(serde_json::json!({
            "ok": true,
            "result": result,
        })))
    }

    fn failure(&self, error: authoring::AuthoringError) -> CallToolResult {
        CallToolResult::structured_error(serde_json::json!({
            "code": error.code,
            "message": error.message,
            "path": error.path,
        }))
    }

    fn mutation(
        &self,
        mode: ApplyMode,
        expected_sha256: BTreeMap<String, String>,
    ) -> MutationOptions {
        MutationOptions {
            mode: match mode {
                ApplyMode::Preview => MutationMode::Preview,
                ApplyMode::Apply => MutationMode::Apply,
            },
            expected_sha256,
        }
    }
}

impl ServerHandler for AuthorMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        let tools = self.tools.clone();
        async move {
            Ok(ListToolsResult {
                tools: (*tools).clone(),
                next_cursor: None,
                meta: None,
            })
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "forall_author_status" => {
                let _: EmptyRequest = parse_arguments(request)?;
                match authoring::project_status(&self.root) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_init" => {
                let input: InitRequest = parse_arguments(request)?;
                let mutation = self.mutation(input.mode, BTreeMap::new());
                match authoring::init_project(&self.root, &mutation) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_discover" => {
                let input: DiscoverRequest = parse_arguments(request)?;
                let result = if input.files.is_empty() {
                    let languages = input
                        .languages
                        .into_iter()
                        .map(|language| match language {
                            LanguageInput::TypeScript => authoring::SourceLanguage::TypeScript,
                            LanguageInput::Rust => authoring::SourceLanguage::Rust,
                            LanguageInput::Java => authoring::SourceLanguage::Java,
                        })
                        .collect::<Vec<_>>();
                    authoring::discover_project_symbols(&self.root, &languages)
                } else {
                    authoring::discover_symbols(&self.root, &input.files)
                };
                match result {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_upsert_requirements" => {
                let input: UpsertRequirementsRequest = parse_arguments(request)?;
                let mut expected = BTreeMap::new();
                if let Some(hash) = input.expected_mapping_sha256 {
                    expected.insert(".forall/verify/mapping.yaml".to_string(), hash);
                }
                let requirements = input
                    .requirements
                    .into_iter()
                    .map(requirement_from_input)
                    .collect();
                let operation = authoring::UpsertRequirementsRequest {
                    requirements,
                    mutation: self.mutation(input.mode, expected),
                };
                match authoring::upsert_requirements(&self.root, &operation) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_scaffold_contracts" => {
                let input: ScaffoldContractsRequest = parse_arguments(request)?;
                let mut expected = BTreeMap::new();
                let contracts = input
                    .contracts
                    .into_iter()
                    .map(|contract| {
                        if let Some(hash) = contract.expected_sha256 {
                            expected.insert(contract.file.clone(), hash);
                        }
                        ContractTarget {
                            file: contract.file,
                            symbol: contract.symbol,
                            requires: contract.requires,
                            ensures: contract.ensures,
                        }
                    })
                    .collect();
                let operation = authoring::ScaffoldContractsRequest {
                    contracts,
                    mutation: self.mutation(input.mode, expected),
                };
                match authoring::scaffold_contracts(&self.root, &operation) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_validate" => {
                let _: EmptyRequest = parse_arguments(request)?;
                match authoring::validate_authoring(&self.root) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            "forall_author_scaffold_property" => {
                let input: ScaffoldPropertyRequest = parse_arguments(request)?;
                let mut expected = BTreeMap::new();
                if let Some(hash) = input.expected_sha256 {
                    expected.insert(
                        format!(".forall/scenarios/{}.property.ts", input.requirement_id),
                        hash,
                    );
                }
                if let Some(hash) = input.expected_mapping_sha256 {
                    expected.insert(".forall/verify/mapping.yaml".to_string(), hash);
                }
                let operation = authoring::ScaffoldPropertyRequest {
                    requirement_id: input.requirement_id,
                    body: input.body,
                    symbol: input.symbol,
                    mutation: self.mutation(input.mode, expected),
                };
                match authoring::scaffold_property(&self.root, &operation) {
                    Ok(result) => self.success(result),
                    Err(error) => Ok(self.failure(error)),
                }
            }
            other => Err(McpError::invalid_params(
                format!("unknown tool: {other}"),
                None,
            )),
        }
    }
}

fn requirement_from_input(input: RequirementInput) -> Requirement {
    Requirement {
        id: input.id,
        capability: input.capability,
        requirement: input.requirement,
        verified: input.verified,
        property_tested: input.property_tested,
        property: None,
        code: input.code.map(|code| CodeRef {
            file: code.file,
            symbols: code.symbols,
        }),
        contract: input.contract,
        claimcheck: None,
        scenarios: None,
    }
}

pub async fn run_stdio(root: &Path) -> anyhow::Result<()> {
    let server = AuthorMcpServer::new(root)?;
    eprintln!(
        "Forall local authoring MCP bound to {}",
        server.root().display()
    );
    let running = server
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?;
    running.waiting().await?;
    Ok(())
}

fn parse_arguments<T: DeserializeOwned>(request: CallToolRequestParams) -> Result<T, McpError> {
    let arguments = request.arguments.unwrap_or_default();
    serde_json::from_value(serde_json::Value::Object(arguments.into_iter().collect()))
        .map_err(|error| McpError::invalid_params(error.to_string(), None))
}

fn tool<I: JsonSchema>(name: &'static str, description: &'static str) -> anyhow::Result<Tool> {
    let input_schema = schema_object::<I>()?;
    let output_schema = schema_object::<StructuredOutput>()?;
    let mut tool = Tool::new(
        Cow::Borrowed(name),
        Cow::Borrowed(description),
        Arc::new(input_schema),
    );
    tool.output_schema = Some(Arc::new(output_schema));
    Ok(tool)
}

fn schema_object<T: JsonSchema>() -> anyhow::Result<JsonObject> {
    let value = serde_json::to_value(schema_for!(T))?;
    Ok(serde_json::from_value(value)?)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn exposes_only_workspace_authoring_tools() {
        let root = tempfile::tempdir().expect("tempdir");
        let server = AuthorMcpServer::new(root.path()).expect("server");
        let names = server
            .tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "forall_author_status",
                "forall_author_init",
                "forall_author_discover",
                "forall_author_upsert_requirements",
                "forall_author_scaffold_contracts",
                "forall_author_validate",
                "forall_author_scaffold_property",
            ]
        );
    }

    #[tokio::test]
    async fn initializes_and_lists_tools_over_mcp_transport() -> anyhow::Result<()> {
        let root = tempfile::tempdir()?;
        std::fs::create_dir_all(root.path().join("src"))?;
        std::fs::write(
            root.path().join("src/clamp.ts"),
            "export function clamp(x: number) {\n  return x;\n}\n",
        )?;
        let (server_transport, client_transport) = tokio::io::duplex(16 * 1024);
        let server = AuthorMcpServer::new(root.path())?;
        tokio::spawn(async move {
            let running = server.serve(server_transport).await?;
            running.waiting().await?;
            anyhow::Ok(())
        });

        let client = ().serve(client_transport).await?;
        let result = client.peer().list_tools(None).await?;
        assert_eq!(result.tools.len(), 7);
        assert!(
            result
                .tools
                .iter()
                .any(|tool| tool.name == "forall_author_validate")
        );
        let status = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams::new(
                "forall_author_status",
            ))
            .await?;
        assert_eq!(
            status
                .structured_content
                .as_ref()
                .and_then(|content| content.get("ok")),
            Some(&serde_json::json!(true))
        );

        let preview = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_init").with_arguments(
                    serde_json::from_value(serde_json::json!({ "mode": "preview" }))?,
                ),
            )
            .await?;
        assert_eq!(preview.is_error, Some(false));
        assert!(!root.path().join(".forall/verify/mapping.yaml").exists());

        let applied = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_init").with_arguments(
                    serde_json::from_value(serde_json::json!({ "mode": "apply" }))?,
                ),
            )
            .await?;
        assert_eq!(applied.is_error, Some(false));
        assert!(root.path().join(".forall/verify/mapping.yaml").is_file());

        let discovered = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_discover").with_arguments(
                    serde_json::from_value(serde_json::json!({
                        "languages": ["type_script"]
                    }))?,
                ),
            )
            .await?;
        let discovered_json = discovered.structured_content.expect("structured discovery");
        assert_eq!(
            discovered_json["result"]["symbols"][0]["name"],
            serde_json::json!("clamp")
        );

        let rejected = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_discover").with_arguments(
                    serde_json::from_value(serde_json::json!({
                        "files": ["../outside.ts"]
                    }))?,
                ),
            )
            .await?;
        assert_eq!(rejected.is_error, Some(true));

        let mapping_hash = status.structured_content.as_ref().and_then(|content| {
            content["result"]["mapping_sha256"]
                .as_str()
                .map(str::to_string)
        });
        assert!(mapping_hash.is_none(), "status was captured before init");
        let initialized_status = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams::new(
                "forall_author_status",
            ))
            .await?;
        let mapping_hash =
            initialized_status.structured_content.expect("status")["result"]["mapping_sha256"]
                .as_str()
                .expect("mapping hash")
                .to_string();
        let upserted = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_upsert_requirements")
                    .with_arguments(serde_json::from_value(serde_json::json!({
                        "mode": "apply",
                        "expected_mapping_sha256": mapping_hash,
                        "requirements": [{
                            "id": "clamp-bounds",
                            "capability": "math",
                            "requirement": "Clamp stays within the requested bounds",
                            "verified": false,
                            "property_tested": false,
                            "code": {
                                "file": "src/clamp.ts",
                                "symbols": ["clamp"]
                            },
                            "contract": "requires true"
                        }]
                    }))?),
            )
            .await?;
        assert_eq!(upserted.is_error, Some(false));
        let mapping_hash =
            upserted.structured_content.expect("upsert")["result"]["files"][0]["after_sha256"]
                .as_str()
                .expect("updated mapping hash")
                .to_string();

        use sha2::{Digest, Sha256};
        let source_hash = format!(
            "{:x}",
            Sha256::digest(std::fs::read(root.path().join("src/clamp.ts"))?)
        );
        let contract_arguments = serde_json::json!({
            "mode": "apply",
            "contracts": [{
                "file": "src/clamp.ts",
                "symbol": "clamp",
                "requires": ["true"],
                "ensures": ["result == x"],
                "expected_sha256": source_hash
            }]
        });
        let scaffolded = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_scaffold_contracts")
                    .with_arguments(serde_json::from_value(contract_arguments)?),
            )
            .await?;
        assert_eq!(scaffolded.is_error, Some(false));

        let property = client
            .peer()
            .call_tool(
                rmcp::model::CallToolRequestParams::new("forall_author_scaffold_property")
                    .with_arguments(serde_json::from_value(serde_json::json!({
                        "mode": "apply",
                        "requirement_id": "clamp-bounds",
                        "body": "export const clampBounds = () => true;\n",
                        "symbol": "clampBounds",
                        "expected_mapping_sha256": mapping_hash
                    }))?),
            )
            .await?;
        assert_eq!(property.is_error, Some(false));

        let validation = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams::new(
                "forall_author_validate",
            ))
            .await?;
        assert_eq!(
            validation.structured_content.expect("validation")["result"]["valid"],
            serde_json::json!(true)
        );
        client.cancel().await?;
        Ok(())
    }
}
