//! An example demonstrating how to generate a DOT representation
//! of an organization using the `managers-template` crate.
//!
//! ```sh
//! cargo run --example dot_generation --package managers-template -- --nocapture
//! ```
use managers_template::*;
use std::collections::HashMap;

fn main() {
    let purpose_id = "purpose_1".to_string();
    let person_id = "alice".to_string();
    let project_x_id = "project_x".to_string();
    let project_y_id = "project_y".to_string();
    let system_a_id = "system_a".to_string();
    let system_b_id = "system_b".to_string();
    let property_1_id = "property_1".to_string();

    let mut purposes = HashMap::new();
    purposes.insert(
        purpose_id.clone(),
        Purpose {
            id: purpose_id.clone(),
            description: "Provide quality products and services".to_string(),
            display: DisplayAttributes {
                color: Some("lightblue".to_string()),
                shape: Some("ellipse".to_string()),
                ..Default::default()
            },
        },
    );

    let mut people = HashMap::new();
    people.insert(
        person_id.clone(),
        Person {
            id: person_id.clone(),
            name: "Alice".to_string(),
            title: "Founder".to_string(),
            display: DisplayAttributes {
                color: Some("lightgreen".to_string()),
                ..Default::default()
            },
        },
    );

    let mut projects = HashMap::new();
    projects.insert(
        project_x_id.clone(),
        Project {
            id: project_x_id.clone(),
            name: "Project X".to_string(),
            status: ProjectStatus::Active,
            display: DisplayAttributes::default(),
        },
    );
    projects.insert(
        project_y_id.clone(),
        Project {
            id: project_y_id.clone(),
            name: "Project Y".to_string(),
            status: ProjectStatus::OnHold,
            display: DisplayAttributes {
                color: Some("orange".to_string()),
                ..Default::default()
            },
        },
    );

    let mut production_systems = HashMap::new();
    production_systems.insert(
        system_a_id.clone(),
        ProductionSystem {
            id: system_a_id.clone(),
            name: "System A".to_string(),
            status: SystemStatus::Operational,
            display: DisplayAttributes {
                color: Some("green".to_string()),
                ..Default::default()
            },
        },
    );
    production_systems.insert(
        system_b_id.clone(),
        ProductionSystem {
            id: system_b_id.clone(),
            name: "System B".to_string(),
            status: SystemStatus::Maintenance,
            display: DisplayAttributes {
                color: Some("yellow".to_string()),
                ..Default::default()
            },
        },
    );

    let mut property_items = HashMap::new();
    property_items.insert(
        property_1_id.clone(),
        PropertyItem {
            id: property_1_id.clone(),
            name: "Office Building".to_string(),
            property_type: PropertyType::Physical,
            display: DisplayAttributes::default(),
        },
    );

    let relationships = vec![
        // Alice works on Project X
        Relationship {
            subject_id: person_id.clone(),
            subject_type: EntityType::Person,
            predicate: RelationType::WorksOn,
            object_id: project_x_id.clone(),
            object_type: EntityType::Project,
            display: EdgeDisplayAttributes::default(),
        },
        // Project X serves Purpose
        Relationship {
            subject_id: project_x_id.clone(),
            subject_type: EntityType::Project,
            predicate: RelationType::Serves,
            object_id: purpose_id.clone(),
            object_type: EntityType::Purpose,
            display: EdgeDisplayAttributes {
                style: Some("dashed".to_string()),
                ..Default::default()
            },
        },
        // Project X transitions to System A
        Relationship {
            subject_id: project_x_id.clone(),
            subject_type: EntityType::Project,
            predicate: RelationType::TransitionsTo,
            object_id: system_a_id.clone(),
            object_type: EntityType::ProductionSystem,
            display: EdgeDisplayAttributes {
                color: Some("blue".to_string()),
                label: Some("deploys".to_string()),
                ..Default::default()
            },
        },
    ];

    let org = Organization {
        name: "Acme Corp".to_string(),
        purposes,
        people,
        projects,
        production_systems,
        property_items,
        relationships,
    };

    let config = DotConfig {
        use_hierarchical_layout: true,
        rankdir: "TB",
        show_status: true,
        ..Default::default()
    };

    let dot_string = org.to_dot(&config);
    println!("{}", dot_string);

    // Now pass dot_string to graphviz to generate SVG
    // Then pass that SVG to your Dioxus GraphvizSvg component
}
