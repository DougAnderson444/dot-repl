//! Creates a basic Organization which meets the JSON schema.
//!
//! # Example
//!
//! ```sh
//! cargo run --example basic --package managers-template -- --nocapture
//! ```
use managers_template::{
    Organization, People, Person, Production, Progress, Project, Projects, Property, Purpose,
};

fn main() {
    // Build the structure from the bottom up
    let purpose = Purpose {
        description: "To provide quality products and services.".to_string(),
    };

    let production_system_a = Production {
        name: "System A".to_string(),
    };

    let production_system_b = Production {
        name: "System B".to_string(),
    };

    let project_x = Project {
        name: "Project X".to_string(),
        purpose: purpose.clone(),
        production_group: production_system_a.clone(),
    };

    let project_y = Project {
        name: "Project Y".to_string(),
        purpose: purpose.clone(),
        production_group: production_system_b.clone(),
    };

    let projects = Projects {
        projects: vec![project_x.clone(), project_y.clone()],
    };

    let founder = Person {
        name: "Alice".to_string(),
        title: "Founder".to_string(),
        projects: vec![project_x].into(),
        production_systems: vec![production_system_a.clone(), production_system_b.clone()],
    };

    let people = People {
        members: vec![founder],
    };

    let progress = Progress {
        project_progress: vec![
            ("Project X".to_string(), "On track".to_string()),
            ("Project Y".to_string(), "Delayed".to_string()),
        ],
        system_progress: vec![
            ("System A".to_string(), "Operational".to_string()),
            ("System B".to_string(), "Maintenance".to_string()),
        ],
    };

    let property = Property {
        items: vec![
            "Office Building".to_string(),
            "Company Vehicles".to_string(),
        ],
    };

    let org = Organization {
        name: "Acme Corp".to_string(),
        purpose,
        people,
        projects,
        production_systems: vec![production_system_a, production_system_b],
        progress,
        property,
    };

    println!("Organization JSON Schema:\n{org}");
}
