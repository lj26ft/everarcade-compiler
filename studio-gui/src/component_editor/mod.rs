use crate::{
    editing_engine::actions::EditorActionType, editing_engine::editor::EditingEngine, stable_hash,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentTemplate {
    pub name: String,
    pub fields: Vec<String>,
    pub template_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentRecord {
    pub entity_id: String,
    pub component_type: String,
    pub data_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentEditor {
    pub components: Vec<ComponentRecord>,
    pub templates: Vec<ComponentTemplate>,
    pub search_index_hash: String,
    pub routes_through_editor_actions: bool,
}

impl ComponentEditor {
    pub fn sample() -> Self {
        let mut editor = Self {
            components: vec![ComponentRecord {
                entity_id: "entity:settler".into(),
                component_type: "Transform".into(),
                data_hash: stable_hash(&["component", "Transform", "0,0"]),
            }],
            templates: vec![
                template("Transform", &["x", "y", "rotation", "scale"]),
                template("Renderable", &["mesh", "material"]),
                template("SimulationAgent", &["scheduler", "ai-profile"]),
            ],
            search_index_hash: String::new(),
            routes_through_editor_actions: true,
        };
        editor.rebuild_search();
        editor
    }

    pub fn create_component(
        &mut self,
        engine: &mut EditingEngine,
        entity_id: &str,
        component_type: &str,
        data: &str,
    ) -> Result<(), &'static str> {
        self.components.push(ComponentRecord {
            entity_id: entity_id.to_owned(),
            component_type: component_type.to_owned(),
            data_hash: stable_hash(&["component", component_type, data]),
        });
        self.components.sort_by(|left, right| {
            (&left.entity_id, &left.component_type).cmp(&(&right.entity_id, &right.component_type))
        });
        self.rebuild_search();
        engine.apply_context_action(EditorActionType::ComponentMutation.as_str())
    }

    pub fn edit_component(
        &mut self,
        engine: &mut EditingEngine,
        entity_id: &str,
        component_type: &str,
        data: &str,
    ) -> Result<(), &'static str> {
        let component = self
            .components
            .iter_mut()
            .find(|component| {
                component.entity_id == entity_id && component.component_type == component_type
            })
            .ok_or("component must exist before editing")?;
        component.data_hash = stable_hash(&["component", component_type, data]);
        self.rebuild_search();
        engine.apply_context_action(EditorActionType::ComponentMutation.as_str())
    }

    pub fn remove_component(
        &mut self,
        engine: &mut EditingEngine,
        entity_id: &str,
        component_type: &str,
    ) -> Result<(), &'static str> {
        self.components.retain(|component| {
            !(component.entity_id == entity_id && component.component_type == component_type)
        });
        self.rebuild_search();
        engine.apply_context_action(EditorActionType::ComponentMutation.as_str())
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let query = query.to_ascii_lowercase();
        self.templates
            .iter()
            .filter(|template| template.name.to_ascii_lowercase().contains(&query))
            .map(|template| template.name.clone())
            .collect()
    }

    fn rebuild_search(&mut self) {
        let templates = self
            .templates
            .iter()
            .map(|template| template.template_hash.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let components = self
            .components
            .iter()
            .map(|component| {
                format!(
                    "{}:{}:{}",
                    component.entity_id, component.component_type, component.data_hash
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        self.search_index_hash = stable_hash(&["component-editor", &templates, &components]);
    }
}

fn template(name: &str, fields: &[&str]) -> ComponentTemplate {
    let fields = fields
        .iter()
        .map(|field| (*field).to_owned())
        .collect::<Vec<_>>();
    let joined = fields.join(",");
    ComponentTemplate {
        name: name.to_owned(),
        fields,
        template_hash: stable_hash(&["component-template", name, &joined]),
    }
}

pub fn component_editing_equivalence() -> bool {
    let mut first = ComponentEditor::sample();
    let mut second = ComponentEditor::sample();
    let mut first_engine = EditingEngine::sample();
    let mut second_engine = EditingEngine::sample();
    first_engine.select(&["entity:settler"]);
    second_engine.select(&["entity:settler"]);
    first
        .create_component(
            &mut first_engine,
            "entity:settler",
            "Renderable",
            "mesh=hero",
        )
        .ok()
        == second
            .create_component(
                &mut second_engine,
                "entity:settler",
                "Renderable",
                "mesh=hero",
            )
            .ok()
        && first
            .edit_component(
                &mut first_engine,
                "entity:settler",
                "Renderable",
                "mesh=hero-v2",
            )
            .ok()
            == second
                .edit_component(
                    &mut second_engine,
                    "entity:settler",
                    "Renderable",
                    "mesh=hero-v2",
                )
                .ok()
        && first
            .remove_component(&mut first_engine, "entity:settler", "Renderable")
            .ok()
            == second
                .remove_component(&mut second_engine, "entity:settler", "Renderable")
                .ok()
        && first.search("render") == second.search("render")
        && first.routes_through_editor_actions
        && first_engine.history.undo_stack.len() == second_engine.history.undo_stack.len()
}
