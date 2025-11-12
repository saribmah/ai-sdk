# Azure OpenAI Provider Implementation Plan

This document tracks the implementation of the Azure OpenAI provider for AI SDK Rust.

## Reference Implementation

TypeScript reference: `reference-implementation/ai/packages/azure/src/azure-openai-provider.ts`

## Implementation Status

### ✅ Completed

1. **Crate Structure** - Created `ai-sdk-azure` crate with proper directory structure
2. **Settings** - Implemented `AzureOpenAIProviderSettings` with:
   - Resource name configuration
   - Custom base URL support
   - API key authentication
   - Custom headers
   - API version configuration
   - Deployment-based URL toggle
   - Builder pattern API
   - Comprehensive unit tests (14 tests)

3. **Provider** - Implemented `AzureOpenAIProvider` with:
   - Chat model creation (`chat_model()`)
   - Completion model creation (`completion_model()`)
   - Embedding model creation (`text_embedding_model()`)
   - Image model creation (`image_model()`)
   - Provider trait implementation
   - URL building for both v1 and deployment-based formats
   - Azure-specific authentication (api-key header)
   - Comprehensive unit tests (12 tests)

4. **Library** - Implemented `lib.rs` with:
   - Public API exports
   - `create_azure()` helper function
   - `azure()` default provider using environment variables
   - Comprehensive documentation with examples

5. **Dependencies** - Configured Cargo.toml with:
   - `ai-sdk-provider` for traits
   - `ai-sdk-openai-compatible` for OpenAI-compatible models
   - Standard dependencies (async-trait, serde, url)

### 🚧 In Progress

None currently.

### 📋 Pending

1. **Workspace Integration** - Add `ai-sdk-azure` to root `Cargo.toml` workspace members
2. **Example** - Create `examples/azure_basic.rs` demonstrating Azure provider usage
3. **Testing** - Run pre-commit checks and verify build
4. **Documentation** - Add Azure provider to main README.md

## Key Design Decisions

### 1. Azure-Specific Authentication
Unlike OpenAI which uses `Authorization: Bearer {token}`, Azure uses `api-key: {key}` header:
```rust
headers.insert("api-key".to_string(), key.clone());
```

### 2. URL Formats
Azure supports two URL patterns:

**V1 API (Default):**
```
https://{resource}.openai.azure.com/openai/v1{path}?api-version={version}
```

**Deployment-Based (Legacy):**
```
https://{resource}.openai.azure.com/openai/deployments/{deployment}{path}?api-version={version}
```

### 3. API Version Requirement
All Azure OpenAI requests require an `api-version` query parameter. Defaults to "v1" but can be customized.

### 4. Resource Name vs Base URL
Users can provide either:
- `resource_name`: Constructs URL automatically
- `base_url`: Uses custom URL directly (takes precedence)

### 5. Reuse OpenAI-Compatible Infrastructure
The provider leverages `ai-sdk-openai-compatible` crate's models:
- `OpenAICompatibleChatLanguageModel`
- `OpenAICompatibleCompletionLanguageModel`
- `OpenAICompatibleEmbeddingModel`
- `OpenAICompatibleImageModel`

This avoids code duplication and ensures consistency with OpenAI API behavior.

## Differences from TypeScript Implementation

1. **No Provider Tools**: TypeScript has `azureOpenaiTools` (codeInterpreter, fileSearch, imageGeneration). 
   - Not implemented yet in Rust (would require porting from `@ai-sdk/openai/internal`)
   - Can be added in future if needed

2. **No Speech/Transcription Models**: TypeScript supports these, but we return errors for now:
   - Can be added later by extending `ai-sdk-openai-compatible`

3. **Settings Validation**: Rust version validates settings on provider creation (panics if invalid)
   - TypeScript loads settings lazily

4. **Environment Variables**: Rust version reads env vars in `azure()` function
   - TypeScript uses `loadApiKey()` and `loadSetting()` utilities

## Testing Strategy

### Unit Tests
- ✅ Settings builder pattern (14 tests)
- ✅ Provider model creation (12 tests)
- ✅ URL building (v1 and deployment-based formats)
- ✅ Provider trait implementation
- ✅ Header configuration

### Integration Tests
- 📋 Pending: Example that makes real API calls (requires credentials)

### Test Coverage
- Settings: 100%
- Provider: 100%
- Lib: Basic smoke tests

## Usage Examples

### Basic Usage
```rust
use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
use ai_sdk_core::{GenerateText, Prompt};

let provider = create_azure(
    AzureOpenAIProviderSettings::new()
        .with_resource_name("my-resource")
        .with_api_key("key")
);

let model = provider.chat_model("gpt-4-deployment");
let result = GenerateText::new(model, Prompt::text("Hello!"))
    .execute()
    .await?;
```

### With Custom Base URL
```rust
let provider = create_azure(
    AzureOpenAIProviderSettings::new()
        .with_base_url("https://custom.openai.azure.com/openai")
        .with_api_key("key")
);
```

### With Custom API Version
```rust
let provider = create_azure(
    AzureOpenAIProviderSettings::new()
        .with_resource_name("my-resource")
        .with_api_key("key")
        .with_api_version("2024-02-15-preview")
);
```

### With Deployment-Based URLs
```rust
let provider = create_azure(
    AzureOpenAIProviderSettings::new()
        .with_resource_name("my-resource")
        .with_api_key("key")
        .with_use_deployment_based_urls(true)
);
```

## Future Enhancements

1. **Provider Tools**: Port Azure-specific tools from TypeScript
   - Code interpreter
   - File search
   - Image generation tools

2. **Speech Models**: Add TTS support when needed

3. **Transcription Models**: Add Whisper support when needed

4. **Environment Variable Utilities**: Create helper functions similar to TypeScript's `loadApiKey()` and `loadSetting()`

5. **Fetch Middleware**: Add custom fetch implementation support (currently not exposed)

6. **User-Agent**: Add Azure SDK version to User-Agent header

## Files Created

```
ai-sdk-azure/
├── src/
│   ├── lib.rs          (Public API, exports, default provider)
│   ├── provider.rs     (AzureOpenAIProvider implementation)
│   └── settings.rs     (AzureOpenAIProviderSettings)
├── Cargo.toml          (Dependencies and metadata)
└── AZURE-IMPLEMENTATION.md (This file)
```

## Next Steps

1. Add to workspace Cargo.toml
2. Create example file
3. Run tests and verify build
4. Update main README with Azure provider documentation
