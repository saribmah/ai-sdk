use ai_sdk_openai::*;
use ai_sdk_provider::language_model::LanguageModel;

fn test_provider() -> OpenAIProvider {
    let settings = OpenAIProviderSettings::default().with_api_key("test-key");
    create_openai(settings)
}

#[test]
fn test_create_provider() {
    let provider = test_provider();
    let model = provider.chat("gpt-4o");

    // Test that model is created successfully
    assert_eq!(model.provider(), "openai.chat");
}

#[test]
fn test_create_model() {
    let provider = test_provider();
    let model = provider.chat("gpt-4o");

    // Test that model is created successfully
    assert_eq!(model.model_id(), "gpt-4o");
    assert_eq!(model.provider(), "openai.chat");
}

#[test]
fn test_reasoning_model_detection() {
    let provider = test_provider();

    // Test various reasoning models
    let o1_model = provider.chat("o1");
    assert_eq!(o1_model.model_id(), "o1");

    let o1_preview = provider.chat("o1-preview");
    assert_eq!(o1_preview.model_id(), "o1-preview");

    let o3_mini = provider.chat("o3-mini");
    assert_eq!(o3_mini.model_id(), "o3-mini");
}

#[test]
fn test_custom_settings() {
    let settings = OpenAIProviderSettings::default()
        .with_base_url("https://custom.openai.com")
        .with_api_key("custom-key");

    let provider = create_openai(settings);
    let model = provider.chat("gpt-4o");
    assert_eq!(model.provider(), "openai.chat");
}

#[test]
fn test_model_ids() {
    // Test that various model IDs are accepted
    let provider = test_provider();

    let models = vec![
        "gpt-4o",
        "gpt-4o-mini",
        "gpt-4-turbo",
        "gpt-3.5-turbo",
        "o1",
        "o1-mini",
        "o3-mini",
    ];

    for model_id in models {
        let model = provider.chat(model_id);
        assert_eq!(model.model_id(), model_id);
    }
}
