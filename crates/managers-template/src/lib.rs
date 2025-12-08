//! Type definitions for the template manager.
//! Uses schemars to generate JSON schema for the data structure.
//! This file is used to define the structure of the data that will be managed by the template manager.
pub mod dot;
pub use dot::{ColorConfig, DotConfig, NodeShapeConfig};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ID = String;

/// Organization structure optimized for graph visualization
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Organization {
    pub name: String,

    // Flat collections of entities (nodes in the graph)
    pub purposes: HashMap<ID, Purpose>,
    pub people: HashMap<ID, Person>,
    pub projects: HashMap<ID, Project>,
    pub production_systems: HashMap<ID, ProductionSystem>,
    pub property_items: HashMap<ID, PropertyItem>,

    // Relationships (edges in the graph)
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Purpose {
    pub id: ID,
    pub description: String,
    /// For visualization: color, priority level, etc.
    pub display: DisplayAttributes,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Person {
    pub id: ID,
    pub name: String,
    pub title: String,
    pub display: DisplayAttributes,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Project {
    pub id: ID,
    pub name: String,
    pub status: ProjectStatus,
    pub display: DisplayAttributes,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub enum ProjectStatus {
    Planning,
    Active,
    Completed,
    OnHold,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ProductionSystem {
    pub id: ID,
    pub name: String,
    pub status: SystemStatus,
    pub display: DisplayAttributes,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub enum SystemStatus {
    Operational,
    Maintenance,
    Degraded,
    Offline,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PropertyItem {
    pub id: ID,
    pub name: String,
    pub property_type: PropertyType,
    pub display: DisplayAttributes,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub enum PropertyType {
    Physical,
    Intellectual,
    Financial,
}

/// Visual attributes for DOT rendering
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DisplayAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_override: Option<String>,
}

impl Default for DisplayAttributes {
    fn default() -> Self {
        Self {
            color: None,
            shape: None,
            style: None,
            label_override: None,
        }
    }
}

/// Relationship following Subject-Predicate-Object pattern
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Relationship {
    pub subject_id: ID,
    pub subject_type: EntityType,
    pub predicate: RelationType,
    pub object_id: ID,
    pub object_type: EntityType,
    /// Visual attributes for edge rendering
    pub display: EdgeDisplayAttributes,
}

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
pub enum EntityType {
    Purpose,
    Person,
    Project,
    ProductionSystem,
    Property,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub enum RelationType {
    // Person relationships
    WorksOn,
    Manages,
    Leads,

    // Project relationships
    Serves,        // Project serves Purpose
    DependsOn,     // Project depends on Project
    Uses,          // Uses PropertyItem
    TransitionsTo, // Project transitions to ProductionSystem

    // Production relationships
    Maintains, // Person maintains ProductionSystem
    Requires,  // ProductionSystem requires PropertyItem
    Supports,  // ProductionSystem supports Purpose

    // Generic
    PartOf,
}

/// Visual attributes for DOT edge rendering
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct EdgeDisplayAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>, // "solid", "dashed", "dotted"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>, // Edge weight for layout
}

impl Default for EdgeDisplayAttributes {
    fn default() -> Self {
        Self {
            color: None,
            style: None,
            label: None,
            weight: None,
        }
    }
}

// Helper methods for DOT generation
impl Organization {
    /// Get all nodes grouped by type for subgraph generation
    pub fn nodes_by_type(&self) -> HashMap<EntityType, Vec<&str>> {
        let mut result: HashMap<EntityType, Vec<&str>> = HashMap::new();

        for id in self.purposes.keys() {
            result
                .entry(EntityType::Purpose)
                .or_default()
                .push(id.as_str());
        }
        for id in self.people.keys() {
            result
                .entry(EntityType::Person)
                .or_default()
                .push(id.as_str());
        }
        for id in self.projects.keys() {
            result
                .entry(EntityType::Project)
                .or_default()
                .push(id.as_str());
        }
        for id in self.production_systems.keys() {
            result
                .entry(EntityType::ProductionSystem)
                .or_default()
                .push(id.as_str());
        }
        for id in self.property_items.keys() {
            result
                .entry(EntityType::Property)
                .or_default()
                .push(id.as_str());
        }

        result
    }

    /// Get node label for DOT rendering
    pub fn get_node_label(&self, id: &str, entity_type: &EntityType) -> String {
        match entity_type {
            EntityType::Purpose => self
                .purposes
                .get(id)
                .and_then(|p| p.display.label_override.clone())
                .unwrap_or_else(|| {
                    self.purposes
                        .get(id)
                        .map(|p| p.description.clone())
                        .unwrap_or_default()
                }),
            EntityType::Person => self
                .people
                .get(id)
                .map(|p| format!("{}\n{}", p.name, p.title))
                .unwrap_or_default(),
            EntityType::Project => self
                .projects
                .get(id)
                .map(|p| format!("{}\n[{:?}]", p.name, p.status))
                .unwrap_or_default(),
            EntityType::ProductionSystem => self
                .production_systems
                .get(id)
                .map(|s| format!("{}\n[{:?}]", s.name, s.status))
                .unwrap_or_default(),
            EntityType::Property => self
                .property_items
                .get(id)
                .map(|i| i.name.clone())
                .unwrap_or_default(),
        }
    }
}
