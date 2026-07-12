use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub const ASSEMBLY_CONTRACT_VERSION: &str = "everarcade.runtime-assembly.v1";
pub const WORLD_CREATE_REQUEST_VERSION: &str = "everarcade.world-create-request.v1";
pub const PTW_RUNTIME_CONTRACT_VERSION: &str = "everarcade.ptw-runtime.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalWorldRequest {
    pub schema_version: String,
    pub world_id: String,
    pub world_name: String,
    #[serde(default)]
    pub profiles: BTreeMap<String, String>,
    #[serde(default)]
    pub module_references: BTreeMap<String, String>,
    #[serde(default)]
    pub runtime_overrides: BTreeMap<String, Value>,
    #[serde(default)]
    pub content_overrides: BTreeMap<String, Value>,
    #[serde(default)]
    pub projection_declarations: BTreeMap<String, Value>,
    #[serde(default)]
    pub proof_policy: BTreeMap<String, Value>,
    #[serde(default)]
    pub economy_metadata: BTreeMap<String, Value>,
    #[serde(default)]
    pub generation_provenance: BTreeMap<String, Value>,
    #[serde(default)]
    pub compiler_policy: BTreeMap<String, Value>,
}

impl CanonicalWorldRequest {
    pub fn from_request_bytes(bytes: &[u8]) -> Result<Self, String> {
        let raw: Value = serde_json::from_slice(bytes)
            .map_err(|e| format!("invalid world configuration: {e}"))?;
        if raw.get("schema_version").and_then(Value::as_str) == Some(WORLD_CREATE_REQUEST_VERSION) {
            let mut request: Self = serde_json::from_value(raw)
                .map_err(|e| format!("invalid canonical world request: {e}"))?;
            request.normalize()?;
            Ok(request)
        } else {
            LegacyWorldCreateRequest::from_value(raw)?.into_canonical()
        }
    }

    pub fn normalize(&mut self) -> Result<(), String> {
        self.world_id = normalize_id(&self.world_id);
        self.world_name = self.world_name.trim().to_string();
        if self.schema_version != WORLD_CREATE_REQUEST_VERSION {
            return Err(format!(
                "unsupported request schema_version: {}",
                self.schema_version
            ));
        }
        if self.world_id.is_empty() {
            return Err("canonical world request requires world_id".into());
        }
        if self.world_name.is_empty() {
            return Err("canonical world request requires world_name".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegacyWorldCreateRequest {
    world_profile: String,
    genre_profile: String,
    biome_profile: String,
    projection_profile: String,
    proof_profile: String,
    #[serde(default = "default_world_name")]
    world_name: String,
}

impl LegacyWorldCreateRequest {
    fn from_value(raw: Value) -> Result<Self, String> {
        serde_json::from_value(raw).map_err(|e| format!("invalid legacy world configuration: {e}"))
    }

    fn into_canonical(self) -> Result<CanonicalWorldRequest, String> {
        let mut profiles = BTreeMap::new();
        profiles.insert("world".into(), self.world_profile);
        profiles.insert("genre".into(), self.genre_profile);
        profiles.insert("biome".into(), self.biome_profile);
        profiles.insert("projection".into(), self.projection_profile);
        profiles.insert("proof".into(), self.proof_profile);
        let mut request = CanonicalWorldRequest {
            schema_version: WORLD_CREATE_REQUEST_VERSION.into(),
            world_id: format!("world-{}", slug(&self.world_name)),
            world_name: self.world_name,
            profiles,
            module_references: BTreeMap::new(),
            runtime_overrides: BTreeMap::new(),
            content_overrides: BTreeMap::new(),
            projection_declarations: BTreeMap::new(),
            proof_policy: BTreeMap::new(),
            economy_metadata: BTreeMap::new(),
            generation_provenance: BTreeMap::new(),
            compiler_policy: BTreeMap::new(),
        };
        request.normalize()?;
        Ok(request)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblyStage {
    InputNormalization,
    ProfileDiscovery,
    DependencyResolution,
    TypedContributionLoading,
    DeterministicContributionMerge,
    SymbolAndReferenceResolution,
    RuntimeIrConstruction,
    StaticValidation,
    InvariantAndBoundAnalysis,
    CapabilityCompatibility,
    CanonicalLowering,
    PackageEmission,
    HashAndProofManifestGeneration,
    ConformanceSelfVerification,
}

impl AssemblyStage {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InputNormalization => "stage_00_input_normalization",
            Self::ProfileDiscovery => "stage_01_profile_discovery",
            Self::DependencyResolution => "stage_02_dependency_resolution",
            Self::TypedContributionLoading => "stage_03_typed_contribution_loading",
            Self::DeterministicContributionMerge => "stage_04_deterministic_contribution_merge",
            Self::SymbolAndReferenceResolution => "stage_05_symbol_and_reference_resolution",
            Self::RuntimeIrConstruction => "stage_06_runtime_ir_construction",
            Self::StaticValidation => "stage_07_static_validation",
            Self::InvariantAndBoundAnalysis => "stage_08_invariant_and_bound_analysis",
            Self::CapabilityCompatibility => "stage_09_capability_compatibility",
            Self::CanonicalLowering => "stage_10_canonical_lowering",
            Self::PackageEmission => "stage_11_package_emission",
            Self::HashAndProofManifestGeneration => "stage_12_hash_and_proof_manifest_generation",
            Self::ConformanceSelfVerification => "stage_13_conformance_self_verification",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssemblyDiagnostic {
    pub code: String,
    pub severity: DiagnosticSeverity,
    pub stage: AssemblyStage,
    pub namespace: String,
    pub source: String,
    pub affected_id: String,
    pub message: String,
    pub suggested_remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContributionProvenance {
    pub source_profile: String,
    pub source_version: String,
    pub source_content_hash: String,
    pub contribution_id: String,
    pub priority_policy: String,
    pub override_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TypedContributionsV1 {
    pub runtime: Vec<TypedContribution<Value>>,
    pub limits: Vec<TypedContribution<Value>>,
    pub topology: Vec<TypedContribution<Value>>,
    pub regions: Vec<TypedContribution<Value>>,
    pub archetypes: Vec<TypedContribution<Value>>,
    pub entities: Vec<TypedContribution<Value>>,
    pub primitives: Vec<TypedContribution<Value>>,
    pub actions: Vec<TypedContribution<Value>>,
    pub transitions: Vec<TypedContribution<Value>>,
    pub invariants: Vec<TypedContribution<Value>>,
    pub encounters: Vec<TypedContribution<Value>>,
    pub progression: Vec<TypedContribution<Value>>,
    pub content: Vec<TypedContribution<Value>>,
    pub projection: Vec<TypedContribution<Value>>,
    pub proof: Vec<TypedContribution<Value>>,
    pub economy: Vec<TypedContribution<Value>>,
    pub ai: Vec<TypedContribution<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypedContribution<T> {
    pub provenance: ContributionProvenance,
    pub value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PtwRuntimeIrV1 {
    pub contract_version: String,
    pub assembly_contract_version: String,
    pub world_id: String,
    pub world_name: String,
    pub resolved_profiles: BTreeMap<String, String>,
    pub module_references: BTreeMap<String, String>,
    pub capability_requirements: Vec<String>,
    pub limits: BTreeMap<String, u64>,
    pub topology_graph: BTreeMap<String, Value>,
    pub regions: BTreeMap<String, Value>,
    pub spawn_points: BTreeMap<String, Value>,
    pub archetypes: BTreeMap<String, Value>,
    pub initial_entities: BTreeMap<String, Value>,
    pub world_variables: BTreeMap<String, Value>,
    pub primitive_configurations: BTreeMap<String, Value>,
    pub actions: BTreeMap<String, Value>,
    pub transition_bindings: BTreeMap<String, Value>,
    pub invariants: BTreeMap<String, Value>,
    pub encounters: BTreeMap<String, Value>,
    pub progression: BTreeMap<String, Value>,
    pub authoritative_content: BTreeMap<String, Value>,
    pub checkpoint_policy: BTreeMap<String, Value>,
    pub journal_policy: BTreeMap<String, Value>,
    pub root_policy: BTreeMap<String, Value>,
    pub proof_constraints: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssemblyManifestV1 {
    pub schema_version: String,
    pub compiler: String,
    pub request_schema_version: String,
    pub runtime_contract_version: String,
    pub stages: Vec<String>,
    pub resolved_profiles: BTreeMap<String, String>,
    pub diagnostics: Vec<AssemblyDiagnostic>,
    pub conformance_status: String,
    pub proof_readiness_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssembledWorld {
    pub request: CanonicalWorldRequest,
    pub contributions: TypedContributionsV1,
    pub ir: PtwRuntimeIrV1,
    pub diagnostics: Vec<AssemblyDiagnostic>,
    pub manifest: AssemblyManifestV1,
}

pub fn assemble_world(request: CanonicalWorldRequest) -> Result<AssembledWorld, String> {
    let contributions = TypedContributionsV1::default();
    let ir = PtwRuntimeIrV1 {
        contract_version: PTW_RUNTIME_CONTRACT_VERSION.into(),
        assembly_contract_version: ASSEMBLY_CONTRACT_VERSION.into(),
        world_id: request.world_id.clone(),
        world_name: request.world_name.clone(),
        resolved_profiles: request.profiles.clone(),
        module_references: request.module_references.clone(),
        capability_requirements: Vec::new(),
        limits: BTreeMap::new(),
        topology_graph: BTreeMap::new(),
        regions: BTreeMap::new(),
        spawn_points: BTreeMap::new(),
        archetypes: BTreeMap::new(),
        initial_entities: BTreeMap::new(),
        world_variables: BTreeMap::new(),
        primitive_configurations: BTreeMap::new(),
        actions: BTreeMap::new(),
        transition_bindings: BTreeMap::new(),
        invariants: BTreeMap::new(),
        encounters: BTreeMap::new(),
        progression: BTreeMap::new(),
        authoritative_content: BTreeMap::new(),
        checkpoint_policy: BTreeMap::new(),
        journal_policy: BTreeMap::new(),
        root_policy: BTreeMap::new(),
        proof_constraints: request.proof_policy.clone(),
    };
    let diagnostics = sorted_diagnostics(vec![AssemblyDiagnostic {
        code: "ASSEMBLY_SKELETON_EMPTY_CONTRIBUTIONS".into(),
        severity: DiagnosticSeverity::Info,
        stage: AssemblyStage::TypedContributionLoading,
        namespace: "contributions".into(),
        source: "runtime-assembly-skeleton".into(),
        affected_id: request.world_id.clone(),
        message: "Runtime Assembly Engine skeleton loaded zero production profile contributions."
            .into(),
        suggested_remediation:
            "Implement profile catalog and contribution loading in the next phase.".into(),
    }]);
    let manifest = AssemblyManifestV1 {
        schema_version: ASSEMBLY_CONTRACT_VERSION.into(),
        compiler: "everarcade-compiler".into(),
        request_schema_version: request.schema_version.clone(),
        runtime_contract_version: PTW_RUNTIME_CONTRACT_VERSION.into(),
        stages: assembly_stages()
            .into_iter()
            .map(|s| s.code().into())
            .collect(),
        resolved_profiles: request.profiles.clone(),
        diagnostics: diagnostics.clone(),
        conformance_status: "skeleton-self-verified".into(),
        proof_readiness_status: "not-evaluated-in-skeleton".into(),
    };
    Ok(AssembledWorld {
        request,
        contributions,
        ir,
        diagnostics,
        manifest,
    })
}

fn assembly_stages() -> Vec<AssemblyStage> {
    vec![
        AssemblyStage::InputNormalization,
        AssemblyStage::ProfileDiscovery,
        AssemblyStage::DependencyResolution,
        AssemblyStage::TypedContributionLoading,
        AssemblyStage::DeterministicContributionMerge,
        AssemblyStage::SymbolAndReferenceResolution,
        AssemblyStage::RuntimeIrConstruction,
        AssemblyStage::StaticValidation,
        AssemblyStage::InvariantAndBoundAnalysis,
        AssemblyStage::CapabilityCompatibility,
        AssemblyStage::CanonicalLowering,
        AssemblyStage::PackageEmission,
        AssemblyStage::HashAndProofManifestGeneration,
        AssemblyStage::ConformanceSelfVerification,
    ]
}

fn sorted_diagnostics(mut diagnostics: Vec<AssemblyDiagnostic>) -> Vec<AssemblyDiagnostic> {
    diagnostics.sort();
    diagnostics
}

fn default_world_name() -> String {
    "EverArcade Catacombs: The Endless Gate".into()
}
fn normalize_id(id: &str) -> String {
    slug(id)
}
fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_request_adapts_to_canonical_request() {
        let request = CanonicalWorldRequest::from_request_bytes(
            br#"{
            "world_profile":"ptw-full-v1",
            "genre_profile":"arpg-v1",
            "biome_profile":"catacombs-v1",
            "projection_profile":"arpg-web-v1",
            "proof_profile":"live-replay-ceremony-v1",
            "world_name":"EverArcade Catacombs: The Endless Gate"
        }"#,
        )
        .unwrap();
        assert_eq!(request.schema_version, WORLD_CREATE_REQUEST_VERSION);
        assert_eq!(request.profiles.get("genre").unwrap(), "arpg-v1");
        assert_eq!(
            request.world_id,
            "world-everarcade-catacombs-the-endless-gate"
        );
    }

    #[test]
    fn assembly_skeleton_builds_empty_runtime_ir_boundary() {
        let request = CanonicalWorldRequest::from_request_bytes(
            br#"{
            "schema_version":"everarcade.world-create-request.v1",
            "world_id":"world_demo",
            "world_name":"Demo World",
            "profiles":{"world":"ptw-full-v1","genre":"social-v1"}
        }"#,
        )
        .unwrap();
        let assembled = assemble_world(request).unwrap();
        assert_eq!(assembled.ir.contract_version, PTW_RUNTIME_CONTRACT_VERSION);
        assert!(assembled.contributions.actions.is_empty());
        assert_eq!(assembled.manifest.stages.len(), 14);
        assert_eq!(
            assembled.diagnostics[0].code,
            "ASSEMBLY_SKELETON_EMPTY_CONTRIBUTIONS"
        );
    }
}
