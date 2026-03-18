use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // List available audio tools
    println!("=== Audio Tools ===");
    let tools = client.list_tools(&Default::default()).await?;
    println!("  Total: {} tools", tools.count);
    for tool in &tools.results {
        println!("  {} — {}", tool.name, tool.display_desc());
    }

    // Get a specific tool
    println!("\n=== Audio BPM Tool ===");
    let tool = client.get_tool("audio-bpm").await?;
    println!("  {}: {}", tool.name, tool.display_desc());

    // List categories
    println!("\n=== Categories ===");
    let categories = client.list_categories(&Default::default()).await?;
    for cat in &categories.results {
        println!("  {} ({} tools)", cat.name, cat.tool_count);
    }

    // List formats
    println!("\n=== Formats ===");
    let formats = client.list_formats(&Default::default()).await?;
    for fmt in &formats.results {
        println!("  .{} — {} ({})", fmt.extension, fmt.name, fmt.mime_type);
    }

    // Search across content
    println!("\n=== Search: 'convert' ===");
    let results = client.search("convert", Some(5)).await?;
    println!("  Tools: {}", results.results.tools.len());
    println!("  Formats: {}", results.results.formats.len());
    println!("  Glossary: {}", results.results.glossary.len());

    // List glossary terms
    println!("\n=== Glossary ===");
    let glossary = client.list_glossary(&Default::default()).await?;
    for term in glossary.results.iter().take(5) {
        println!("  {}: {}", term.term, term.definition);
    }

    // List Peasy sister sites
    println!("\n=== Peasy Tools Family ===");
    let sites = client.list_sites().await?;
    for site in &sites.results {
        println!("  {} — {}", site.name, site.domain);
    }

    Ok(())
}
