//! DOT graph generation for organization structures
use crate::*;
use std::fmt::Write;

/// Configuration for DOT graph generation
#[derive(Clone)]
pub struct DotConfig {
    /// Whether to group entities into subgraphs by type
    pub use_subgraphs: bool,
    /// Graph layout direction (LR, TB, BT, RL)
    pub rankdir: &'static str,
    /// Whether to show entity statuses in labels
    pub show_status: bool,
    /// Node shape by entity type
    pub node_shapes: NodeShapeConfig,
    /// Color scheme
    pub colors: ColorConfig,
    /// Use hierarchical layout
    pub use_hierarchical_layout: bool,
}

impl Default for DotConfig {
    fn default() -> Self {
        Self {
            use_subgraphs: true,
            rankdir: "TB",
            show_status: true,
            node_shapes: NodeShapeConfig::default(),
            colors: ColorConfig::default(),
            use_hierarchical_layout: true,
        }
    }
}

#[derive(Clone)]
pub struct NodeShapeConfig {
    pub purpose: &'static str,
    pub person: &'static str,
    pub project: &'static str,
    pub production: &'static str,
    pub property: &'static str,
}

impl Default for NodeShapeConfig {
    fn default() -> Self {
        Self {
            purpose: "ellipse",
            person: "box",
            project: "component",
            production: "cylinder",
            property: "folder",
        }
    }
}

#[derive(Clone)]
pub struct ColorConfig {
    pub purpose: &'static str,
    pub person: &'static str,
    pub project_planning: &'static str,
    pub project_active: &'static str,
    pub project_completed: &'static str,
    pub project_onhold: &'static str,
    pub production_operational: &'static str,
    pub production_maintenance: &'static str,
    pub production_degraded: &'static str,
    pub production_offline: &'static str,
    pub property: &'static str,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            purpose: "lightblue",
            person: "lightgreen",
            project_planning: "lightyellow",
            project_active: "lightcyan",
            project_completed: "lightgray",
            project_onhold: "orange",
            production_operational: "green",
            production_maintenance: "yellow",
            production_degraded: "orange",
            production_offline: "red",
            property: "wheat",
        }
    }
}

impl Organization {
    /// Generate a DOT graph representation with hierarchical layout
    pub fn to_dot(&self, config: &DotConfig) -> String {
        let mut dot = String::with_capacity(4096);

        writeln!(dot, "digraph organization {{").unwrap();
        writeln!(dot, "  rankdir={};", config.rankdir).unwrap();
        writeln!(dot, "  node [style=filled];").unwrap();
        writeln!(dot, "  label=\"{}\";", escape_dot(&self.name)).unwrap();
        writeln!(dot, "  labelloc=t;").unwrap();
        writeln!(dot, "  compound=true;").unwrap();
        writeln!(dot, "  newrank=true;").unwrap();
        writeln!(dot).unwrap();

        if config.use_hierarchical_layout {
            self.write_hierarchical_layout(&mut dot, config);
        } else if config.use_subgraphs {
            self.write_subgraphs(&mut dot, config);
        } else {
            self.write_flat_nodes(&mut dot, config);
        }

        writeln!(dot).unwrap();
        self.write_edges(&mut dot, config);

        writeln!(dot, "}}").unwrap();
        dot
    }

    fn write_hierarchical_layout(&self, dot: &mut String, config: &DotConfig) {
        // Top rank: Purpose (spans full width)
        writeln!(dot, "  // Top tier: Purpose").unwrap();
        writeln!(dot, "  subgraph cluster_purpose {{").unwrap();
        writeln!(dot, "    label=\"Purpose\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        writeln!(dot, "    rank=same;").unwrap();
        for (id, purpose) in &self.purposes {
            write_purpose_node(dot, id, purpose, config);
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Middle tier: People, Projects, Progress (side by side)
        writeln!(dot, "  // Middle tier: People, Projects").unwrap();

        // People subgraph (left)
        writeln!(dot, "  subgraph cluster_people {{").unwrap();
        writeln!(dot, "    label=\"People\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        for (id, person) in &self.people {
            write_person_node(dot, id, person, config);
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Projects subgraph (middle-top)
        writeln!(dot, "  subgraph cluster_projects {{").unwrap();
        writeln!(dot, "    label=\"Projects\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        for (id, project) in &self.projects {
            write_project_node(dot, id, project, config);
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Production subgraph (middle-bottom, below projects)
        writeln!(dot, "  subgraph cluster_production {{").unwrap();
        writeln!(dot, "    label=\"Production\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        for (id, system) in &self.production_systems {
            write_production_node(dot, id, system, config);
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Progress placeholder (we'll add this when you define Progress entities)
        // For now, we can create invisible nodes to force layout
        writeln!(dot, "  // Progress placeholder").unwrap();
        writeln!(dot, "  subgraph cluster_progress {{").unwrap();
        writeln!(dot, "    label=\"Progress\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        writeln!(dot, "    progress_node [label=\"Progress Reports\", shape=note, style=filled, fillcolor=lightyellow];").unwrap();
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Bottom tier: Property (spans full width)
        writeln!(dot, "  // Bottom tier: Property").unwrap();
        writeln!(dot, "  subgraph cluster_property {{").unwrap();
        writeln!(dot, "    label=\"Property\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    color=lightgrey;").unwrap();
        writeln!(dot, "    rank=same;").unwrap();
        for (id, item) in &self.property_items {
            write_property_node(dot, id, item, config);
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Invisible edges to enforce vertical ranking
        writeln!(dot, "  // Invisible edges to enforce tier ordering").unwrap();

        if let (Some(first_purpose_id), Some(first_person_id)) =
            (self.purposes.keys().next(), self.people.keys().next())
        {
            writeln!(
                dot,
                "  \"{}\" -> \"{}\" [style=invis, weight=10];",
                escape_dot(first_purpose_id),
                escape_dot(first_person_id)
            )
            .unwrap();
        }

        if let (Some(first_person_id), Some(first_property_id)) =
            (self.people.keys().next(), self.property_items.keys().next())
        {
            writeln!(
                dot,
                "  \"{}\" -> \"{}\" [style=invis, weight=10];",
                escape_dot(first_person_id),
                escape_dot(first_property_id)
            )
            .unwrap();
        }

        // Enforce project above production
        if let (Some(first_project_id), Some(first_system_id)) = (
            self.projects.keys().next(),
            self.production_systems.keys().next(),
        ) {
            writeln!(
                dot,
                "  \"{}\" -> \"{}\" [style=invis, weight=10];",
                escape_dot(first_project_id),
                escape_dot(first_system_id)
            )
            .unwrap();
        }
    }

    fn write_subgraphs(&self, dot: &mut String, config: &DotConfig) {
        // Purpose subgraph
        if !self.purposes.is_empty() {
            writeln!(dot, "  subgraph cluster_purpose {{").unwrap();
            writeln!(dot, "    label=\"Purpose\";").unwrap();
            writeln!(dot, "    style=dashed;").unwrap();
            for (id, purpose) in &self.purposes {
                write_purpose_node(dot, id, purpose, config);
            }
            writeln!(dot, "  }}").unwrap();
            writeln!(dot).unwrap();
        }

        // People subgraph
        if !self.people.is_empty() {
            writeln!(dot, "  subgraph cluster_people {{").unwrap();
            writeln!(dot, "    label=\"People\";").unwrap();
            writeln!(dot, "    style=dashed;").unwrap();
            for (id, person) in &self.people {
                write_person_node(dot, id, person, config);
            }
            writeln!(dot, "  }}").unwrap();
            writeln!(dot).unwrap();
        }

        // Projects subgraph
        if !self.projects.is_empty() {
            writeln!(dot, "  subgraph cluster_projects {{").unwrap();
            writeln!(dot, "    label=\"Projects\";").unwrap();
            writeln!(dot, "    style=dashed;").unwrap();
            for (id, project) in &self.projects {
                write_project_node(dot, id, project, config);
            }
            writeln!(dot, "  }}").unwrap();
            writeln!(dot).unwrap();
        }

        // Production systems subgraph
        if !self.production_systems.is_empty() {
            writeln!(dot, "  subgraph cluster_production {{").unwrap();
            writeln!(dot, "    label=\"Production\";").unwrap();
            writeln!(dot, "    style=dashed;").unwrap();
            for (id, system) in &self.production_systems {
                write_production_node(dot, id, system, config);
            }
            writeln!(dot, "  }}").unwrap();
            writeln!(dot).unwrap();
        }

        // Property subgraph
        if !self.property_items.is_empty() {
            writeln!(dot, "  subgraph cluster_property {{").unwrap();
            writeln!(dot, "    label=\"Property\";").unwrap();
            writeln!(dot, "    style=dashed;").unwrap();
            for (id, item) in &self.property_items {
                write_property_node(dot, id, item, config);
            }
            writeln!(dot, "  }}").unwrap();
            writeln!(dot).unwrap();
        }
    }

    fn write_flat_nodes(&self, dot: &mut String, config: &DotConfig) {
        for (id, purpose) in &self.purposes {
            write_purpose_node(dot, id, purpose, config);
        }
        for (id, person) in &self.people {
            write_person_node(dot, id, person, config);
        }
        for (id, project) in &self.projects {
            write_project_node(dot, id, project, config);
        }
        for (id, system) in &self.production_systems {
            write_production_node(dot, id, system, config);
        }
        for (id, item) in &self.property_items {
            write_property_node(dot, id, item, config);
        }
    }

    fn write_edges(&self, dot: &mut String, _config: &DotConfig) {
        for rel in &self.relationships {
            let edge_label = format!("{:?}", rel.predicate);
            let style = if let Some(s) = &rel.display.style {
                s.as_str()
            } else {
                "solid"
            };
            let color = rel.display.color.as_deref().unwrap_or("black");

            write!(
                dot,
                "  \"{}\" -> \"{}\" [label=\"{}\", style={}, color=\"{}\"",
                escape_dot(&rel.subject_id),
                escape_dot(&rel.object_id),
                escape_dot(&edge_label),
                style,
                color
            )
            .unwrap();

            if let Some(label) = &rel.display.label {
                write!(dot, ", xlabel=\"{}\"", escape_dot(label)).unwrap();
            }
            if let Some(weight) = rel.display.weight {
                write!(dot, ", weight={}", weight).unwrap();
            }

            writeln!(dot, "];").unwrap();
        }
    }
}

fn write_purpose_node(dot: &mut String, id: &str, purpose: &Purpose, config: &DotConfig) {
    let label = if let Some(override_label) = &purpose.display.label_override {
        override_label.clone()
    } else {
        truncate(&purpose.description, 30)
    };

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(&label),
        config.node_shapes.purpose,
        purpose
            .display
            .color
            .as_deref()
            .unwrap_or(config.colors.purpose)
    )
    .unwrap();
}

fn write_person_node(dot: &mut String, id: &str, person: &Person, config: &DotConfig) {
    let label = format!("{}\\n{}", person.name, person.title);

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(&label),
        config.node_shapes.person,
        person
            .display
            .color
            .as_deref()
            .unwrap_or(config.colors.person)
    )
    .unwrap();
}

fn write_project_node(dot: &mut String, id: &str, project: &Project, config: &DotConfig) {
    let label = if config.show_status {
        format!("{}\\n[{:?}]", project.name, project.status)
    } else {
        project.name.clone()
    };

    let color = project
        .display
        .color
        .as_deref()
        .unwrap_or_else(|| match project.status {
            ProjectStatus::Planning => config.colors.project_planning,
            ProjectStatus::Active => config.colors.project_active,
            ProjectStatus::Completed => config.colors.project_completed,
            ProjectStatus::OnHold => config.colors.project_onhold,
        });

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(&label),
        config.node_shapes.project,
        color
    )
    .unwrap();
}

fn write_production_node(
    dot: &mut String,
    id: &str,
    system: &ProductionSystem,
    config: &DotConfig,
) {
    let label = if config.show_status {
        format!("{}\\n[{:?}]", system.name, system.status)
    } else {
        system.name.clone()
    };

    let color = system
        .display
        .color
        .as_deref()
        .unwrap_or_else(|| match system.status {
            SystemStatus::Operational => config.colors.production_operational,
            SystemStatus::Maintenance => config.colors.production_maintenance,
            SystemStatus::Degraded => config.colors.production_degraded,
            SystemStatus::Offline => config.colors.production_offline,
        });

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(&label),
        config.node_shapes.production,
        color
    )
    .unwrap();
}

fn write_property_node(dot: &mut String, id: &str, item: &PropertyItem, config: &DotConfig) {
    let label = format!("{}\\n[{:?}]", item.name, item.property_type);

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(&label),
        config.node_shapes.property,
        item.display
            .color
            .as_deref()
            .unwrap_or(config.colors.property)
    )
    .unwrap();
}

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
