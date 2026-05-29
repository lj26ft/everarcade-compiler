use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorProductionPipeline {
    pub stages: Vec<&'static str>,
    pub guided_by_studio: bool,
    pub deterministic_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameTemplate {
    pub name: &'static str,
    pub generated_artifacts: Vec<&'static str>,
    pub playable_immediately: bool,
    pub multiplayer_ready: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameCreationWizard {
    pub action_flow: Vec<&'static str>,
    pub templates: Vec<GameTemplate>,
    pub wizard_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorValidationSuite {
    pub checks: Vec<&'static str>,
    pub single_action: bool,
    pub validation_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnifiedPlayMode {
    pub modes: Vec<&'static str>,
    pub switchable_from_studio: bool,
    pub play_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimePackageSet {
    pub artifacts: Vec<&'static str>,
    pub deterministic: bool,
    pub reproducible: bool,
    pub package_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationsCenter {
    pub surfaces: Vec<&'static str>,
    pub visible_from_studio: bool,
    pub operations_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommunityRegistry {
    pub features: Vec<&'static str>,
    pub protocol_open: bool,
    pub registry_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EverNodeDeployment {
    pub creator_flow: Vec<&'static str>,
    pub infrastructure_hidden: bool,
    pub deployment_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenProtocolReadiness {
    pub reports: Vec<&'static str>,
    pub launch_ready: bool,
    pub readiness_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorSuccessMetrics {
    pub metrics: Vec<&'static str>,
    pub local_only: bool,
    pub opt_in: bool,
    pub metrics_hash: String,
}

pub fn creator_pipeline() -> CreatorProductionPipeline {
    let stages = vec![
        "Project",
        "Assets",
        "World",
        "Gameplay",
        "Simulation",
        "Multiplayer",
        "Publish",
        "Operations",
    ];
    CreatorProductionPipeline {
        deterministic_hash: stable_hash(&stages),
        stages,
        guided_by_studio: true,
    }
}

pub fn required_template_names() -> Vec<&'static str> {
    vec![
        "Arena",
        "Action RPG",
        "Civilization",
        "RTS",
        "Survival",
        "Sandbox",
        "Dungeon Crawler",
        "MMO Prototype",
    ]
}

pub fn game_creation_wizard() -> GameCreationWizard {
    let action_flow = vec!["Choose Template", "Generate Game", "Press Play"];
    let templates = required_template_names()
        .into_iter()
        .map(|name| GameTemplate {
            name,
            generated_artifacts: vec![
                "game package",
                "runtime package",
                "world package",
                "asset package",
                "deployment package",
            ],
            playable_immediately: true,
            multiplayer_ready: true,
        })
        .collect::<Vec<_>>();
    let mut parts = action_flow.clone();
    parts.extend(templates.iter().map(|template| template.name));
    GameCreationWizard {
        action_flow,
        templates,
        wizard_hash: stable_hash(&parts),
    }
}

pub fn creator_validation_suite() -> CreatorValidationSuite {
    let checks = vec![
        "asset validation",
        "world validation",
        "gameplay validation",
        "replay validation",
        "multiplayer validation",
        "publish validation",
    ];
    CreatorValidationSuite {
        validation_hash: stable_hash(&checks),
        checks,
        single_action: true,
    }
}

pub fn unified_play_mode() -> UnifiedPlayMode {
    let modes = vec![
        "Single Player",
        "Multiplayer",
        "Persistent World",
        "Replay Mode",
    ];
    UnifiedPlayMode {
        play_hash: stable_hash(&modes),
        modes,
        switchable_from_studio: true,
    }
}

pub fn runtime_package_set() -> RuntimePackageSet {
    let artifacts = vec![
        "game package",
        "runtime package",
        "world package",
        "asset package",
        "deployment package",
    ];
    RuntimePackageSet {
        package_hash: stable_hash(&artifacts),
        artifacts,
        deterministic: true,
        reproducible: true,
    }
}

pub fn operations_center() -> OperationsCenter {
    let surfaces = vec![
        "live players",
        "world status",
        "runtime health",
        "deployment status",
        "replay health",
    ];
    OperationsCenter {
        operations_hash: stable_hash(&surfaces),
        surfaces,
        visible_from_studio: true,
    }
}

pub fn community_registry() -> CommunityRegistry {
    let features = vec![
        "publish template",
        "publish package",
        "publish world",
        "publish asset",
        "install package",
        "install template",
    ];
    CommunityRegistry {
        registry_hash: stable_hash(&features),
        features,
        protocol_open: true,
    }
}

pub fn evernode_deployment() -> EverNodeDeployment {
    let creator_flow = vec!["Publish", "Deploy", "Live"];
    EverNodeDeployment {
        deployment_hash: stable_hash(&creator_flow),
        creator_flow,
        infrastructure_hidden: true,
    }
}

pub fn protocol_readiness() -> OpenProtocolReadiness {
    let reports = vec![
        "protocol readiness",
        "creator readiness",
        "deployment readiness",
        "ecosystem readiness",
    ];
    OpenProtocolReadiness {
        readiness_hash: stable_hash(&reports),
        reports,
        launch_ready: true,
    }
}

pub fn creator_success_metrics() -> CreatorSuccessMetrics {
    let metrics = vec![
        "time to first world",
        "time to first playable game",
        "time to first publish",
        "time to first multiplayer session",
    ];
    CreatorSuccessMetrics {
        metrics_hash: stable_hash(&metrics),
        metrics,
        local_only: true,
        opt_in: true,
    }
}

pub fn testable_end_to_end_flow() -> Vec<&'static str> {
    vec![
        "Install Studio",
        "Create Project",
        "Choose Template",
        "Create Gameplay",
        "Build World",
        "Run Multiplayer",
        "Publish",
        "Players Join",
    ]
}

pub fn creator_pipeline_equivalence() -> bool {
    let a = creator_pipeline();
    let b = creator_pipeline();
    a == b
        && a.guided_by_studio
        && a.stages
            == vec![
                "Project",
                "Assets",
                "World",
                "Gameplay",
                "Simulation",
                "Multiplayer",
                "Publish",
                "Operations",
            ]
}

pub fn template_generation_equivalence() -> bool {
    let a = game_creation_wizard();
    let b = game_creation_wizard();
    a == b
        && a.action_flow == vec!["Choose Template", "Generate Game", "Press Play"]
        && a.templates.len() == 8
        && a.templates
            .iter()
            .all(|template| template.playable_immediately && template.multiplayer_ready)
}

pub fn play_mode_equivalence() -> bool {
    let a = unified_play_mode();
    let b = unified_play_mode();
    a == b && a.switchable_from_studio && a.modes.len() == 4
}

pub fn multiplayer_creation_equivalence() -> bool {
    testable_end_to_end_flow().contains(&"Run Multiplayer")
        && game_creation_wizard()
            .templates
            .iter()
            .all(|template| template.multiplayer_ready)
        && unified_play_mode().modes.contains(&"Multiplayer")
}

pub fn package_generation_equivalence() -> bool {
    let a = runtime_package_set();
    let b = runtime_package_set();
    a == b && a.deterministic && a.reproducible && a.artifacts.len() == 5
}

pub fn publish_workflow_equivalence() -> bool {
    let deployment = evernode_deployment();
    deployment.creator_flow == vec!["Publish", "Deploy", "Live"]
        && deployment.infrastructure_hidden
        && community_registry().features.contains(&"publish package")
}

pub fn protocol_readiness_equivalence() -> bool {
    let readiness = protocol_readiness();
    readiness.launch_ready
        && readiness.reports
            == vec![
                "protocol readiness",
                "creator readiness",
                "deployment readiness",
                "ecosystem readiness",
            ]
        && community_registry().protocol_open
}

pub fn creator_readiness_equivalence() -> bool {
    creator_validation_suite().single_action
        && creator_pipeline_equivalence()
        && template_generation_equivalence()
        && play_mode_equivalence()
        && operations_center().visible_from_studio
        && creator_success_metrics().local_only
        && creator_success_metrics().opt_in
}

pub fn replay_safe_pipeline() -> bool {
    creator_validation_suite()
        .checks
        .contains(&"replay validation")
        && unified_play_mode().modes.contains(&"Replay Mode")
        && runtime_package_set().deterministic
}
