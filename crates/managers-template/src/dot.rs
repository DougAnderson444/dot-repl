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
    /// Use template mode (matches managers_template.dot exactly)
    pub use_template_mode: bool,
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
            use_template_mode: false,
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

        if config.use_template_mode {
            writeln!(dot, "digraph Organization {{").unwrap();
            writeln!(dot, "    graph [").unwrap();
            writeln!(dot, "        newrank = true,").unwrap();
            writeln!(dot, "        nodesep = 0.3,").unwrap();
            writeln!(dot, "        ranksep = 0.5,").unwrap();
            writeln!(dot, "        splines = false").unwrap();
            writeln!(dot, "    ]").unwrap();
            writeln!(dot).unwrap();
            writeln!(dot, "    node [").unwrap();
            writeln!(dot, "        shape = box,").unwrap();
            writeln!(dot, "        style = filled,").unwrap();
            writeln!(dot, "        fillcolor = lightblue").unwrap();
            writeln!(dot, "    ]").unwrap();
            writeln!(dot).unwrap();
            writeln!(dot, "    edge [").unwrap();
            writeln!(dot, "        weight = 10").unwrap();
            writeln!(dot, "    ]").unwrap();
            writeln!(dot).unwrap();
        } else {
            writeln!(dot, "digraph organization {{").unwrap();
            writeln!(dot, "  rankdir={};", config.rankdir).unwrap();
            writeln!(dot, "  node [style=filled];").unwrap();
            writeln!(dot, "  label=\"{}\";", escape_dot(&self.name)).unwrap();
            writeln!(dot, "  labelloc=t;").unwrap();
            writeln!(dot, "  compound=true;").unwrap();
            writeln!(dot, "  newrank=true;").unwrap();
            writeln!(dot).unwrap();
        }

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
        // Set global graph properties
        writeln!(
            dot,
            "  graph [newrank=true, nodesep=0.3, ranksep=0.5, splines=false];"
        )
        .unwrap();
        writeln!(dot, "  edge [style=invis, weight=10];").unwrap();
        writeln!(dot).unwrap();

        // Top tier: Purpose (spans full width)
        writeln!(dot, "  // Purpose cluster").unwrap();
        writeln!(dot, "  subgraph cluster_purpose {{").unwrap();
        writeln!(dot, "    label=\"Purpose\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightgray;").unwrap();

        let purpose_ids: Vec<_> = self.purposes.keys().collect();
        for (id, purpose) in &self.purposes {
            write_purpose_node(dot, id, purpose, config);
        }

        // Chain purposes horizontally with invisible edges
        for i in 0..purpose_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(purpose_ids[i]),
                escape_dot(purpose_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Middle tier: People (left pillar)
        writeln!(dot, "  // People cluster").unwrap();
        writeln!(dot, "  subgraph cluster_people {{").unwrap();
        writeln!(dot, "    label=\"People\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightyellow;").unwrap();

        let people_ids: Vec<_> = self.people.keys().collect();
        for (id, person) in &self.people {
            write_person_node(dot, id, person, config);
        }

        // Chain people vertically with invisible edges
        for i in 0..people_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(people_ids[i]),
                escape_dot(people_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Middle tier: Projects (center top)
        writeln!(dot, "  // Projects cluster").unwrap();
        writeln!(dot, "  subgraph cluster_projects {{").unwrap();
        writeln!(dot, "    label=\"Projects\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightgreen;").unwrap();

        let project_ids: Vec<_> = self.projects.keys().collect();
        for (id, project) in &self.projects {
            write_project_node(dot, id, project, config);
        }

        // Chain projects vertically with invisible edges
        for i in 0..project_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(project_ids[i]),
                escape_dot(project_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Middle tier: Progress (right pillar)
        writeln!(dot, "  // Progress cluster").unwrap();
        writeln!(dot, "  subgraph cluster_progress {{").unwrap();
        writeln!(dot, "    label=\"Progress\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightpink;").unwrap();

        let metric_ids: Vec<_> = self.progress_metrics.keys().collect();
        for (id, metric) in &self.progress_metrics {
            write_progress_node(dot, id, metric, config);
        }

        // Chain metrics vertically with invisible edges
        for i in 0..metric_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(metric_ids[i]),
                escape_dot(metric_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Middle tier: Production (center bottom, below projects)
        writeln!(dot, "  // Production cluster").unwrap();
        writeln!(dot, "  subgraph cluster_production {{").unwrap();
        writeln!(dot, "    label=\"Production\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightcyan;").unwrap();

        let system_ids: Vec<_> = self.production_systems.keys().collect();
        for (id, system) in &self.production_systems {
            write_production_node(dot, id, system, config);
        }

        // Chain production systems vertically with invisible edges
        for i in 0..system_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(system_ids[i]),
                escape_dot(system_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Bottom tier: Property (spans full width)
        writeln!(dot, "  // Property cluster").unwrap();
        writeln!(dot, "  subgraph cluster_property {{").unwrap();
        writeln!(dot, "    label=\"Property\";").unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightgray;").unwrap();

        let property_ids: Vec<_> = self.property_items.keys().collect();
        for (id, item) in &self.property_items {
            write_property_node(dot, id, item, config);
        }

        // Chain property items horizontally with invisible edges
        for i in 0..property_ids.len().saturating_sub(1) {
            writeln!(
                dot,
                "    \"{}\" -> \"{}\";",
                escape_dot(property_ids[i]),
                escape_dot(property_ids[i + 1])
            )
            .unwrap();
        }
        writeln!(dot, "  }}").unwrap();
        writeln!(dot).unwrap();

        // Create rank=same subgraphs to align nodes horizontally
        writeln!(dot, "  // Horizontal alignment").unwrap();

        // Purpose row
        if !purpose_ids.is_empty() {
            write!(dot, "  {{ rank=same; ").unwrap();
            for id in &purpose_ids {
                write!(dot, "\"{}\"; ", escape_dot(id)).unwrap();
            }
            writeln!(dot, "}}").unwrap();
        }

        // Top middle row - align first node of each middle column
        if !people_ids.is_empty() || !project_ids.is_empty() || !metric_ids.is_empty() {
            write!(dot, "  {{ rank=same; ").unwrap();
            if !people_ids.is_empty() {
                write!(dot, "\"{}\"; ", escape_dot(people_ids[0])).unwrap();
            }
            if !project_ids.is_empty() {
                write!(dot, "\"{}\"; ", escape_dot(project_ids[0])).unwrap();
            }
            if !metric_ids.is_empty() {
                write!(dot, "\"{}\"; ", escape_dot(metric_ids[0])).unwrap();
            }
            writeln!(dot, "}}").unwrap();
        }

        // Property row
        if !property_ids.is_empty() {
            write!(dot, "  {{ rank=same; ").unwrap();
            for id in &property_ids {
                write!(dot, "\"{}\"; ", escape_dot(id)).unwrap();
            }
            writeln!(dot, "}}").unwrap();
        }
        writeln!(dot).unwrap();

        // Invisible edges to enforce vertical tier ordering
        writeln!(dot, "  // Vertical tier ordering").unwrap();

        // Purpose -> People/Projects
        if let Some(first_purpose) = purpose_ids.first() {
            if let Some(first_person) = people_ids.first() {
                writeln!(
                    dot,
                    "  \"{}\" -> \"{}\";",
                    escape_dot(first_purpose),
                    escape_dot(first_person)
                )
                .unwrap();
            } else if let Some(first_project) = project_ids.first() {
                writeln!(
                    dot,
                    "  \"{}\" -> \"{}\";",
                    escape_dot(first_purpose),
                    escape_dot(first_project)
                )
                .unwrap();
            }
        }

        // Projects -> Production
        if let (Some(last_project), Some(first_system)) = (project_ids.last(), system_ids.first()) {
            writeln!(
                dot,
                "  \"{}\" -> \"{}\";",
                escape_dot(last_project),
                escape_dot(first_system)
            )
            .unwrap();
        }

        // Middle tier -> Property
        if let Some(first_property) = property_ids.first() {
            if let Some(last_person) = people_ids.last() {
                writeln!(
                    dot,
                    "  \"{}\" -> \"{}\";",
                    escape_dot(last_person),
                    escape_dot(first_property)
                )
                .unwrap();
            } else if let Some(last_system) = system_ids.last() {
                writeln!(
                    dot,
                    "  \"{}\" -> \"{}\";",
                    escape_dot(last_system),
                    escape_dot(first_property)
                )
                .unwrap();
            }
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
    let label = if person.title.is_empty() {
        person.name.clone()
    } else {
        format!("{}\n{}", person.name, person.title)
    };

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
    let label = &item.name;
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

fn write_progress_node(dot: &mut String, id: &str, metric: &ProgressMetric, config: &DotConfig) {
    let label = &metric.name;

    writeln!(
        dot,
        "    \"{}\" [label=\"{}\", shape={}, fillcolor=\"{}\"];",
        escape_dot(id),
        escape_dot(label),
        config.node_shapes.person, // Use box shape like other nodes
        metric.display.color.as_deref().unwrap_or("lightblue")
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
