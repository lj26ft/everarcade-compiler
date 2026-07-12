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
pub const CONTRIBUTION_SCHEMA_VERSION: &str = "everarcade.contribution.v1";
pub const CONTRIBUTION_GRAPH_SCHEMA_VERSION: &str = "everarcade.contribution-graph.v1";
pub const MERGED_CONTRIBUTIONS_SCHEMA_VERSION: &str = "everarcade.merged-contributions.v1";

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
    LoadTypedContributions,
    BuildContributionGraph,
    ValidateContributionOperations,
    MergeUniqueDeclarations,
    MergeKeyedUnions,
    ApplyBoundedAggregations,
    ResolveOrderedCompositions,
    ApplyAuthorizedOverrides,
    FinalizeMergedContributions,
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
            Self::LoadTypedContributions => "stage_06_load_typed_contributions",
            Self::BuildContributionGraph => "stage_07_build_contribution_graph",
            Self::ValidateContributionOperations => "stage_08_validate_contribution_operations",
            Self::MergeUniqueDeclarations => "stage_09_merge_unique_declarations",
            Self::MergeKeyedUnions => "stage_10_merge_keyed_unions",
            Self::ApplyBoundedAggregations => "stage_11_apply_bounded_aggregations",
            Self::ResolveOrderedCompositions => "stage_12_resolve_ordered_compositions",
            Self::ApplyAuthorizedOverrides => "stage_13_apply_authorized_overrides",
            Self::FinalizeMergedContributions => "stage_14_finalize_merged_contributions",
            Self::TypedContributionLoading => "stage_15_typed_contribution_loading",
            Self::DeterministicContributionMerge => "stage_16_deterministic_contribution_merge",
            Self::SymbolAndReferenceResolution => "stage_17_symbol_and_reference_resolution",
            Self::RuntimeIrConstruction => "stage_18_runtime_ir_construction",
            Self::StaticValidation => "stage_19_static_validation",
            Self::InvariantAndBoundAnalysis => "stage_20_invariant_and_bound_analysis",
            Self::CapabilityCompatibility => "stage_21_capability_compatibility",
            Self::CanonicalLowering => "stage_22_canonical_lowering",
            Self::PackageEmission => "stage_23_package_emission",
            Self::HashAndProofManifestGeneration => "stage_24_hash_and_proof_manifest_generation",
            Self::ConformanceSelfVerification => "stage_25_conformance_self_verification",
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
    EntityArchetypes,
    ItemArchetypes,
    EncounterArchetypes,
    Entities,
    WorldVariables,
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
            Self::EntityArchetypes => "entity_archetypes",
            Self::ItemArchetypes => "item_archetypes",
            Self::EncounterArchetypes => "encounter_archetypes",
            Self::Entities => "entities",
            Self::WorldVariables => "world_variables",
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ContributionOperation {
    Declare,
    Aggregate,
    Compose,
    Override,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum MergeClass {
    UniqueDeclaration,
    KeyedUnion,
    BoundedAggregation,
    OrderedComposition,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum OverridePolicy {
    Forbidden,
    SameProviderOnly,
    DeclaredExtensionPoint,
    RequestOverrideAllowed,
    OperatorPolicyRequired,
    ExactTargetHashRequired,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OverrideTargetV1 {
    pub target_contribution_id: String,
    pub target_declaration_key: String,
    pub expected_target_content_hash: String,
    pub policy_id: String,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderingMetadataV1 {
    pub order_key: String,
    #[serde(default)]
    pub after: Vec<String>,
    #[serde(default)]
    pub before: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContributionIdentityV1 {
    pub contribution_id: String,
    pub namespace: ContributionNamespace,
    pub declaration_key: String,
    pub schema_version: String,
    pub source_profile_id: String,
    pub source_profile_version: String,
    pub source_profile_content_hash: String,
    pub contribution_content_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceProvenanceV1 {
    pub profile_identity: String,
    pub profile_version: String,
    pub profile_content_hash: String,
    pub profile_source_kind: String,
    pub dependency_path: Vec<String>,
    pub namespace: ContributionNamespace,
    pub contribution_id: String,
    pub contribution_content_hash: String,
    pub requested_root_profile: String,
}
macro_rules! payload {
    ($n:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        pub struct $n {
            pub authoritative: bool,
            pub value: Value,
        }
    };
}
payload!(RuntimeContributionPayload);
payload!(LimitsContributionPayload);
payload!(TopologyContributionPayload);
payload!(RegionContributionPayload);
payload!(SpawnPointContributionPayload);
payload!(EntityArchetypeContributionPayload);
payload!(ItemArchetypeContributionPayload);
payload!(EncounterArchetypeContributionPayload);
payload!(EntityContributionPayload);
payload!(WorldVariableContributionPayload);
payload!(PrimitiveContributionPayload);
payload!(ActionContributionPayload);
payload!(TransitionContributionPayload);
payload!(InvariantContributionPayload);
payload!(EncounterContributionPayload);
payload!(ProgressionContributionPayload);
payload!(ContentContributionPayload);
payload!(ProjectionContributionPayload);
payload!(ProofContributionPayload);
payload!(EconomyContributionPayload);
payload!(AiContributionPayload);
payload!(AccessContributionPayload);
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "namespace", content = "payload", rename_all = "snake_case")]
pub enum ContributionPayloadV1 {
    Runtime(RuntimeContributionPayload),
    Limits(LimitsContributionPayload),
    Topology(TopologyContributionPayload),
    Regions(RegionContributionPayload),
    SpawnPoints(SpawnPointContributionPayload),
    EntityArchetypes(EntityArchetypeContributionPayload),
    ItemArchetypes(ItemArchetypeContributionPayload),
    EncounterArchetypes(EncounterArchetypeContributionPayload),
    Entities(EntityContributionPayload),
    WorldVariables(WorldVariableContributionPayload),
    Primitives(PrimitiveContributionPayload),
    Actions(ActionContributionPayload),
    Transitions(TransitionContributionPayload),
    Invariants(InvariantContributionPayload),
    Encounters(EncounterContributionPayload),
    Progression(ProgressionContributionPayload),
    Content(ContentContributionPayload),
    Projection(ProjectionContributionPayload),
    Proof(ProofContributionPayload),
    Economy(EconomyContributionPayload),
    Ai(AiContributionPayload),
    Access(AccessContributionPayload),
}
impl ContributionPayloadV1 {
    fn value(&self) -> &Value {
        match self {
            Self::Runtime(x) => &x.value,
            Self::Limits(x) => &x.value,
            Self::Topology(x) => &x.value,
            Self::Regions(x) => &x.value,
            Self::SpawnPoints(x) => &x.value,
            Self::EntityArchetypes(x) => &x.value,
            Self::ItemArchetypes(x) => &x.value,
            Self::EncounterArchetypes(x) => &x.value,
            Self::Entities(x) => &x.value,
            Self::WorldVariables(x) => &x.value,
            Self::Primitives(x) => &x.value,
            Self::Actions(x) => &x.value,
            Self::Transitions(x) => &x.value,
            Self::Invariants(x) => &x.value,
            Self::Encounters(x) => &x.value,
            Self::Progression(x) => &x.value,
            Self::Content(x) => &x.value,
            Self::Projection(x) => &x.value,
            Self::Proof(x) => &x.value,
            Self::Economy(x) => &x.value,
            Self::Ai(x) => &x.value,
            Self::Access(x) => &x.value,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileContributionV1 {
    pub identity: ContributionIdentityV1,
    pub operation: ContributionOperation,
    pub merge_class: MergeClass,
    pub override_policy: OverridePolicy,
    #[serde(default)]
    pub override_target: Option<OverrideTargetV1>,
    #[serde(default)]
    pub ordering: Option<OrderingMetadataV1>,
    pub payload: ContributionPayloadV1,
    pub provenance: SourceProvenanceV1,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContributionGraphV1 {
    pub schema_version: String,
    pub resolved_profile_nodes: Vec<ResolvedProfileNodeV1>,
    pub contribution_nodes: Vec<ProfileContributionV1>,
    pub profile_to_contribution_edges: Vec<(String, String)>,
    pub contribution_dependency_edges: Vec<(String, String)>,
    pub ordering_constraints: Vec<(String, String)>,
    pub override_relationships: Vec<(String, String)>,
    pub provenance: BTreeMap<String, SourceProvenanceV1>,
    pub contribution_graph_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MergeDecisionV1 {
    pub namespace: ContributionNamespace,
    pub declaration_key: String,
    pub contribution_ids: Vec<String>,
    pub decision: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MergedContributionSetV1 {
    pub schema_version: String,
    pub runtime: BTreeMap<String, Value>,
    pub limits: BTreeMap<String, Value>,
    pub topology: BTreeMap<String, Value>,
    pub regions: BTreeMap<String, Value>,
    pub spawn_points: BTreeMap<String, Value>,
    pub entity_archetypes: BTreeMap<String, Value>,
    pub item_archetypes: BTreeMap<String, Value>,
    pub encounter_archetypes: BTreeMap<String, Value>,
    pub entities: BTreeMap<String, Value>,
    pub world_variables: BTreeMap<String, Value>,
    pub primitives: BTreeMap<String, Value>,
    pub actions: BTreeMap<String, Value>,
    pub transitions: BTreeMap<String, Value>,
    pub invariants: BTreeMap<String, Value>,
    pub encounters: BTreeMap<String, Value>,
    pub progression: BTreeMap<String, Value>,
    pub content: BTreeMap<String, Value>,
    pub projection: BTreeMap<String, Value>,
    pub proof: BTreeMap<String, Value>,
    pub economy: BTreeMap<String, Value>,
    pub ai: BTreeMap<String, Value>,
    pub access: BTreeMap<String, Value>,
    pub applied_overrides: Vec<MergeDecisionV1>,
    pub deduplicated_identical_contributions: Vec<MergeDecisionV1>,
    pub aggregation_decisions: Vec<MergeDecisionV1>,
    pub ordering_decisions: Vec<MergeDecisionV1>,
    pub provenance_index: BTreeMap<String, Vec<SourceProvenanceV1>>,
    pub diagnostics: Vec<AssemblyDiagnostic>,
    pub merged_contribution_hash: String,
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
    pub entity_archetypes: Vec<TypedContribution<Value>>,
    pub item_archetypes: Vec<TypedContribution<Value>>,
    pub encounter_archetypes: Vec<TypedContribution<Value>>,
    pub entities: Vec<TypedContribution<Value>>,
    pub world_variables: Vec<TypedContribution<Value>>,
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
    pub contribution_graph: ContributionGraphV1,
    pub merged_contributions: MergedContributionSetV1,
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
    pub contribution_graph_hash: String,
    pub merged_contribution_hash: String,
    pub contribution_graph_schema_version: String,
    pub merged_contributions_schema_version: String,
    pub merge_diagnostics: Vec<AssemblyDiagnostic>,
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
    pub contribution_graph: ContributionGraphV1,
    pub merged_contributions: MergedContributionSetV1,
    pub ir: PtwRuntimeIrV1,
    pub diagnostics: Vec<AssemblyDiagnostic>,
    pub manifest: AssemblyManifestV1,
}

fn ns_of_payload(p: &ContributionPayloadV1) -> ContributionNamespace {
    match p {
        ContributionPayloadV1::Runtime(_) => ContributionNamespace::Runtime,
        ContributionPayloadV1::Limits(_) => ContributionNamespace::Limits,
        ContributionPayloadV1::Topology(_) => ContributionNamespace::Topology,
        ContributionPayloadV1::Regions(_) => ContributionNamespace::Regions,
        ContributionPayloadV1::SpawnPoints(_) => ContributionNamespace::SpawnPoints,
        ContributionPayloadV1::EntityArchetypes(_) => ContributionNamespace::EntityArchetypes,
        ContributionPayloadV1::ItemArchetypes(_) => ContributionNamespace::ItemArchetypes,
        ContributionPayloadV1::EncounterArchetypes(_) => ContributionNamespace::EncounterArchetypes,
        ContributionPayloadV1::Entities(_) => ContributionNamespace::Entities,
        ContributionPayloadV1::WorldVariables(_) => ContributionNamespace::WorldVariables,
        ContributionPayloadV1::Primitives(_) => ContributionNamespace::Primitives,
        ContributionPayloadV1::Actions(_) => ContributionNamespace::Actions,
        ContributionPayloadV1::Transitions(_) => ContributionNamespace::Transitions,
        ContributionPayloadV1::Invariants(_) => ContributionNamespace::Invariants,
        ContributionPayloadV1::Encounters(_) => ContributionNamespace::Encounters,
        ContributionPayloadV1::Progression(_) => ContributionNamespace::Progression,
        ContributionPayloadV1::Content(_) => ContributionNamespace::Content,
        ContributionPayloadV1::Projection(_) => ContributionNamespace::Projection,
        ContributionPayloadV1::Proof(_) => ContributionNamespace::Proof,
        ContributionPayloadV1::Economy(_) => ContributionNamespace::Economy,
        ContributionPayloadV1::Ai(_) => ContributionNamespace::Ai,
        ContributionPayloadV1::Access(_) => ContributionNamespace::Access,
    }
}
fn make_contribution(
    n: &ResolvedProfileNodeV1,
    ns: ContributionNamespace,
    key: &str,
    op: ContributionOperation,
    class: MergeClass,
    policy: OverridePolicy,
    value: Value,
    ordering: Option<OrderingMetadataV1>,
    target: Option<OverrideTargetV1>,
) -> ProfileContributionV1 {
    let payload = match ns {
        ContributionNamespace::Runtime => {
            ContributionPayloadV1::Runtime(RuntimeContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::Limits => ContributionPayloadV1::Limits(LimitsContributionPayload {
            authoritative: true,
            value,
        }),
        ContributionNamespace::Topology => {
            ContributionPayloadV1::Topology(TopologyContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::Regions => {
            ContributionPayloadV1::Regions(RegionContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::SpawnPoints => {
            ContributionPayloadV1::SpawnPoints(SpawnPointContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::Primitives => {
            ContributionPayloadV1::Primitives(PrimitiveContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::Actions => {
            ContributionPayloadV1::Actions(ActionContributionPayload {
                authoritative: true,
                value,
            })
        }
        ContributionNamespace::Proof => ContributionPayloadV1::Proof(ProofContributionPayload {
            authoritative: true,
            value,
        }),
        _ => ContributionPayloadV1::Content(ContentContributionPayload {
            authoritative: false,
            value,
        }),
    };
    let h = hash_json(&payload);
    let cid = format!("{}:{}:{}:{}", n.identity, ns.as_str(), key, h);
    let id = ContributionIdentityV1 {
        contribution_id: cid.clone(),
        namespace: ns.clone(),
        declaration_key: key.into(),
        schema_version: CONTRIBUTION_SCHEMA_VERSION.into(),
        source_profile_id: format!("{}.{}.{}", n.provider, n.category.as_str(), n.name),
        source_profile_version: n.version.clone(),
        source_profile_content_hash: n.content_hash.clone(),
        contribution_content_hash: h.clone(),
    };
    let prov = SourceProvenanceV1 {
        profile_identity: n.identity.clone(),
        profile_version: n.version.clone(),
        profile_content_hash: n.content_hash.clone(),
        profile_source_kind: n.source.source_type.clone(),
        dependency_path: vec![n.identity.clone()],
        namespace: ns,
        contribution_id: cid,
        contribution_content_hash: h,
        requested_root_profile: String::new(),
    };
    ProfileContributionV1 {
        identity: id,
        operation: op,
        merge_class: class,
        override_policy: policy,
        override_target: target,
        ordering,
        payload,
        provenance: prov,
    }
}
pub fn load_typed_contributions(g: &ResolvedProfileGraphV1) -> Vec<ProfileContributionV1> {
    let mut out = Vec::new();
    for n in &g.resolved_profile_nodes {
        match n.name.as_str() {
            "base-world-v1" => {
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Runtime,
                    "runtime-contract",
                    ContributionOperation::Declare,
                    MergeClass::UniqueDeclaration,
                    OverridePolicy::Forbidden,
                    json!({"contract":"ptw","numeric_model":"u64","tick_model":"deterministic"}),
                    None,
                    None,
                ));
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Limits,
                    "max_players",
                    ContributionOperation::Aggregate,
                    MergeClass::BoundedAggregation,
                    OverridePolicy::Forbidden,
                    json!({"rule":"strictest_maximum","value":64}),
                    None,
                    None,
                ));
            }
            "movement-enabled-v1" => {
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Primitives,
                    "movement",
                    ContributionOperation::Declare,
                    MergeClass::KeyedUnion,
                    OverridePolicy::Forbidden,
                    json!({"primitive":"movement","mode":"grid"}),
                    None,
                    None,
                ));
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Actions,
                    "entity_move",
                    ContributionOperation::Declare,
                    MergeClass::KeyedUnion,
                    OverridePolicy::SameProviderOnly,
                    json!({"action":"entity_move","cost":1}),
                    None,
                    None,
                ));
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Transitions,
                    "move-precondition:has_actor",
                    ContributionOperation::Compose,
                    MergeClass::OrderedComposition,
                    OverridePolicy::Forbidden,
                    json!({"precondition":"has_actor"}),
                    Some(OrderingMetadataV1 {
                        order_key: "010-has-actor".into(),
                        after: vec![],
                        before: vec![],
                    }),
                    None,
                ));
            }
            "simple-topology-v1" => {
                out.push(make_contribution(
                    n,
                    ContributionNamespace::Regions,
                    "origin",
                    ContributionOperation::Declare,
                    MergeClass::KeyedUnion,
                    OverridePolicy::Forbidden,
                    json!({"region":"origin"}),
                    None,
                    None,
                ));
                out.push(make_contribution(
                    n,
                    ContributionNamespace::SpawnPoints,
                    "default",
                    ContributionOperation::Declare,
                    MergeClass::KeyedUnion,
                    OverridePolicy::Forbidden,
                    json!({"spawn":"default","region":"origin"}),
                    None,
                    None,
                ));
            }
            "replay-proof-v1" => out.push(make_contribution(
                n,
                ContributionNamespace::Proof,
                "replay",
                ContributionOperation::Declare,
                MergeClass::KeyedUnion,
                OverridePolicy::Forbidden,
                json!({"proof":"replay"}),
                None,
                None,
            )),
            _ => {}
        }
    }
    out.sort_by_key(|c| {
        (
            c.identity.namespace.clone(),
            c.identity.declaration_key.clone(),
            c.identity.contribution_id.clone(),
        )
    });
    out
}
pub fn build_contribution_graph(
    g: &ResolvedProfileGraphV1,
    mut c: Vec<ProfileContributionV1>,
) -> ContributionGraphV1 {
    c.sort_by_key(|x| {
        (
            x.identity.namespace.clone(),
            x.identity.declaration_key.clone(),
            x.identity.contribution_id.clone(),
        )
    });
    let p2c = c
        .iter()
        .map(|x| {
            (
                x.provenance.profile_identity.clone(),
                x.identity.contribution_id.clone(),
            )
        })
        .collect();
    let ord = c
        .iter()
        .filter_map(|x| {
            x.ordering
                .as_ref()
                .map(|o| (x.identity.contribution_id.clone(), o.order_key.clone()))
        })
        .collect();
    let ov = c
        .iter()
        .filter_map(|x| {
            x.override_target.as_ref().map(|o| {
                (
                    x.identity.contribution_id.clone(),
                    o.target_contribution_id.clone(),
                )
            })
        })
        .collect();
    let prov = c
        .iter()
        .map(|x| (x.identity.contribution_id.clone(), x.provenance.clone()))
        .collect();
    let mut cg = ContributionGraphV1 {
        schema_version: CONTRIBUTION_GRAPH_SCHEMA_VERSION.into(),
        resolved_profile_nodes: g.resolved_profile_nodes.clone(),
        contribution_nodes: c,
        profile_to_contribution_edges: p2c,
        contribution_dependency_edges: vec![],
        ordering_constraints: ord,
        override_relationships: ov,
        provenance: prov,
        contribution_graph_hash: String::new(),
    };
    cg.contribution_graph_hash = hash_json(
        &json!({"domain":CONTRIBUTION_GRAPH_SCHEMA_VERSION,"nodes":cg.contribution_nodes,"p2c":cg.profile_to_contribution_edges,"dep":cg.contribution_dependency_edges,"order":cg.ordering_constraints,"overrides":cg.override_relationships}),
    );
    cg
}
fn map_for<'a>(
    m: &'a mut MergedContributionSetV1,
    ns: &ContributionNamespace,
) -> &'a mut BTreeMap<String, Value> {
    match ns {
        ContributionNamespace::Runtime => &mut m.runtime,
        ContributionNamespace::Limits => &mut m.limits,
        ContributionNamespace::Topology => &mut m.topology,
        ContributionNamespace::Regions => &mut m.regions,
        ContributionNamespace::SpawnPoints => &mut m.spawn_points,
        ContributionNamespace::EntityArchetypes => &mut m.entity_archetypes,
        ContributionNamespace::ItemArchetypes => &mut m.item_archetypes,
        ContributionNamespace::EncounterArchetypes => &mut m.encounter_archetypes,
        ContributionNamespace::Entities => &mut m.entities,
        ContributionNamespace::WorldVariables => &mut m.world_variables,
        ContributionNamespace::Primitives => &mut m.primitives,
        ContributionNamespace::Actions => &mut m.actions,
        ContributionNamespace::Transitions => &mut m.transitions,
        ContributionNamespace::Invariants => &mut m.invariants,
        ContributionNamespace::Encounters => &mut m.encounters,
        ContributionNamespace::Progression => &mut m.progression,
        ContributionNamespace::Content => &mut m.content,
        ContributionNamespace::Projection => &mut m.projection,
        ContributionNamespace::Proof => &mut m.proof,
        ContributionNamespace::Economy => &mut m.economy,
        ContributionNamespace::Ai => &mut m.ai,
        ContributionNamespace::Access => &mut m.access,
    }
}
pub fn merge_contribution_graph(cg: &ContributionGraphV1) -> MergedContributionSetV1 {
    let mut m = MergedContributionSetV1 {
        schema_version: MERGED_CONTRIBUTIONS_SCHEMA_VERSION.into(),
        ..Default::default()
    };
    for c in &cg.contribution_nodes {
        let ns = ns_of_payload(&c.payload);
        if ns != c.identity.namespace {
            m.diagnostics.push(contrib_diag(
                "ASSEMBLY_CONTRIBUTION_NAMESPACE_MISMATCH",
                AssemblyStage::ValidateContributionOperations,
                &c,
            ));
            continue;
        }
        if c.identity.schema_version != CONTRIBUTION_SCHEMA_VERSION {
            m.diagnostics.push(contrib_diag(
                "ASSEMBLY_CONTRIBUTION_SCHEMA_UNSUPPORTED",
                AssemblyStage::ValidateContributionOperations,
                &c,
            ));
            continue;
        }
        if c.operation == ContributionOperation::Override {
            continue;
        }
        let key = c.identity.declaration_key.clone();
        let existing = map_for(&mut m, &ns).get(&key).cloned();
        match (c.merge_class.clone(), existing) {
            (MergeClass::BoundedAggregation, Some(old)) => {
                let nv = c.payload.value()["value"].as_u64().unwrap_or(u64::MAX);
                let ov = old["value"].as_u64().unwrap_or(u64::MAX);
                let chosen = if nv < ov {
                    c.payload.value().clone()
                } else {
                    old
                };
                map_for(&mut m, &ns).insert(key.clone(), chosen);
                m.aggregation_decisions.push(MergeDecisionV1 {
                    namespace: ns,
                    declaration_key: key,
                    contribution_ids: vec![c.identity.contribution_id.clone()],
                    decision: "strictest_maximum".into(),
                });
            }
            (_, Some(old)) if old == *c.payload.value() => m
                .deduplicated_identical_contributions
                .push(MergeDecisionV1 {
                    namespace: ns,
                    declaration_key: key,
                    contribution_ids: vec![c.identity.contribution_id.clone()],
                    decision: "byte_identical_duplicate_deduplicated".into(),
                }),
            (_, Some(_)) => m.diagnostics.push(contrib_diag(
                if c.merge_class == MergeClass::UniqueDeclaration {
                    "ASSEMBLY_UNIQUE_DECLARATION_CONFLICT"
                } else {
                    "ASSEMBLY_CONTRIBUTION_CONFLICT"
                },
                AssemblyStage::MergeKeyedUnions,
                c,
            )),
            (_, None) => {
                map_for(&mut m, &ns).insert(key, c.payload.value().clone());
            }
        }
        m.provenance_index
            .entry(format!(
                "{}:{}",
                c.identity.namespace.as_str(),
                c.identity.declaration_key
            ))
            .or_default()
            .push(c.provenance.clone());
    }
    let mut h = m.clone();
    h.merged_contribution_hash = String::new();
    h.diagnostics = sorted_diagnostics(h.diagnostics);
    m.diagnostics = h.diagnostics.clone();
    m.merged_contribution_hash =
        hash_json(&json!({"domain":MERGED_CONTRIBUTIONS_SCHEMA_VERSION,"merged":h}));
    m
}
fn contrib_diag(code: &str, stage: AssemblyStage, c: &ProfileContributionV1) -> AssemblyDiagnostic {
    AssemblyDiagnostic{code:code.into(),severity:DiagnosticSeverity::Error,stage,namespace:c.identity.namespace.as_str().into(),source:c.provenance.profile_identity.clone(),affected_id:c.identity.declaration_key.clone(),message:format!("{} for contribution {} in namespace {} declaration {}",code,c.identity.contribution_id,c.identity.namespace.as_str(),c.identity.declaration_key),suggested_remediation:"Use explicit compatible declarations, deterministic ordering metadata, or an authorized override target.".into()}
}

pub fn assemble_world(request: CanonicalWorldRequest) -> Result<AssembledWorld, String> {
    let catalog = builtin_profile_catalog();
    let graph = resolve_profile_graph(request.profiles.values().cloned().collect(), &catalog);
    let loaded_contributions = load_typed_contributions(&graph);
    let contribution_graph = build_contribution_graph(&graph, loaded_contributions);
    let merged_contributions = merge_contribution_graph(&contribution_graph);
    let mut contributions = TypedContributionsV1::default();
    for c in &contribution_graph.contribution_nodes {
        let tc = TypedContribution {
            provenance: ContributionProvenance {
                source_profile: c.provenance.profile_identity.clone(),
                source_version: c.provenance.profile_version.clone(),
                source_content_hash: c.provenance.profile_content_hash.clone(),
                contribution_id: c.identity.contribution_id.clone(),
                priority_policy: format!("{:?}", c.merge_class),
                override_policy: format!("{:?}", c.override_policy),
            },
            value: c.payload.value().clone(),
        };
        match c.identity.namespace {
            ContributionNamespace::Runtime => contributions.runtime.push(tc),
            ContributionNamespace::Limits => contributions.limits.push(tc),
            ContributionNamespace::Topology => contributions.topology.push(tc),
            ContributionNamespace::Regions => contributions.regions.push(tc),
            ContributionNamespace::SpawnPoints => contributions.spawn_points.push(tc),
            ContributionNamespace::EntityArchetypes => contributions.entity_archetypes.push(tc),
            ContributionNamespace::ItemArchetypes => contributions.item_archetypes.push(tc),
            ContributionNamespace::EncounterArchetypes => {
                contributions.encounter_archetypes.push(tc)
            }
            ContributionNamespace::Entities => contributions.entities.push(tc),
            ContributionNamespace::WorldVariables => contributions.world_variables.push(tc),
            ContributionNamespace::Primitives => contributions.primitives.push(tc),
            ContributionNamespace::Actions => contributions.actions.push(tc),
            ContributionNamespace::Transitions => contributions.transitions.push(tc),
            ContributionNamespace::Invariants => contributions.invariants.push(tc),
            ContributionNamespace::Encounters => contributions.encounters.push(tc),
            ContributionNamespace::Progression => contributions.progression.push(tc),
            ContributionNamespace::Content => contributions.content.push(tc),
            ContributionNamespace::Projection => contributions.projection.push(tc),
            ContributionNamespace::Proof => contributions.proof.push(tc),
            ContributionNamespace::Economy => contributions.economy.push(tc),
            ContributionNamespace::Ai => contributions.ai.push(tc),
            ContributionNamespace::Access => contributions.access.push(tc),
        }
    }
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
        contribution_graph: contribution_graph.clone(),
        merged_contributions: merged_contributions.clone(),
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
    diagnostics.extend(merged_contributions.diagnostics.clone());
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
        contribution_graph_hash: contribution_graph.contribution_graph_hash.clone(),
        merged_contribution_hash: merged_contributions.merged_contribution_hash.clone(),
        contribution_graph_schema_version: CONTRIBUTION_GRAPH_SCHEMA_VERSION.into(),
        merged_contributions_schema_version: MERGED_CONTRIBUTIONS_SCHEMA_VERSION.into(),
        merge_diagnostics: merged_contributions.diagnostics.clone(),
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
        contribution_graph,
        merged_contributions,
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
        AssemblyStage::LoadTypedContributions,
        AssemblyStage::BuildContributionGraph,
        AssemblyStage::ValidateContributionOperations,
        AssemblyStage::MergeUniqueDeclarations,
        AssemblyStage::MergeKeyedUnions,
        AssemblyStage::ApplyBoundedAggregations,
        AssemblyStage::ResolveOrderedCompositions,
        AssemblyStage::ApplyAuthorizedOverrides,
        AssemblyStage::FinalizeMergedContributions,
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
        assert_eq!(assembled.manifest.stages.len(), 26);
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
    fn contribution_loading_builds_graph_and_hashes_deterministically() {
        let catalog = builtin_profile_catalog();
        let graph = resolve_profile_graph(
            vec!["everarcade.topology.simple-topology-v1@1.0.0".into()],
            &catalog,
        );
        let g1 = build_contribution_graph(&graph, load_typed_contributions(&graph));
        let g2 = build_contribution_graph(&graph, load_typed_contributions(&graph));
        assert_eq!(g1.contribution_graph_hash, g2.contribution_graph_hash);
        assert!(g1
            .contribution_nodes
            .iter()
            .any(|c| c.identity.namespace == ContributionNamespace::Regions));
        assert!(g1
            .provenance
            .values()
            .all(|p| !p.profile_identity.is_empty() && !p.contribution_content_hash.is_empty()));
    }

    #[test]
    fn merge_keyed_union_deduplicates_and_conflicts() {
        let catalog = builtin_profile_catalog();
        let graph = resolve_profile_graph(
            vec!["everarcade.runtime.movement-enabled-v1@1.0.0".into()],
            &catalog,
        );
        let mut nodes = load_typed_contributions(&graph);
        let first_action = nodes
            .iter()
            .find(|c| c.identity.namespace == ContributionNamespace::Actions)
            .unwrap()
            .clone();
        nodes.push(first_action.clone());
        let mut conflicting = first_action;
        conflicting.identity.contribution_id.push_str(":conflict");
        if let ContributionPayloadV1::Actions(p) = &mut conflicting.payload {
            p.value = json!({"action":"entity_move","cost":2});
        }
        nodes.push(conflicting);
        let merged = merge_contribution_graph(&build_contribution_graph(&graph, nodes));
        assert!(merged
            .deduplicated_identical_contributions
            .iter()
            .any(|d| d.decision == "byte_identical_duplicate_deduplicated"));
        assert!(merged
            .diagnostics
            .iter()
            .any(|d| d.code == "ASSEMBLY_CONTRIBUTION_CONFLICT"));
    }

    #[test]
    fn bounded_aggregation_uses_strictest_maximum_order_independently() {
        let catalog = builtin_profile_catalog();
        let graph = resolve_profile_graph(
            vec!["everarcade.world.base-world-v1@1.0.0".into()],
            &catalog,
        );
        let mut nodes = load_typed_contributions(&graph);
        let base = nodes
            .iter()
            .find(|c| c.identity.namespace == ContributionNamespace::Limits)
            .unwrap()
            .clone();
        let mut stricter = base.clone();
        stricter.identity.contribution_id.push_str(":strict");
        if let ContributionPayloadV1::Limits(p) = &mut stricter.payload {
            p.value = json!({"rule":"strictest_maximum","value":32});
        }
        nodes.push(stricter);
        let merged = merge_contribution_graph(&build_contribution_graph(&graph, nodes));
        assert_eq!(merged.limits["max_players"]["value"], json!(32));
        assert!(merged.merged_contribution_hash.starts_with("sha256:"));
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
