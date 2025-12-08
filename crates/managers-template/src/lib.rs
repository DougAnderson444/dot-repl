//! Type definitions for the template manager.
//! Uses schemars to generate JSON schema for the data structure.
//! This file is used to define the structure of the data that will be managed by the template manager.
use std::fmt::Display;

use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};

/// Top most parent structure for the organization.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Organization {
    /// The name of the organization.
    pub name: String,
    /// The purpose of the organization.
    pub purpose: Purpose,
    /// The people involved in the organization.
    pub people: People,
    /// The projects managed by the organization.
    pub projects: Projects,
    /// The systems in production managed by the organization.
    pub production_systems: Vec<Production>,
    /// Progress made by projects and systems in production.
    pub progress: Progress,
    /// The property available to the organization.
    pub property: Property,
}

/// The purpose of this organization.
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Purpose {
    pub description: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Person {
    /// The name of this person.
    pub name: String,
    /// The title or role of this person in the organization.
    pub title: String,
    /// The projects this person is involved in.
    pub projects: Projects,
    /// The production systems this person is responsible for.
    pub production_systems: Vec<Production>,
}

/// The people involved in the organization.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct People {
    pub members: Vec<Person>,
}

/// Project
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Project {
    pub name: String,
    /// The purpose which this project serves.
    pub purpose: Purpose,
    /// What production group will maintain this project once it's in production.
    pub production_group: Production,
}

/// The Projects managed by the organization.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Projects {
    pub projects: Vec<Project>,
}

impl From<Vec<Project>> for Projects {
    fn from(projects: Vec<Project>) -> Self {
        Projects { projects }
    }
}

/// A system in production managed by the organization.
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Production {
    pub name: String,
}

/// Progress made by projects and systems in production.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Progress {
    pub project_progress: Vec<(String, String)>, // (project_name, progress_description)
    pub system_progress: Vec<(String, String)>,  // (system_name, progress_description)
}

/// The property available to the organization
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Property {
    pub items: Vec<String>,
}

impl Organization {
    /// Generates a JSON schema for the Organization structure.
    pub fn schema() -> Schema {
        schemars::schema_for!(Self)
    }
}

impl From<Organization> for String {
    fn from(org: Organization) -> Self {
        serde_json::to_string_pretty(&org).unwrap_or_default()
    }
}

impl Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // json pretty print
        let json = serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        let schema = Organization::schema();
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        // Further assertions can be made here to validate the schema structure.
    }
}
