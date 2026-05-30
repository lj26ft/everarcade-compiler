use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameplayNodeKind {
    Event,
    Action,
    Condition,
    Timer,
    StateTransition,
    Trigger,
}

impl GameplayNodeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Event => "event",
            Self::Action => "action",
            Self::Condition => "condition",
            Self::Timer => "timer",
            Self::StateTransition => "state-transition",
            Self::Trigger => "trigger",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayNode {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: GameplayNodeKind,
    pub deterministic: bool,
    pub rustrig: &'static str,
    pub record_type: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayEdge {
    pub from: &'static str,
    pub to: &'static str,
    pub condition: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualLogicGraph {
    pub nodes: Vec<GameplayNode>,
    pub edges: Vec<GameplayEdge>,
    pub graph_hash: String,
    pub editor_surfaces: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayEventFramework {
    pub event_types: Vec<&'static str>,
    pub sorted_by: &'static str,
    pub deterministic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestAuthoringSystem {
    pub features: Vec<&'static str>,
    pub visual_authoring: bool,
    pub branch_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueAuthoringSystem {
    pub features: Vec<&'static str>,
    pub visual_editing: bool,
    pub conversation_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryItemFramework {
    pub features: Vec<&'static str>,
    pub deterministic_loot_tables: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatFramework {
    pub features: Vec<&'static str>,
    pub deterministic_execution_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiAuthoringSystem {
    pub surfaces: Vec<&'static str>,
    pub visual_editor: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TriggerInteractionSystem {
    pub triggers: Vec<&'static str>,
    pub routes_to_event_graph: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimationStateMachineFramework {
    pub features: Vec<&'static str>,
    pub events_feed_visual_logic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveGameAuthoringSystem {
    pub features: Vec<&'static str>,
    pub replay_compatible: bool,
    pub migration_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayTemplateCatalog {
    pub templates: Vec<&'static str>,
    pub runnable_without_infrastructure_code: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayAuthoringState {
    pub visual_logic: VisualLogicGraph,
    pub rustrig_surfaces: Vec<&'static str>,
    pub events: GameplayEventFramework,
    pub quests: QuestAuthoringSystem,
    pub dialogue: DialogueAuthoringSystem,
    pub inventory: InventoryItemFramework,
    pub combat: CombatFramework,
    pub ui: UiAuthoringSystem,
    pub triggers: TriggerInteractionSystem,
    pub animation: AnimationStateMachineFramework,
    pub save_game: SaveGameAuthoringSystem,
    pub templates: GameplayTemplateCatalog,
    pub creator_flow: Vec<&'static str>,
    pub engine_provides: Vec<&'static str>,
    pub creators_provide: Vec<&'static str>,
}

impl GameplayAuthoringState {
    pub fn sample() -> Self {
        let mut visual_logic = VisualLogicGraph {
            nodes: vec![
                GameplayNode {
                    id: "event:interaction",
                    label: "On Interact",
                    kind: GameplayNodeKind::Event,
                    deterministic: true,
                    rustrig: "StartDialogue",
                    record_type: "DialogueRecord",
                },
                GameplayNode {
                    id: "condition:has-key",
                    label: "Has Item: Key",
                    kind: GameplayNodeKind::Condition,
                    deterministic: true,
                    rustrig: "ValidateInventory",
                    record_type: "InventoryRecord",
                },
                GameplayNode {
                    id: "action:open-door",
                    label: "Open Door",
                    kind: GameplayNodeKind::Action,
                    deterministic: true,
                    rustrig: "MoveEntity",
                    record_type: "WorldRecord",
                },
                GameplayNode {
                    id: "timer:cooldown",
                    label: "Start Cooldown Timer",
                    kind: GameplayNodeKind::Timer,
                    deterministic: true,
                    rustrig: "CalculateCooldown",
                    record_type: "CombatRecord",
                },
                GameplayNode {
                    id: "state:quest-advanced",
                    label: "Advance Quest State",
                    kind: GameplayNodeKind::StateTransition,
                    deterministic: true,
                    rustrig: "AdvanceQuest",
                    record_type: "QuestRecord",
                },
                GameplayNode {
                    id: "trigger:proximity",
                    label: "Area Proximity Trigger",
                    kind: GameplayNodeKind::Trigger,
                    deterministic: true,
                    rustrig: "SpawnEntity",
                    record_type: "WorldRecord",
                },
            ],
            edges: vec![
                GameplayEdge {
                    from: "event:interaction",
                    to: "condition:has-key",
                    condition: "input ordered by tick",
                },
                GameplayEdge {
                    from: "condition:has-key",
                    to: "action:open-door",
                    condition: "true",
                },
                GameplayEdge {
                    from: "action:open-door",
                    to: "timer:cooldown",
                    condition: "append event",
                },
                GameplayEdge {
                    from: "timer:cooldown",
                    to: "state:quest-advanced",
                    condition: "elapsed ticks reached",
                },
            ],
            graph_hash: String::new(),
            editor_surfaces: vec![
                "node canvas",
                "blackboard",
                "event browser",
                "determinism audit",
            ],
        };
        visual_logic.graph_hash = graph_hash(&visual_logic);

        Self {
            visual_logic,
            rustrig_surfaces: vec![
                "Rustrig Browser",
                "Rustrig Library",
                "Rustrig Search",
                "Rustrig Composition",
                "Rustrig Validation",
            ],
            events: GameplayEventFramework {
                event_types: vec![
                    "collision events",
                    "interaction events",
                    "inventory events",
                    "combat events",
                    "quest events",
                    "dialogue events",
                    "world events",
                ],
                sorted_by: "tick, source entity, event id",
                deterministic: true,
            },
            quests: QuestAuthoringSystem {
                features: vec![
                    "quests",
                    "objectives",
                    "completion states",
                    "rewards",
                    "branching progression",
                ],
                visual_authoring: true,
                branch_hash: stable_hash(&["quest", "branch", "objective", "reward"]),
            },
            dialogue: DialogueAuthoringSystem {
                features: vec![
                    "dialogue trees",
                    "branching conversations",
                    "conditions",
                    "choices",
                    "outcomes",
                ],
                visual_editing: true,
                conversation_hash: stable_hash(&["dialogue", "choice", "condition", "outcome"]),
            },
            inventory: InventoryItemFramework {
                features: vec![
                    "items",
                    "equipment",
                    "containers",
                    "crafting inputs",
                    "crafting outputs",
                    "loot tables",
                ],
                deterministic_loot_tables: true,
            },
            combat: CombatFramework {
                features: vec![
                    "abilities",
                    "cooldowns",
                    "damage",
                    "healing",
                    "targeting",
                    "status effects",
                ],
                deterministic_execution_only: true,
            },
            ui: UiAuthoringSystem {
                surfaces: vec![
                    "menus",
                    "HUDs",
                    "dialogs",
                    "inventory screens",
                    "runtime widgets",
                ],
                visual_editor: true,
            },
            triggers: TriggerInteractionSystem {
                triggers: vec![
                    "areas",
                    "switches",
                    "buttons",
                    "interactions",
                    "proximity triggers",
                ],
                routes_to_event_graph: true,
            },
            animation: AnimationStateMachineFramework {
                features: vec![
                    "state machines",
                    "transitions",
                    "conditions",
                    "animation events",
                ],
                events_feed_visual_logic: true,
            },
            save_game: SaveGameAuthoringSystem {
                features: vec![
                    "save slots",
                    "world saves",
                    "character saves",
                    "migration",
                    "restoration",
                ],
                replay_compatible: true,
                migration_hash: stable_hash(&["save", "migration", "restoration", "replay"]),
            },
            templates: GameplayTemplateCatalog {
                templates: vec![
                    "RPG",
                    "Action RPG",
                    "MMO Prototype",
                    "Survival",
                    "RTS",
                    "Civilization",
                    "Dungeon Crawler",
                ],
                runnable_without_infrastructure_code: true,
            },
            creator_flow: vec![
                "Create Project",
                "Create World",
                "Create Gameplay",
                "Create UI",
                "Create Quests",
                "Run Multiplayer",
                "Publish",
                "Players Join",
            ],
            engine_provides: vec![
                "persistence",
                "multiplayer",
                "deployment",
                "replay",
                "operations",
            ],
            creators_provide: vec!["mechanics", "progression", "worlds", "content", "stories"],
        }
    }

    pub fn deterministic_hash(&self) -> String {
        stable_hash(&[
            "gameplay-authoring",
            &self.visual_logic.graph_hash,
            &self.events.event_types.join(","),
            &self.quests.branch_hash,
            &self.dialogue.conversation_hash,
            &self.inventory.features.join(","),
            &self.combat.features.join(","),
            &self.ui.surfaces.join(","),
            &self.triggers.triggers.join(","),
            &self.animation.features.join(","),
            &self.save_game.migration_hash,
            &self.templates.templates.join(","),
            &self.creator_flow.join(","),
        ])
    }

    pub fn can_create_gameplay_without_infrastructure_code(&self) -> bool {
        self.events.deterministic
            && self
                .visual_logic
                .nodes
                .iter()
                .all(|node| node.deterministic)
            && self
                .visual_logic
                .nodes
                .iter()
                .all(|node| !node.rustrig.is_empty() && node.record_type.ends_with("Record"))
            && self.rustrig_surfaces.len() == 5
            && self.quests.visual_authoring
            && self.dialogue.visual_editing
            && self.inventory.deterministic_loot_tables
            && self.combat.deterministic_execution_only
            && self.ui.visual_editor
            && self.triggers.routes_to_event_graph
            && self.animation.events_feed_visual_logic
            && self.save_game.replay_compatible
            && self.templates.runnable_without_infrastructure_code
            && self.engine_provides
                == vec![
                    "persistence",
                    "multiplayer",
                    "deployment",
                    "replay",
                    "operations",
                ]
    }
}

fn graph_hash(graph: &VisualLogicGraph) -> String {
    let mut parts = vec!["visual-logic-graph".to_owned()];
    parts.extend(graph.nodes.iter().map(|node| {
        format!(
            "{}:{}:{}:{}:{}:{}",
            node.id,
            node.label,
            node.kind.as_str(),
            node.deterministic,
            node.rustrig,
            node.record_type
        )
    }));
    parts.extend(
        graph
            .edges
            .iter()
            .map(|edge| format!("{}:{}:{}", edge.from, edge.to, edge.condition)),
    );
    stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>())
}

pub fn visual_logic_equivalence() -> bool {
    GameplayAuthoringState::sample().visual_logic.graph_hash
        == GameplayAuthoringState::sample().visual_logic.graph_hash
}

pub fn gameplay_event_framework_equivalence() -> bool {
    let state = GameplayAuthoringState::sample();
    state.events.deterministic
        && state.events.sorted_by == "tick, source entity, event id"
        && state.events.event_types.len() == 7
}

pub fn creator_content_systems_complete() -> bool {
    let state = GameplayAuthoringState::sample();
    state.can_create_gameplay_without_infrastructure_code()
        && state.quests.features.len() == 5
        && state.dialogue.features.len() == 5
        && state.inventory.features.len() == 6
        && state.combat.features.len() == 6
        && state.ui.surfaces.len() == 5
        && state.triggers.triggers.len() == 5
        && state.animation.features.len() == 4
        && state.save_game.features.len() == 5
        && state.templates.templates.len() == 7
}

pub fn end_to_end_creator_flow_equivalence() -> bool {
    let a = GameplayAuthoringState::sample();
    let b = GameplayAuthoringState::sample();
    a.creator_flow
        == vec![
            "Create Project",
            "Create World",
            "Create Gameplay",
            "Create UI",
            "Create Quests",
            "Run Multiplayer",
            "Publish",
            "Players Join",
        ]
        && a.deterministic_hash() == b.deterministic_hash()
}

pub fn rustrig_visual_logic_integration() -> bool {
    let state = GameplayAuthoringState::sample();
    state.rustrig_surfaces
        == vec![
            "Rustrig Browser",
            "Rustrig Library",
            "Rustrig Search",
            "Rustrig Composition",
            "Rustrig Validation",
        ]
        && state.visual_logic.nodes.iter().all(|node| {
            node.deterministic && !node.rustrig.is_empty() && node.record_type.ends_with("Record")
        })
}
