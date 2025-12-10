//! An example demonstrating how to generate a DOT representation
//! of an organization using the `managers-template` crate.
//!
//! ```sh
//! cargo run --example dot_generation --package managers-template -- --nocapture
//! ```
use managers_template::*;
use std::collections::HashMap;

fn main() {
    // IDs matching the target template structure
    let mission_id = "mission".to_string();
    let vision_id = "vision".to_string();
    let values_id = "values".to_string();
    
    let person1_id = "person1".to_string();
    let person2_id = "person2".to_string();
    let person3_id = "person3".to_string();
    
    let project1_id = "project1".to_string();
    let project2_id = "project2".to_string();
    
    let metric1_id = "metric1".to_string();
    let metric2_id = "metric2".to_string();
    let metric3_id = "metric3".to_string();
    
    let prod1_id = "prod1".to_string();
    let prod2_id = "prod2".to_string();
    
    let asset1_id = "asset1".to_string();
    let asset2_id = "asset2".to_string();
    let asset3_id = "asset3".to_string();

    let mut purposes = HashMap::new();
    purposes.insert(
        mission_id.clone(),
        Purpose {
            id: mission_id.clone(),
            description: "Mission".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    purposes.insert(
        vision_id.clone(),
        Purpose {
            id: vision_id.clone(),
            description: "Vision".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    purposes.insert(
        values_id.clone(),
        Purpose {
            id: values_id.clone(),
            description: "Values".to_string(),
            display: DisplayAttributes::default(),
        },
    );

    let mut people = HashMap::new();
    people.insert(
        person1_id.clone(),
        Person {
            id: person1_id.clone(),
            name: "Person 1".to_string(),
            title: "".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    people.insert(
        person2_id.clone(),
        Person {
            id: person2_id.clone(),
            name: "Person 2".to_string(),
            title: "".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    people.insert(
        person3_id.clone(),
        Person {
            id: person3_id.clone(),
            name: "Person 3".to_string(),
            title: "".to_string(),
            display: DisplayAttributes::default(),
        },
    );

    let mut projects = HashMap::new();
    projects.insert(
        project1_id.clone(),
        Project {
            id: project1_id.clone(),
            name: "Project A".to_string(),
            status: ProjectStatus::Active,
            display: DisplayAttributes::default(),
        },
    );
    projects.insert(
        project2_id.clone(),
        Project {
            id: project2_id.clone(),
            name: "Project B".to_string(),
            status: ProjectStatus::Active,
            display: DisplayAttributes::default(),
        },
    );

    let mut production_systems = HashMap::new();
    production_systems.insert(
        prod1_id.clone(),
        ProductionSystem {
            id: prod1_id.clone(),
            name: "Product 1".to_string(),
            status: SystemStatus::Operational,
            display: DisplayAttributes::default(),
        },
    );
    production_systems.insert(
        prod2_id.clone(),
        ProductionSystem {
            id: prod2_id.clone(),
            name: "Product 2".to_string(),
            status: SystemStatus::Operational,
            display: DisplayAttributes::default(),
        },
    );

    let mut property_items = HashMap::new();
    property_items.insert(
        asset1_id.clone(),
        PropertyItem {
            id: asset1_id.clone(),
            name: "Asset 1".to_string(),
            property_type: PropertyType::Physical,
            display: DisplayAttributes::default(),
        },
    );
    property_items.insert(
        asset2_id.clone(),
        PropertyItem {
            id: asset2_id.clone(),
            name: "Asset 2".to_string(),
            property_type: PropertyType::Physical,
            display: DisplayAttributes::default(),
        },
    );
    property_items.insert(
        asset3_id.clone(),
        PropertyItem {
            id: asset3_id.clone(),
            name: "Asset 3".to_string(),
            property_type: PropertyType::Physical,
            display: DisplayAttributes::default(),
        },
    );

    let mut progress_metrics = HashMap::new();
    progress_metrics.insert(
        metric1_id.clone(),
        ProgressMetric {
            id: metric1_id.clone(),
            name: "Metric 1".to_string(),
            metric_type: "KPI".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    progress_metrics.insert(
        metric2_id.clone(),
        ProgressMetric {
            id: metric2_id.clone(),
            name: "Metric 2".to_string(),
            metric_type: "KPI".to_string(),
            display: DisplayAttributes::default(),
        },
    );
    progress_metrics.insert(
        metric3_id.clone(),
        ProgressMetric {
            id: metric3_id.clone(),
            name: "Metric 3".to_string(),
            metric_type: "KPI".to_string(),
            display: DisplayAttributes::default(),
        },
    );

    let relationships = vec![
        // person1 -> project1 (Manages)
        Relationship {
            subject_id: person1_id.clone(),
            subject_type: EntityType::Person,
            predicate: RelationType::Manages,
            object_id: project1_id.clone(),
            object_type: EntityType::Project,
            display: EdgeDisplayAttributes::default(),
        },
        // person2 -> project2 (Leads)
        Relationship {
            subject_id: person2_id.clone(),
            subject_type: EntityType::Person,
            predicate: RelationType::Leads,
            object_id: project2_id.clone(),
            object_type: EntityType::Project,
            display: EdgeDisplayAttributes::default(),
        },
        // project1 -> prod1 (Delivers)
        Relationship {
            subject_id: project1_id.clone(),
            subject_type: EntityType::Project,
            predicate: RelationType::TransitionsTo,
            object_id: prod1_id.clone(),
            object_type: EntityType::ProductionSystem,
            display: EdgeDisplayAttributes {
                label: Some("Delivers".to_string()),
                ..Default::default()
            },
        },
        // project2 -> prod2 (Maintains)
        Relationship {
            subject_id: project2_id.clone(),
            subject_type: EntityType::Project,
            predicate: RelationType::Maintains,
            object_id: prod2_id.clone(),
            object_type: EntityType::ProductionSystem,
            display: EdgeDisplayAttributes::default(),
        },
        // asset2 -> prod2 (Used by)
        Relationship {
            subject_id: asset2_id.clone(),
            subject_type: EntityType::Property,
            predicate: RelationType::Uses,
            object_id: prod2_id.clone(),
            object_type: EntityType::ProductionSystem,
            display: EdgeDisplayAttributes {
                label: Some("Used by".to_string()),
                ..Default::default()
            },
        },
        // metric1 -> project1 (Tracks)
        Relationship {
            subject_id: metric1_id.clone(),
            subject_type: EntityType::Progress,
            predicate: RelationType::PartOf,
            object_id: project1_id.clone(),
            object_type: EntityType::Project,
            display: EdgeDisplayAttributes {
                label: Some("Tracks".to_string()),
                ..Default::default()
            },
        },
    ];

    let org = Organization {
        name: "Organization".to_string(),
        purposes,
        people,
        projects,
        progress_metrics,
        production_systems,
        property_items,
        relationships,
    };

    let config = DotConfig {
        use_hierarchical_layout: true,
        use_template_mode: true,
        rankdir: "TB",
        show_status: false,
        ..Default::default()
    };

    let dot_string = org.to_dot(&config);
    println!("{}", dot_string);

    // Now pass dot_string to graphviz to generate SVG
    // Then pass that SVG to your Dioxus GraphvizSvg component
}
