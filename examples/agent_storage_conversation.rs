use ai_sdk_core::ToolSet;
/// Agent storage conversation example demonstrating persistent conversations with agents.
///
/// This example shows how to:
/// - Configure an agent with persistent storage (stateful mode)
/// - Have multi-turn conversations with automatic history loading
/// - Store and retrieve conversation context across agent calls
/// - Use both stateful (session in AgentSettings) and stateless (session per call) modes
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example agent_storage_conversation --features storage
/// ```
use ai_sdk_core::tool::definition::Tool;
use ai_sdk_core::{Agent, AgentCallParameters, AgentInterface, AgentSettings};
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use ai_sdk_storage::Storage;
use ai_sdk_storage_filesystem::FilesystemStorage;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates storing a user's preference
fn store_preference(key: &str, value: &str) -> Value {
    println!("    💾 Storing preference: {} = {}", key, value);
    json!({
        "success": true,
        "key": key,
        "value": value,
        "message": format!("Stored preference: {}", key)
    })
}

/// Simulates setting a reminder
fn set_reminder(task: &str, time: &str) -> Value {
    println!("    ⏰ Setting reminder: {} at {}", task, time);
    json!({
        "success": true,
        "task": task,
        "time": time,
        "message": format!("Reminder set: {} at {}", task, time)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Agent Storage Conversation Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    let model = provider.chat_model("openai/gpt-4o-mini");
    println!("✓ Model loaded: {}\n", model.model_id());

    // Initialize storage
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Setting Up Storage");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let storage_path = "./storage-agent-example";
    let fs_storage = FilesystemStorage::new(storage_path)?;
    fs_storage.initialize().await?;

    let storage: Arc<dyn Storage> = Arc::new(fs_storage);
    let session_id = storage.generate_session_id();
    println!("✓ Storage initialized at: {}", storage_path);
    println!("✓ Session ID: {}\n", session_id);

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use ai_sdk_core::tool::definition::ToolExecutionOutput;

    let mut tools = ToolSet::new();

    // Preference storage tool
    let preference_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "key": {
                "type": "string",
                "description": "The preference key (e.g., 'favorite_color', 'language')"
            },
            "value": {
                "type": "string",
                "description": "The preference value"
            }
        },
        "required": ["key", "value"]
    }))
    .with_description("Store a user preference for future reference")
    .with_execute(Arc::new(|input: Value, _options| {
        let key = input.get("key").and_then(|v| v.as_str()).unwrap_or("");
        let value = input.get("value").and_then(|v| v.as_str()).unwrap_or("");

        let result = store_preference(key, value);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    // Reminder tool
    let reminder_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "task": {
                "type": "string",
                "description": "The task to be reminded about"
            },
            "time": {
                "type": "string",
                "description": "When to be reminded (e.g., '3pm', 'tomorrow', '2 hours')"
            }
        },
        "required": ["task", "time"]
    }))
    .with_description("Set a reminder for a specific task")
    .with_execute(Arc::new(|input: Value, _options| {
        let task = input.get("task").and_then(|v| v.as_str()).unwrap_or("");
        let time = input.get("time").and_then(|v| v.as_str()).unwrap_or("");

        let result = set_reminder(task, time);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    tools.insert("store_preference".to_string(), preference_tool);
    tools.insert("set_reminder".to_string(), reminder_tool);

    println!("📋 Available Tools:");
    println!("   1. store_preference - Store user preferences");
    println!("   2. set_reminder - Set reminders\n");

    // PART 1: Stateful Agent (session configured in AgentSettings)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Part 1: Stateful Agent (Session in AgentSettings)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let settings = AgentSettings::new(model.clone())
        .with_id("personal-assistant")
        .with_instructions(
            "You are a personal assistant that helps users manage their preferences and reminders. \
             You have access to tools to store preferences and set reminders. \
             When users mention their preferences, use the store_preference tool. \
             When they ask you to remind them of something, use the set_reminder tool. \
             Remember context from previous messages in the conversation.",
        )
        .with_temperature(0.7)
        .with_tools(tools)
        .with_storage(storage.clone())
        .with_session_id(session_id.clone());

    let agent = Agent::new(settings);

    println!("✓ Agent created (stateful mode)");
    println!("✓ Session ID: {}\n", session_id);

    // First message - no history
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Turn 1: Introduction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params1 =
        AgentCallParameters::from_text("Hi! My name is Alex and my favorite color is blue.");

    println!("📝 User: Hi! My name is Alex and my favorite color is blue.\n");

    let result1 = agent
        .generate(params1)?
        .without_history() // First message - no history to load
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result1.text);
    println!(
        "📊 Stats: {} tokens, {} steps\n",
        result1.usage.total_tokens,
        result1.steps.len()
    );

    // Second message - history automatically loaded
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Turn 2: Follow-up (History Loaded)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params2 = AgentCallParameters::from_text("Can you remind me to call my mom at 3pm?");

    println!("📝 User: Can you remind me to call my mom at 3pm?\n");

    let result2 = agent
        .generate(params2)?
        // No .without_history() - history loaded automatically!
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result2.text);
    println!(
        "📊 Stats: {} tokens, {} steps\n",
        result2.usage.total_tokens,
        result2.steps.len()
    );

    // Third message - testing context retention
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Turn 3: Context Retention Test");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params3 = AgentCallParameters::from_text("What's my name and what's my favorite color?");

    println!("📝 User: What's my name and what's my favorite color?\n");

    let result3 = agent.generate(params3)?.execute().await?;

    println!("🤖 Assistant: {}\n", result3.text);
    println!(
        "📊 Stats: {} tokens, {} steps\n",
        result3.usage.total_tokens,
        result3.steps.len()
    );

    // PART 2: Demonstrating conversation retrieval
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Part 2: Retrieving Conversation History");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Get the session
    let session = storage.get_session(&session_id).await?;
    println!("✓ Session retrieved");
    println!("   • Session ID: {}", session.id);
    println!("   • Created: {}", session.created_at);
    println!("   • Updated: {}", session.updated_at);

    // List all messages
    let message_ids = storage.list_messages(&session_id, None).await?;
    println!("\n✓ Messages in conversation: {}", message_ids.len());

    for (idx, msg_id) in message_ids.iter().enumerate() {
        let (role, parts) = storage.get_message(&session_id, msg_id).await?;
        println!("\n   Message {} ({})", idx + 1, msg_id);
        println!("   • Role: {:?}", role);
        println!("   • Parts: {}", parts.len());
        if let Some(first_part) = parts.first() {
            println!("   • First part created: {}", first_part.created_at());
        }
    }

    // PART 3: Stateless mode example (different session per call)
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Part 3: Stateless Agent (Session Per Call)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create agent without session_id in settings
    let tools2 = ToolSet::new();
    let settings_stateless = AgentSettings::new(model)
        .with_id("stateless-assistant")
        .with_instructions("You are a helpful assistant.")
        .with_temperature(0.7)
        .with_tools(tools2)
        .with_storage(storage.clone()); // Storage but no session_id

    let agent_stateless = Agent::new(settings_stateless);

    let new_session_id = storage.generate_session_id();
    println!("✓ Agent created (stateless mode)");
    println!("✓ New session ID: {}\n", new_session_id);

    let params4 = AgentCallParameters::from_text("Hello! This is a new session.");

    println!("📝 User: Hello! This is a new session.\n");

    // Specify session_id per call
    let result4 = agent_stateless
        .generate(params4)?
        .with_session_id(new_session_id.clone())
        .without_history()
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result4.text);

    // List all sessions
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Summary: All Sessions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let all_sessions = storage.list_sessions(None).await?;
    println!("✓ Total sessions: {}", all_sessions.len());

    for (idx, sess) in all_sessions.iter().enumerate() {
        let msg_count = storage.list_messages(&sess.id, None).await?.len();
        println!("\n   Session {}", idx + 1);
        println!("   • ID: {}", sess.id);
        println!("   • Messages: {}", msg_count);
        println!("   • Created: {}", sess.created_at);
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Agent Storage Conversation Example Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n💡 Key Takeaways:");
    println!("   • Stateful agents: Session ID in AgentSettings");
    println!("   • Stateless agents: Session ID per call");
    println!("   • History loaded automatically (unless .without_history())");
    println!("   • All messages persisted to filesystem storage");
    println!("   • Sessions can be retrieved and inspected");

    Ok(())
}
