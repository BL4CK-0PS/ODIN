# AI Architecture

## Overview

ODIN uses LLMs for three specific, bounded tasks:
1. Narrative generation from structured incident data
2. Similarity explanation between incidents
3. MITRE technique suggestion from evidence text

## Guiding Principles

- **Deterministic grounding** — every AI output is traceable to specific evidence
- **Bounded scope** — AI never makes autonomous decisions; it only generates text
- **LLM-agnostic** — adapter pattern supports OpenAI, Anthropic, local (Ollama)
- **Auditable** — every prompt + response is logged with model + version metadata

## Architecture

```
┌──────────────┐     ┌────────────────┐     ┌──────────────┐
│  Caller       │────►│  LLM Adapter   │────►│   Provider    │
│  (Service)    │     │  (odin-llm)    │     │   (OpenAI/    │
│               │◄────│                │◄────│   Anthropic/  │
│               │     │                │     │   Ollama)     │
└──────────────┘     └────────────────┘     └──────────────┘
                            │
                            ▼
                     ┌──────────────┐
                     │  Prompt       │
                     │  Templates    │
                     │  (odin-prompts)│
                     └──────────────┘
```

## LLM Adapter Interface

```rust
#[async_trait]
trait LlmProvider {
    /// Generate text from a prompt with optional system message
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;

    /// Generate structured output (JSON mode)
    async fn generate_structured<T: DeserializeOwned + Send>(
        &self,
        request: StructuredRequest,
    ) -> Result<T, LlmError>;

    /// Get current model info
    fn model_info(&self) -> ModelInfo;
}

struct LlmRequest {
    system_prompt: Option<String>,
    messages: Vec<ChatMessage>,
    temperature: f64,        // default: 0.3
    max_tokens: usize,
    response_format: ResponseFormat,
}
```

## Provider Configuration

```toml
[llm]
provider = "openai"           # openai | anthropic | ollama
model = "gpt-4o-mini"         # or "claude-sonnet-4-20250514", "llama3.1:8b"
temperature = 0.3
max_tokens = 4096

[llm.openai]
api_key = "${OPENAI_API_KEY}"
organization = ""

[llm.anthropic]
api_key = "${ANTHROPIC_API_KEY}"

[llm.ollama]
base_url = "http://localhost:11434"
```

## Prompt Caching

- Identical prompts (same incident state) served from cache: TTL 5 minutes
- Cache key: hash(incident_id + incident_updated_at + template_name + model)
- Invalidation: on incident update, on model change

## Rate Limiting

- Per-provider token bucket: configurable RPM and TPM
- Queue overflow to Redis for async generation
- Graceful degradation: return cached narrative if LLM unavailable
