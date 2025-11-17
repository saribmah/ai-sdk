/// Example demonstrating how to use the bash_20241022 provider tool.
///
/// This shows various ways to create and configure the Anthropic bash tool
/// using the provider-defined tool factory pattern.
use ai_sdk_anthropic::provider_tool::bash_20241022;
use ai_sdk_provider_utils::{ProviderDefinedToolOptions, ToolExecutionOutput, ToolType};
use serde_json::json;
use std::sync::Arc;

fn main() {
    println!("=== Anthropic bash_20241022 Tool Examples ===\n");

    // Example 1: Create with default options
    println!("1. Basic bash tool (default options):");
    let tool1 = bash_20241022(None);

    if let ToolType::ProviderDefined { id, name, args } = &tool1.tool_type {
        println!("   ID: {}", id);
        println!("   Name: {}", name);
        println!(
            "   Args: {}",
            if args.is_empty() { "none" } else { "custom" }
        );
    }
    println!("   Has description: {}", tool1.description.is_some());
    println!("   Has execute function: {}", tool1.execute.is_some());
    println!();

    // Example 2: With description
    println!("2. Bash tool with description:");
    let tool2 = bash_20241022(Some(
        ProviderDefinedToolOptions::new()
            .with_description("Execute bash commands in a sandboxed environment"),
    ));
    println!("   Description: {:?}", tool2.description);
    println!();

    // Example 3: With approval requirement
    println!("3. Bash tool requiring approval:");
    let tool3 = bash_20241022(Some(
        ProviderDefinedToolOptions::new()
            .with_description("Execute bash commands (requires approval)")
            .with_needs_approval(true),
    ));
    println!("   Description: {:?}", tool3.description);
    println!("   Needs approval: Yes");
    println!();

    // Example 4: With custom arguments
    println!("4. Bash tool with custom configuration:");
    let tool4 = bash_20241022(Some(
        ProviderDefinedToolOptions::new()
            .with_description("Bash with timeout")
            .with_arg("timeout", json!(30))
            .with_arg("shell", json!("/bin/bash"))
            .with_arg("working_dir", json!("/tmp")),
    ));

    if let ToolType::ProviderDefined { args, .. } = &tool4.tool_type {
        println!(
            "   Custom args: {}",
            serde_json::to_string_pretty(args).unwrap()
        );
    }
    println!();

    // Example 5: With execute function
    println!("5. Bash tool with custom execute function:");
    let tool5 = bash_20241022(Some(
        ProviderDefinedToolOptions::new()
            .with_description("Bash with custom execution")
            .with_execute(Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    let command = input
                        .get("command")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(no command)");

                    println!("      [Simulated] Executing: {}", command);

                    Ok(json!({
                        "output": format!("Simulated output for: {}", command),
                        "exit_code": 0
                    }))
                }))
            })),
    ));
    println!("   Has execute function: {}", tool5.execute.is_some());
    println!();

    // Example 6: Input schema structure
    println!("6. Input schema structure:");
    let tool6 = bash_20241022(None);
    println!(
        "{}",
        serde_json::to_string_pretty(&tool6.input_schema).unwrap()
    );
    println!();

    // Example 7: Conditional approval
    println!("7. Bash tool with conditional approval:");
    let _tool7 = bash_20241022(Some(
        ProviderDefinedToolOptions::new()
            .with_description("Bash with smart approval")
            .with_needs_approval_function(Arc::new(|input, _opts| {
                Box::pin(async move {
                    // Require approval for dangerous commands
                    let command = input.get("command").and_then(|v| v.as_str()).unwrap_or("");

                    let dangerous_keywords = ["rm", "delete", "format", "dd", ">"];
                    dangerous_keywords
                        .iter()
                        .any(|&keyword| command.contains(keyword))
                })
            })),
    ));
    println!("   Uses conditional approval (requires approval for dangerous commands)");
    println!();

    // Example 8: Multiple tools from the same configuration
    println!("8. Creating multiple bash tool instances:");
    let tools: Vec<_> = (1..=3)
        .map(|i| {
            bash_20241022(Some(
                ProviderDefinedToolOptions::new()
                    .with_description(format!("Bash instance #{}", i))
                    .with_arg("instance_id", json!(i)),
            ))
        })
        .collect();

    for (i, tool) in tools.iter().enumerate() {
        println!("   Tool {}: {:?}", i + 1, tool.description);
    }
    println!();

    println!("=== Example Complete ===");
    println!("\nKey Features:");
    println!("  ✓ Simple factory pattern (bash_20241022)");
    println!("  ✓ Optional configuration via ProviderDefinedToolOptions");
    println!("  ✓ Support for descriptions, approval, custom args");
    println!("  ✓ Optional execute functions for custom logic");
    println!("  ✓ Fully compatible with Anthropic's API");
}
