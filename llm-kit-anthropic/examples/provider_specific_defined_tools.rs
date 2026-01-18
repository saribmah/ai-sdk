/// Provider-defined tools example with Anthropic Claude.
///
/// This example demonstrates how to access Anthropic's provider-defined tools
/// using only llm-kit-provider (no llm-kit-core dependency).
///
/// NOTE: This is a demonstration example. Provider-defined tools are available
/// via the anthropic_tools module, which can be used with llm-kit-core's tool
/// execution system for full functionality.
///
/// Run with:
/// ```bash
/// cargo run --example provider_specific_defined_tools -p llm-kit-anthropic
/// ```
use llm_kit_anthropic::anthropic_tools;

fn main() {
    println!("🛠️  Anthropic Provider-Defined Tools Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Available Provider-Defined Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Anthropic provides several powerful provider-defined tools:");
    println!();

    // Demonstrate bash tool creation
    let _bash_tool = anthropic_tools::bash_20241022(None);
    println!("✓ bash_20241022");
    println!("  Description: Executes bash commands");
    println!();

    // Demonstrate web search tool creation
    let _web_search_tool = anthropic_tools::web_search_20250305().max_uses(5).build();
    println!("✓ web_search_20250305");
    println!("  Description: Searches the web with citations");
    println!();

    // List other available tools
    println!("✓ computer_20241022 / computer_20250124");
    println!("  Description: Control desktop environments");
    println!();

    println!("✓ code_execution_20250522 / code_execution_20250825");
    println!("  Description: Execute Python/Bash code");
    println!();

    println!("✓ text_editor_20241022 / text_editor_20250124 / text_editor_20250728");
    println!("  Description: Edit files");
    println!();

    println!("✓ web_fetch_20250910");
    println!("  Description: Fetch web content");
    println!();

    println!("✓ memory_20250818");
    println!("  Description: Persistent memory across conversations");
    println!();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Points:");
    println!("   ✓ Access provider-defined tools via anthropic_tools module");
    println!("   ✓ Tools can be configured with builder pattern");
    println!("   ✓ Use with llm-kit-core for full tool execution");
    println!("\n📝 For full examples with tool execution, see:");
    println!("   - Provider examples in the llm-kit-anthropic/examples/ directory");
    println!("   - Main examples in the root examples/ directory");
}
