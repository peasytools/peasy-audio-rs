use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub tool_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub slug: String,
    pub name: String,
    pub extension: String,
    pub mime_type: String,
    pub category: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Conversion {
    pub source: String,
    pub target: String,
    pub description: String,
    pub tool_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct GlossaryTerm {
    pub slug: String,
    pub term: String,
    pub definition: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct Guide {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub audience_level: String,
    pub word_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct UseCase {
    pub slug: String,
    pub title: String,
    pub industry: String,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub name: String,
    pub domain: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub results: SearchCategories,
}

#[derive(Debug, Deserialize)]
pub struct SearchCategories {
    pub tools: Vec<Tool>,
    pub formats: Vec<Format>,
    pub glossary: Vec<GlossaryTerm>,
}

/// Options for paginated list requests.
#[derive(Default)]
pub struct ListOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
    pub search: Option<String>,
}

/// Options for list_guides with audience level.
#[derive(Default)]
pub struct ListGuidesOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
    pub audience_level: Option<String>,
    pub search: Option<String>,
}

/// Options for list_conversions with source/target.
#[derive(Default)]
pub struct ListConversionsOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub source: Option<String>,
    pub target: Option<String>,
}
