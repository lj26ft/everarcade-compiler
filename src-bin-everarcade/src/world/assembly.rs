use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const ASSEMBLY_CONTRACT_VERSION: &str = "everarcade.runtime-assembly.v1";
pub const WORLD_CREATE_REQUEST_VERSION: &str = "everarcade.world-create-request.v1";
pub const PTW_RUNTIME_CONTRACT_VERSION: &str = "everarcade.ptw-runtime.v1";
pub const PROFILE_SCHEMA_VERSION: &str = "everarcade.profile.v1";
pub const PROFILE_GRAPH_SCHEMA_VERSION: &str = "everarcade.resolved-profile-graph.v1";
pub const PROFILE_GRAPH_HASH_DOMAIN: &str = "everarcade.profile-graph.v1";
pub const COMPILER_CAPABILITIES_SCHEMA_VERSION: &str = "everarcade.compiler-capabilities.v1";

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
    ValidateProfileCapabilities,
    ValidateProfileNamespaces,
    PrepareContributionLoading,
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
            Self::ValidateProfileCapabilities => "stage_03_validate_profile_capabilities",
            Self::ValidateProfileNamespaces => "stage_04_validate_profile_namespaces",
            Self::PrepareContributionLoading => "stage_05_prepare_contribution_loading",
            Self::TypedContributionLoading => "stage_06_typed_contribution_loading",
            Self::DeterministicContributionMerge => "stage_07_deterministic_contribution_merge",
            Self::SymbolAndReferenceResolution => "stage_08_symbol_and_reference_resolution",
            Self::RuntimeIrConstruction => "stage_09_runtime_ir_construction",
            Self::StaticValidation => "stage_10_static_validation",
            Self::InvariantAndBoundAnalysis => "stage_11_invariant_and_bound_analysis",
            Self::CapabilityCompatibility => "stage_12_capability_compatibility",
            Self::CanonicalLowering => "stage_13_canonical_lowering",
            Self::PackageEmission => "stage_14_package_emission",
            Self::HashAndProofManifestGeneration => "stage_15_hash_and_proof_manifest_generation",
            Self::ConformanceSelfVerification => "stage_16_conformance_self_verification",
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProfileId {
    pub provider: String,
    pub category: ProfileCategory,
    pub name: String,
}
impl ProfileId {
    pub fn key(&self) -> String {
        format!("{}.{}.{}", self.provider, self.category.as_str(), self.name)
    }
    pub fn identity(&self, version: &str) -> String {
        format!("{}@{}", self.key(), version)
    }
}
pub type ProfileVersion = String;
pub type ProfileSchemaVersion = String;
pub type ProviderNamespace = String;
pub type ProfileCapabilityRequirement = String;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCategory {
    World,
    Runtime,
    Genre,
    Topology,
    Biome,
    Encounter,
    Progression,
    Economy,
    Proof,
    Projection,
    Ai,
    Access,
    Module,
}
impl ProfileCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Runtime => "runtime",
            Self::Genre => "genre",
            Self::Topology => "topology",
            Self::Biome => "biome",
            Self::Encounter => "encounter",
            Self::Progression => "progression",
            Self::Economy => "economy",
            Self::Proof => "proof",
            Self::Projection => "projection",
            Self::Ai => "ai",
            Self::Access => "access",
            Self::Module => "module",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ContributionNamespace {
    Runtime,
    Limits,
    Topology,
    Regions,
    SpawnPoints,
    Archetypes,
    Entities,
    Primitives,
    Actions,
    Transitions,
    Invariants,
    Encounters,
    Progression,
    Content,
    Projection,
    Proof,
    Economy,
    Ai,
    Access,
}
impl ContributionNamespace {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Limits => "limits",
            Self::Topology => "topology",
            Self::Regions => "regions",
            Self::SpawnPoints => "spawn_points",
            Self::Archetypes => "archetypes",
            Self::Entities => "entities",
            Self::Primitives => "primitives",
            Self::Actions => "actions",
            Self::Transitions => "transitions",
            Self::Invariants => "invariants",
            Self::Encounters => "encounters",
            Self::Progression => "progression",
            Self::Content => "content",
            Self::Projection => "projection",
            Self::Proof => "proof",
            Self::Economy => "economy",
            Self::Ai => "ai",
            Self::Access => "access",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProfileReference {
    pub provider: String,
    pub category: ProfileCategory,
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub compatible_version: Option<String>,
}
impl ProfileReference {
    pub fn key(&self) -> String {
        ProfileId {
            provider: self.provider.clone(),
            category: self.category.clone(),
            name: self.name.clone(),
        }
        .key()
    }
    pub fn identity(&self) -> String {
        match &self.version {
            Some(v) => format!("{}@{}", self.key(), v),
            None => self.key(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileDependency {
    pub reference: ProfileReference,
    pub version_requirement: String,
    pub required: bool,
    pub expected_content_hash: Option<String>,
    #[serde(default)]
    pub required_contribution_namespaces: Vec<ContributionNamespace>,
    pub compatibility_policy: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileConflict {
    pub reference: ProfileReference,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileSource {
    pub source_type: String,
    pub provenance: String,
    #[serde(default)]
    pub immutable_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileDefinition {
    pub schema_version: ProfileSchemaVersion,
    pub id: ProfileId,
    pub version: ProfileVersion,
    pub content_hash: String,
    pub category: ProfileCategory,
    #[serde(default)]
    pub dependencies: Vec<ProfileDependency>,
    #[serde(default)]
    pub optional_dependencies: Vec<ProfileDependency>,
    #[serde(default)]
    pub conflicts: Vec<ProfileConflict>,
    #[serde(default)]
    pub required_compiler_capabilities: Vec<ProfileCapabilityRequirement>,
    #[serde(default)]
    pub declared_contribution_namespaces: Vec<ContributionNamespace>,
    pub source: ProfileSource,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub replacement: Option<ProfileReference>,
}
impl ProfileDefinition {
    pub fn identity(&self) -> String {
        self.id.identity(&self.version)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileCatalog {
    entries: BTreeMap<String, ProfileDefinition>,
}
impl ProfileCatalog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, p: ProfileDefinition) {
        self.entries.insert(p.identity(), p);
    }
    pub fn lookup_exact(&self, r: &ProfileReference) -> Option<&ProfileDefinition> {
        r.version
            .as_ref()
            .and_then(|v| self.entries.get(&format!("{}@{}", r.key(), v)))
    }
    pub fn compatible(&self, r: &ProfileReference) -> Vec<&ProfileDefinition> {
        self.entries
            .values()
            .filter(|p| p.id.key() == r.key())
            .collect()
    }
    pub fn entries(&self) -> Vec<&ProfileDefinition> {
        self.entries.values().collect()
    }
    pub fn catalog_hash(&self) -> String {
        hash_json(&json!({"domain":"everarcade.profile-catalog.v1","entries":self.entries}))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolvedProfileNodeV1 {
    pub identity: String,
    pub provider: String,
    pub category: ProfileCategory,
    pub name: String,
    pub version: String,
    pub content_hash: String,
    pub source: ProfileSource,
    pub required_capabilities: Vec<String>,
    pub contribution_namespaces: Vec<ContributionNamespace>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolvedProfileEdgeV1 {
    pub from: String,
    pub to: String,
    pub required: bool,
    pub expected_content_hash: Option<String>,
    pub required_contribution_namespaces: Vec<ContributionNamespace>,
    pub compatibility_policy: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OptionalDependencyDecisionV1 {
    pub from: String,
    pub target: String,
    pub selected: bool,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationResultV1 {
    pub status: String,
    pub checked: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolvedProfileGraphV1 {
    pub schema_version: String,
    pub requested_root_profiles: Vec<String>,
    pub resolved_profile_nodes: Vec<ResolvedProfileNodeV1>,
    pub dependency_edges: Vec<ResolvedProfileEdgeV1>,
    pub optional_dependency_decisions: Vec<OptionalDependencyDecisionV1>,
    pub stable_topological_order: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub contribution_namespaces: Vec<ContributionNamespace>,
    pub source_provenance: BTreeMap<String, ProfileSource>,
    pub conflict_check: ValidationResultV1,
    pub capability_check: ValidationResultV1,
    pub graph_hash: String,
    pub diagnostics: Vec<AssemblyDiagnostic>,
}

fn compiler_capabilities() -> BTreeSet<String> {
    [
        "ptw.runtime.v1",
        "ptw.primitive.identity.v1",
        "ptw.primitive.movement.v1",
        "ptw.action.player_join.v1",
        "ptw.action.entity_move.v1",
        "ptw.proof.replay.v1",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}
fn ref_for(provider: &str, cat: ProfileCategory, name: &str, version: &str) -> ProfileReference {
    ProfileReference {
        provider: provider.into(),
        category: cat,
        name: name.into(),
        version: Some(version.into()),
        compatible_version: None,
    }
}
fn dep(r: ProfileReference) -> ProfileDependency {
    ProfileDependency {
        reference: r,
        version_requirement: "exact".into(),
        required: true,
        expected_content_hash: None,
        required_contribution_namespaces: vec![],
        compatibility_policy: "exact-only".into(),
    }
}
fn prof(
    cat: ProfileCategory,
    name: &str,
    caps: Vec<&str>,
    ns: Vec<ContributionNamespace>,
    deps: Vec<ProfileDependency>,
) -> ProfileDefinition {
    let id = ProfileId {
        provider: "everarcade".into(),
        category: cat.clone(),
        name: name.into(),
    };
    let version = "1.0.0".to_string();
    let identity = id.identity(&version);
    ProfileDefinition {
        schema_version: PROFILE_SCHEMA_VERSION.into(),
        id,
        version,
        content_hash: hash_json(&json!({"fixture":identity})),
        category: cat,
        dependencies: deps,
        optional_dependencies: vec![],
        conflicts: vec![],
        required_compiler_capabilities: caps.into_iter().map(str::to_string).collect(),
        declared_contribution_namespaces: ns,
        source: ProfileSource {
            source_type: "embedded_fixture".into(),
            provenance: format!("builtin-fixture:{name}"),
            immutable_ref: identity,
        },
        deprecated: false,
        replacement: None,
    }
}
pub fn builtin_profile_catalog() -> ProfileCatalog {
    let mut c = ProfileCatalog::new();
    let base = prof(
        ProfileCategory::World,
        "base-world-v1",
        vec!["ptw.runtime.v1"],
        vec![ContributionNamespace::Runtime],
        vec![],
    );
    c.register(base);
    c.register(prof(
        ProfileCategory::Runtime,
        "movement-enabled-v1",
        vec!["ptw.primitive.movement.v1", "ptw.action.entity_move.v1"],
        vec![
            ContributionNamespace::Primitives,
            ContributionNamespace::Actions,
        ],
        vec![dep(ref_for(
            "everarcade",
            ProfileCategory::World,
            "base-world-v1",
            "1.0.0",
        ))],
    ));
    c.register(prof(
        ProfileCategory::Runtime,
        "movement-disabled-v1",
        vec!["ptw.runtime.v1"],
        vec![ContributionNamespace::Runtime],
        vec![dep(ref_for(
            "everarcade",
            ProfileCategory::World,
            "base-world-v1",
            "1.0.0",
        ))],
    ));
    c.register(prof(
        ProfileCategory::Topology,
        "simple-topology-v1",
        vec!["ptw.runtime.v1"],
        vec![
            ContributionNamespace::Topology,
            ContributionNamespace::Regions,
            ContributionNamespace::SpawnPoints,
        ],
        vec![dep(ref_for(
            "everarcade",
            ProfileCategory::Runtime,
            "movement-enabled-v1",
            "1.0.0",
        ))],
    ));
    c.register(prof(
        ProfileCategory::Proof,
        "replay-proof-v1",
        vec!["ptw.proof.replay.v1"],
        vec![ContributionNamespace::Proof],
        vec![],
    ));
    c.register(prof(
        ProfileCategory::Runtime,
        "unsupported-capability-v1",
        vec!["ptw.unsupported.future.v1"],
        vec![ContributionNamespace::Runtime],
        vec![],
    ));
    c.register(prof(
        ProfileCategory::Module,
        "dependency-cycle-a-v1",
        vec![],
        vec![],
        vec![dep(ref_for(
            "everarcade",
            ProfileCategory::Module,
            "dependency-cycle-b-v1",
            "1.0.0",
        ))],
    ));
    c.register(prof(
        ProfileCategory::Module,
        "dependency-cycle-b-v1",
        vec![],
        vec![],
        vec![dep(ref_for(
            "everarcade",
            ProfileCategory::Module,
            "dependency-cycle-a-v1",
            "1.0.0",
        ))],
    ));
    let mut a = prof(
        ProfileCategory::Module,
        "conflict-a-v1",
        vec![],
        vec![],
        vec![],
    );
    a.conflicts.push(ProfileConflict {
        reference: ref_for(
            "everarcade",
            ProfileCategory::Module,
            "conflict-b-v1",
            "1.0.0",
        ),
        reason: "fixture direct conflict".into(),
    });
    c.register(a);
    c.register(prof(
        ProfileCategory::Module,
        "conflict-b-v1",
        vec![],
        vec![],
        vec![],
    ));
    let mut opt = prof(
        ProfileCategory::World,
        "optional-dependency-root-v1",
        vec!["ptw.runtime.v1"],
        vec![ContributionNamespace::Runtime],
        vec![],
    );
    opt.optional_dependencies.push(ProfileDependency {
        required: false,
        ..dep(ref_for(
            "everarcade",
            ProfileCategory::Module,
            "missing-optional-v1",
            "1.0.0",
        ))
    });
    c.register(opt);
    c
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
    pub spawn_points: Vec<TypedContribution<Value>>,
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
    pub access: Vec<TypedContribution<Value>>,
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
    pub resolved_profile_graph: ResolvedProfileGraphV1,
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
    pub profile_schema_version: String,
    pub requested_root_profiles: Vec<String>,
    pub profile_content_hashes: BTreeMap<String, String>,
    pub dependency_graph: Vec<ResolvedProfileEdgeV1>,
    pub stable_resolution_order: Vec<String>,
    pub profile_graph_hash: String,
    pub required_capabilities: Vec<String>,
    pub capability_validation_result: ValidationResultV1,
    pub conflict_validation_result: ValidationResultV1,
    pub source_provenance: BTreeMap<String, ProfileSource>,
    pub resolution_diagnostics: Vec<AssemblyDiagnostic>,
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
    let catalog = builtin_profile_catalog();
    let graph = resolve_profile_graph(request.profiles.values().cloned().collect(), &catalog);
    let contributions = TypedContributionsV1::default();
    let resolved_profiles = graph
        .resolved_profile_nodes
        .iter()
        .map(|n| (n.category.as_str().to_string(), n.identity.clone()))
        .collect();
    let ir = PtwRuntimeIrV1 {
        contract_version: PTW_RUNTIME_CONTRACT_VERSION.into(),
        assembly_contract_version: ASSEMBLY_CONTRACT_VERSION.into(),
        world_id: request.world_id.clone(),
        world_name: request.world_name.clone(),
        resolved_profiles,
        module_references: request.module_references.clone(),
        capability_requirements: graph.required_capabilities.clone(),
        limits: BTreeMap::new(),
        topology_graph: BTreeMap::new(),
        regions: BTreeMap::new(),
        spawn_points: BTreeMap::new(),
        resolved_profile_graph: graph.clone(),
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
    let mut diagnostics = graph.diagnostics.clone();
    diagnostics.push(AssemblyDiagnostic {
        code: "ASSEMBLY_PROFILE_CONTRIBUTION_LOADING_DEFERRED".into(),
        severity: DiagnosticSeverity::Info,
        stage: AssemblyStage::PrepareContributionLoading,
        namespace: "contributions".into(),
        source: "runtime-assembly-profile-resolution".into(),
        affected_id: request.world_id.clone(),
        message: "Profile graph resolved; typed contribution merging is deferred to Phase C."
            .into(),
        suggested_remediation:
            "Load declared profile contribution namespaces in the Phase C merge engine.".into(),
    });
    let diagnostics = sorted_diagnostics(diagnostics);
    let profile_content_hashes = graph
        .resolved_profile_nodes
        .iter()
        .map(|n| (n.identity.clone(), n.content_hash.clone()))
        .collect();
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
        profile_schema_version: PROFILE_SCHEMA_VERSION.into(),
        requested_root_profiles: graph.requested_root_profiles.clone(),
        profile_content_hashes,
        dependency_graph: graph.dependency_edges.clone(),
        stable_resolution_order: graph.stable_topological_order.clone(),
        profile_graph_hash: graph.graph_hash.clone(),
        required_capabilities: graph.required_capabilities.clone(),
        capability_validation_result: graph.capability_check.clone(),
        conflict_validation_result: graph.conflict_check.clone(),
        source_provenance: graph.source_provenance.clone(),
        resolution_diagnostics: graph.diagnostics.clone(),
        diagnostics: diagnostics.clone(),
        conformance_status: "profile-resolution-self-verified".into(),
        proof_readiness_status: "profile-graph-resolved-contributions-deferred".into(),
    };
    Ok(AssembledWorld {
        request,
        contributions,
        ir,
        diagnostics,
        manifest,
    })
}

pub fn resolve_profile_graph(
    requested: Vec<String>,
    catalog: &ProfileCatalog,
) -> ResolvedProfileGraphV1 {
    let mut roots: Vec<ProfileReference> = requested
        .into_iter()
        .filter_map(|s| parse_profile_ref(&s))
        .collect();
    roots.sort_by_key(|r| r.identity());
    roots.dedup_by_key(|r| r.identity());
    let mut diagnostics = Vec::new();
    let mut nodes: BTreeMap<String, ProfileDefinition> = BTreeMap::new();
    let mut edges = Vec::new();
    let mut optional = Vec::new();
    for r in &roots {
        visit(
            r,
            catalog,
            &mut nodes,
            &mut edges,
            &mut optional,
            &mut diagnostics,
            Vec::new(),
            "root",
        );
    }
    for p in nodes.values() {
        for cfl in &p.conflicts {
            if let Some(v) = &cfl.reference.version {
                let id = format!("{}@{}", cfl.reference.key(), v);
                if nodes.contains_key(&id) {
                    diagnostics.push(diag(
                        "ASSEMBLY_PROFILE_CONFLICT",
                        AssemblyStage::DependencyResolution,
                        &p.identity(),
                        &p.source.provenance,
                        &format!(
                            "profile {} conflicts with {}: {}",
                            p.identity(),
                            id,
                            cfl.reason
                        ),
                        "Remove one of the conflicting profiles.",
                    ));
                }
            }
        }
    }
    let supported = compiler_capabilities();
    let mut caps = BTreeSet::new();
    let mut nss = BTreeSet::new();
    for p in nodes.values() {
        if p.schema_version != PROFILE_SCHEMA_VERSION {
            diagnostics.push(diag(
                "ASSEMBLY_PROFILE_UNSUPPORTED_SCHEMA",
                AssemblyStage::ProfileDiscovery,
                &p.identity(),
                &p.source.provenance,
                "profile schema is not supported",
                "Use everarcade.profile.v1.",
            ));
        }
        for cap in &p.required_compiler_capabilities {
            caps.insert(cap.clone());
            if !supported.contains(cap) {
                diagnostics.push(diag(
                    "ASSEMBLY_UNSUPPORTED_CAPABILITY",
                    AssemblyStage::ValidateProfileCapabilities,
                    &p.identity(),
                    &p.source.provenance,
                    &format!("unsupported compiler capability {cap}"),
                    "Use a compiler that advertises the required capability or remove the profile.",
                ));
            }
        }
        for ns in &p.declared_contribution_namespaces {
            nss.insert(ns.clone());
        }
    }
    let order = topo(&nodes, &edges, &mut diagnostics);
    let node_vec: Vec<_> = order
        .iter()
        .filter_map(|id| nodes.get(id))
        .map(|p| ResolvedProfileNodeV1 {
            identity: p.identity(),
            provider: p.id.provider.clone(),
            category: p.category.clone(),
            name: p.id.name.clone(),
            version: p.version.clone(),
            content_hash: p.content_hash.clone(),
            source: p.source.clone(),
            required_capabilities: sorted(p.required_compiler_capabilities.clone()),
            contribution_namespaces: sorted_ns(p.declared_contribution_namespaces.clone()),
        })
        .collect();
    let source_provenance = node_vec
        .iter()
        .map(|n| (n.identity.clone(), n.source.clone()))
        .collect();
    let required_capabilities = caps.into_iter().collect();
    let contribution_namespaces = nss.into_iter().collect();
    diagnostics = sorted_diagnostics(diagnostics);
    let conflict_status = if diagnostics
        .iter()
        .any(|d| d.code == "ASSEMBLY_PROFILE_CONFLICT")
    {
        "failed"
    } else {
        "passed"
    }
    .into();
    let cap_status = if diagnostics
        .iter()
        .any(|d| d.code == "ASSEMBLY_UNSUPPORTED_CAPABILITY")
    {
        "failed"
    } else {
        "passed"
    }
    .into();
    let mut graph = ResolvedProfileGraphV1 {
        schema_version: PROFILE_GRAPH_SCHEMA_VERSION.into(),
        requested_root_profiles: roots.iter().map(|r| r.identity()).collect(),
        resolved_profile_nodes: node_vec,
        dependency_edges: edges,
        optional_dependency_decisions: optional,
        stable_topological_order: order,
        required_capabilities,
        contribution_namespaces,
        source_provenance,
        conflict_check: ValidationResultV1 {
            status: conflict_status,
            checked: Vec::new(),
        },
        capability_check: ValidationResultV1 {
            status: cap_status,
            checked: Vec::new(),
        },
        graph_hash: String::new(),
        diagnostics,
    };
    graph.graph_hash = profile_graph_hash(&graph);
    graph
}
fn visit(
    r: &ProfileReference,
    c: &ProfileCatalog,
    n: &mut BTreeMap<String, ProfileDefinition>,
    e: &mut Vec<ResolvedProfileEdgeV1>,
    o: &mut Vec<OptionalDependencyDecisionV1>,
    d: &mut Vec<AssemblyDiagnostic>,
    mut path: Vec<String>,
    from: &str,
) {
    let p = match resolve_ref(r, c, d) {
        Some(p) => p.clone(),
        None => return,
    };
    let id = p.identity();
    if path.contains(&id) {
        d.push(diag(
            "ASSEMBLY_PROFILE_DEPENDENCY_CYCLE",
            AssemblyStage::DependencyResolution,
            &id,
            &p.source.provenance,
            &format!("dependency cycle detected: {} -> {}", path.join(" -> "), id),
            "Break the profile dependency cycle.",
        ));
        return;
    }
    if n.contains_key(&id) {
        return;
    }
    path.push(id.clone());
    n.insert(id.clone(), p.clone());
    if from != "root" {}
    for depn in sorted_deps(p.dependencies.clone()) {
        let target = depn.reference.identity();
        if let Some(t) = resolve_ref(&depn.reference, c, d) {
            e.push(ResolvedProfileEdgeV1 {
                from: id.clone(),
                to: t.identity(),
                required: true,
                expected_content_hash: depn.expected_content_hash.clone(),
                required_contribution_namespaces: depn.required_contribution_namespaces.clone(),
                compatibility_policy: depn.compatibility_policy.clone(),
            });
            if depn
                .expected_content_hash
                .as_ref()
                .is_some_and(|h| h != &t.content_hash)
            {
                d.push(diag(
                    "ASSEMBLY_PROFILE_CONTENT_HASH_MISMATCH",
                    AssemblyStage::DependencyResolution,
                    &t.identity(),
                    &t.source.provenance,
                    "dependency content hash does not match pinned expectation",
                    "Update the pinned content hash or dependency.",
                ));
            }
            visit(&depn.reference, c, n, e, o, d, path.clone(), &id);
        } else {
            d.push(diag(
                "ASSEMBLY_PROFILE_DEPENDENCY_MISSING",
                AssemblyStage::DependencyResolution,
                &target,
                &p.source.provenance,
                "required dependency is missing",
                "Register the required pinned dependency in the profile catalog.",
            ));
        }
    }
    for depn in sorted_deps(p.optional_dependencies.clone()) {
        if let Some(t) = resolve_ref(&depn.reference, c, d) {
            e.push(ResolvedProfileEdgeV1 {
                from: id.clone(),
                to: t.identity(),
                required: false,
                expected_content_hash: depn.expected_content_hash.clone(),
                required_contribution_namespaces: depn.required_contribution_namespaces.clone(),
                compatibility_policy: depn.compatibility_policy.clone(),
            });
            o.push(OptionalDependencyDecisionV1 {
                from: id.clone(),
                target: t.identity(),
                selected: true,
                reason: "optional dependency present in pinned catalog".into(),
            });
            visit(&depn.reference, c, n, e, o, d, path.clone(), &id);
        } else {
            o.push(OptionalDependencyDecisionV1 {
                from: id.clone(),
                target: depn.reference.identity(),
                selected: false,
                reason: "optional dependency missing; declared policy permits omission".into(),
            });
        }
    }
}
fn resolve_ref<'a>(
    r: &ProfileReference,
    c: &'a ProfileCatalog,
    d: &mut Vec<AssemblyDiagnostic>,
) -> Option<&'a ProfileDefinition> {
    if r.version.is_some() {
        if let Some(p) = c.lookup_exact(r) {
            Some(p)
        } else {
            let code = if c.compatible(r).is_empty() {
                "ASSEMBLY_PROFILE_NOT_FOUND"
            } else {
                "ASSEMBLY_PROFILE_VERSION_NOT_FOUND"
            };
            d.push(diag(
                code,
                AssemblyStage::ProfileDiscovery,
                &r.identity(),
                "catalog",
                "profile reference could not be resolved exactly",
                "Register the exact pinned profile version.",
            ));
            None
        }
    } else {
        let m = c.compatible(r);
        if m.len() == 1 {
            Some(m[0])
        } else {
            d.push(diag(
                "ASSEMBLY_PROFILE_VERSION_AMBIGUOUS",
                AssemblyStage::DependencyResolution,
                &r.identity(),
                "catalog",
                "compatible profile reference did not resolve to exactly one version",
                "Pin an exact profile version.",
            ));
            None
        }
    }
}
fn topo(
    n: &BTreeMap<String, ProfileDefinition>,
    e: &Vec<ResolvedProfileEdgeV1>,
    d: &mut Vec<AssemblyDiagnostic>,
) -> Vec<String> {
    let mut indeg: BTreeMap<String, usize> = n.keys().map(|k| (k.clone(), 0)).collect();
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ed in e {
        if n.contains_key(&ed.from) && n.contains_key(&ed.to) {
            *indeg.get_mut(&ed.from).unwrap() += 1;
            adj.entry(ed.to.clone()).or_default().push(ed.from.clone());
        }
    }
    let mut ready: Vec<String> = indeg
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k.clone())
        .collect();
    ready.sort();
    let mut out = Vec::new();
    while let Some(x) = ready.first().cloned() {
        ready.remove(0);
        out.push(x.clone());
        if let Some(v) = adj.get(&x) {
            for y in v {
                let z = indeg.get_mut(y).unwrap();
                *z -= 1;
                if *z == 0 {
                    ready.push(y.clone());
                    ready.sort();
                }
            }
        }
    }
    if out.len() != n.len() {
        d.push(diag(
            "ASSEMBLY_PROFILE_DEPENDENCY_CYCLE",
            AssemblyStage::DependencyResolution,
            "profile-graph",
            "catalog",
            "dependency graph contains a cycle",
            "Break the dependency cycle.",
        ));
        for k in n.keys() {
            if !out.contains(k) {
                out.push(k.clone())
            }
        }
    }
    out
}
fn parse_profile_ref(s: &str) -> Option<ProfileReference> {
    let (left, version) = s
        .split_once('@')
        .map(|(a, b)| (a, Some(b.to_string())))
        .unwrap_or((s, None));
    let parts: Vec<_> = left.split('.').collect();
    if parts.len() < 3 {
        return legacy_ref(s);
    };
    Some(ProfileReference {
        provider: parts[0].into(),
        category: parse_cat(parts[1])?,
        name: parts[2..].join("."),
        version,
        compatible_version: None,
    })
}
fn legacy_ref(s: &str) -> Option<ProfileReference> {
    let name = s.strip_suffix("-v1").unwrap_or(s).to_string() + "-v1";
    Some(ref_for(
        "everarcade",
        if name.contains("topology") {
            ProfileCategory::Topology
        } else if name.contains("proof") {
            ProfileCategory::Proof
        } else {
            ProfileCategory::World
        },
        &name,
        "1.0.0",
    ))
}
fn parse_cat(s: &str) -> Option<ProfileCategory> {
    Some(match s {
        "world" => ProfileCategory::World,
        "runtime" => ProfileCategory::Runtime,
        "genre" => ProfileCategory::Genre,
        "topology" => ProfileCategory::Topology,
        "biome" => ProfileCategory::Biome,
        "encounter" => ProfileCategory::Encounter,
        "progression" => ProfileCategory::Progression,
        "economy" => ProfileCategory::Economy,
        "proof" => ProfileCategory::Proof,
        "projection" => ProfileCategory::Projection,
        "ai" => ProfileCategory::Ai,
        "access" => ProfileCategory::Access,
        "module" => ProfileCategory::Module,
        _ => return None,
    })
}
fn diag(
    code: &str,
    stage: AssemblyStage,
    id: &str,
    source: &str,
    msg: &str,
    rem: &str,
) -> AssemblyDiagnostic {
    AssemblyDiagnostic {
        code: code.into(),
        severity: DiagnosticSeverity::Error,
        stage,
        namespace: "profile_resolution".into(),
        source: source.into(),
        affected_id: id.into(),
        message: msg.into(),
        suggested_remediation: rem.into(),
    }
}
fn hash_json<T: Serialize>(v: &T) -> String {
    let b = serde_json::to_vec(v).expect("stable json");
    format!("sha256:{}", hex::encode(Sha256::digest(&b)))
}
fn profile_graph_hash(g: &ResolvedProfileGraphV1) -> String {
    hash_json(
        &json!({"domain":PROFILE_GRAPH_HASH_DOMAIN,"nodes":g.resolved_profile_nodes,"edges":g.dependency_edges,"optional":g.optional_dependency_decisions,"capabilities":g.required_capabilities,"namespaces":g.contribution_namespaces}),
    )
}
fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}
fn sorted_ns(mut v: Vec<ContributionNamespace>) -> Vec<ContributionNamespace> {
    v.sort();
    v
}
fn sorted_deps(mut v: Vec<ProfileDependency>) -> Vec<ProfileDependency> {
    v.sort_by_key(|d| d.reference.identity());
    v
}

fn assembly_stages() -> Vec<AssemblyStage> {
    vec![
        AssemblyStage::InputNormalization,
        AssemblyStage::ProfileDiscovery,
        AssemblyStage::DependencyResolution,
        AssemblyStage::ValidateProfileCapabilities,
        AssemblyStage::ValidateProfileNamespaces,
        AssemblyStage::PrepareContributionLoading,
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
        assert_eq!(assembled.manifest.stages.len(), 17);
        assert_eq!(
            assembled.ir.resolved_profile_graph.schema_version,
            PROFILE_GRAPH_SCHEMA_VERSION
        );
    }

    #[test]
    fn profile_resolver_resolves_transitive_dependencies_and_hashes() {
        let catalog = builtin_profile_catalog();
        let graph = resolve_profile_graph(
            vec!["everarcade.topology.simple-topology-v1@1.0.0".into()],
            &catalog,
        );
        assert!(graph.diagnostics.is_empty(), "{:?}", graph.diagnostics);
        assert_eq!(
            graph.stable_topological_order,
            vec![
                "everarcade.world.base-world-v1@1.0.0",
                "everarcade.runtime.movement-enabled-v1@1.0.0",
                "everarcade.topology.simple-topology-v1@1.0.0"
            ]
        );
        assert!(graph
            .resolved_profile_nodes
            .iter()
            .all(|n| n.content_hash.starts_with("sha256:")));
    }

    #[test]
    fn profile_resolver_is_order_independent() {
        let mut a = builtin_profile_catalog();
        let g1 = resolve_profile_graph(
            vec![
                "everarcade.proof.replay-proof-v1@1.0.0".into(),
                "everarcade.topology.simple-topology-v1@1.0.0".into(),
            ],
            &a,
        );
        let g2 = resolve_profile_graph(
            vec![
                "everarcade.topology.simple-topology-v1@1.0.0".into(),
                "everarcade.proof.replay-proof-v1@1.0.0".into(),
            ],
            &a,
        );
        assert_eq!(
            serde_json::to_vec(&g1).unwrap(),
            serde_json::to_vec(&g2).unwrap()
        );
        a.register(prof(
            ProfileCategory::Module,
            "zzz-extra-v1",
            vec![],
            vec![],
            vec![],
        ));
        let g3 = resolve_profile_graph(
            vec![
                "everarcade.proof.replay-proof-v1@1.0.0".into(),
                "everarcade.topology.simple-topology-v1@1.0.0".into(),
            ],
            &a,
        );
        assert_eq!(g1.graph_hash, g3.graph_hash);
    }

    #[test]
    fn profile_resolver_reports_failures() {
        let catalog = builtin_profile_catalog();
        let missing =
            resolve_profile_graph(vec!["everarcade.world.missing-v1@1.0.0".into()], &catalog);
        assert!(missing
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_PROFILE_NOT_FOUND"));
        let cycle = resolve_profile_graph(
            vec!["everarcade.module.dependency-cycle-a-v1@1.0.0".into()],
            &catalog,
        );
        assert!(cycle
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_PROFILE_DEPENDENCY_CYCLE"));
        let conflict = resolve_profile_graph(
            vec![
                "everarcade.module.conflict-a-v1@1.0.0".into(),
                "everarcade.module.conflict-b-v1@1.0.0".into(),
            ],
            &catalog,
        );
        assert!(conflict
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_PROFILE_CONFLICT"));
        let cap = resolve_profile_graph(
            vec!["everarcade.runtime.unsupported-capability-v1@1.0.0".into()],
            &catalog,
        );
        assert!(cap
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_UNSUPPORTED_CAPABILITY"));
    }

    #[test]
    fn optional_missing_dependency_is_recorded_without_error() {
        let catalog = builtin_profile_catalog();
        let graph = resolve_profile_graph(
            vec!["everarcade.world.optional-dependency-root-v1@1.0.0".into()],
            &catalog,
        );
        assert!(graph
            .optional_dependency_decisions
            .iter()
            .any(|d| !d.selected));
        assert!(!graph
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_PROFILE_DEPENDENCY_MISSING"));
    }

    #[test]
    fn graph_hash_changes_for_material_graph_changes_only() {
        let mut catalog = builtin_profile_catalog();
        let base = resolve_profile_graph(
            vec!["everarcade.runtime.movement-enabled-v1@1.0.0".into()],
            &catalog,
        );
        let mut p = catalog
            .lookup_exact(&ref_for(
                "everarcade",
                ProfileCategory::Runtime,
                "movement-enabled-v1",
                "1.0.0",
            ))
            .unwrap()
            .clone();
        p.content_hash =
            "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".into();
        catalog.register(p);
        let changed = resolve_profile_graph(
            vec!["everarcade.runtime.movement-enabled-v1@1.0.0".into()],
            &catalog,
        );
        assert_ne!(base.graph_hash, changed.graph_hash);
    }
}
