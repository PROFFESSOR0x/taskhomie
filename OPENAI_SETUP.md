# OpenAI-Compatible API Setup Guide

## Overview

Taskhomie now supports OpenAI-compatible APIs in addition to Anthropic's Claude. You can use:
- **OpenAI** (GPT-4o, GPT-4 Turbo, o1, etc.)
- **Custom providers** (Ollama, LM Studio, vLLM, Azure OpenAI, etc.)

## Quick Setup

### 1. Set Your API Key

Create or edit the `.env` file in the project root:

```bash
# For OpenAI
OPENAI_API_KEY=sk-your-openai-key-here

# For Anthropic (still supported)
ANTHROPIC_API_KEY=sk-ant-your-anthropic-key-here

# Optional: Voice features
DEEPGRAM_API_KEY=your-deepgram-key
ELEVENLABS_API_KEY=your-elevenlabs-key
```

### 2. Select Your Provider

**In the UI:**
1. Click the provider dropdown in the header (next to the model selector)
2. Choose: **Anthropic**, **OpenAI**, or **Custom**
3. Select your desired model

**In Settings:**
1. Click the Settings icon (⚙️)
2. Go to **API Provider** section
3. Select provider and model
4. For Custom providers, enter your base URL

## Using Custom Providers

### Ollama (Local Models)

```bash
# 1. Install and run Ollama
ollama serve

# 2. Set in .env
OPENAI_API_KEY=ollama  # Ollama doesn't require a real key
```

In the UI:
- Provider: **Custom**
- Base URL: `http://localhost:11434/v1/chat/completions`
- Model: Your Ollama model (e.g., `llama3.1`, `mistral`, etc.)

### LM Studio

```bash
# 1. Start LM Studio local server
# 2. Set in .env
OPENAI_API_KEY=lm-studio
```

In the UI:
- Provider: **Custom**
- Base URL: `http://localhost:1234/v1/chat/completions`
- Model: Your loaded model

### Azure OpenAI

```bash
# Set in .env
OPENAI_API_KEY=your-azure-key
```

In the UI:
- Provider: **Custom**
- Base URL: `https://your-resource.openai.azure.com/openai/deployments/your-deployment/chat/completions?api-version=2024-02-15-preview`
- Model: Your deployment name

### vLLM

```bash
# 1. Start vLLM server
python -m vllm.entrypoints.openai.api_server --model your-model

# 2. Set in .env
OPENAI_API_KEY=vllm
```

In the UI:
- Provider: **Custom**
- Base URL: `http://localhost:8000/v1/chat/completions`

## Available Models

### Anthropic
- Claude Haiku 4.5
- Claude Sonnet 4.5
- Claude Opus 4.5

### OpenAI
- GPT-4o
- GPT-4o Mini
- GPT-4 Turbo
- o1
- o1 Mini
- o3 Mini

### Custom (examples)
- GPT-4o
- GPT-4 Turbo
- Claude Sonnet
- Claude Haiku
- Llama 3.1 70B
- Mistral Large
- Any model your provider supports

## Important Notes

### Computer Use Mode
The `computer` tool for screen control is optimized for Claude. When using OpenAI or other providers:
- The tool definitions are converted to OpenAI's function calling format
- Some advanced features may work differently
- Test thoroughly before autonomous use

### Browser Mode
Browser automation tools work with all providers since they use standard function calling.

### Voice Mode
Voice features (TTS) require ElevenLabs API key regardless of provider.

### Token Usage
Different providers have different token limits and pricing. Check your provider's documentation.

## Troubleshooting

### "No API key set" error
Make sure you've set the correct environment variable:
- `OPENAI_API_KEY` for OpenAI/Custom
- `ANTHROPIC_API_KEY` for Anthropic

### Custom provider not connecting
1. Verify the base URL is correct
2. Check that your server is running
3. Ensure CORS is enabled (for web-based servers)
4. Check firewall settings

### Model not responding
- Verify the model name matches exactly
- Check if the model supports function/tool calling
- Some models may need specific parameters

## Architecture

The implementation includes:

1. **OpenAI API Client** (`src-tauri/src/openai_api.rs`)
   - SSE streaming support
   - Function calling format
   - Compatible with OpenAI and OpenAI-compatible APIs

2. **Frontend Types** (`src/types/index.ts`)
   - `ApiProvider` type
   - Extended `ModelId` union
   - Provider configuration state

3. **State Management** (`src/stores/agentStore.ts`)
   - Provider selection
   - Custom base URL
   - Custom model name

4. **UI Components**
   - Provider selector in header
   - Model selector (dynamic based on provider)
   - Settings panel with provider config

## Future Enhancements

To fully integrate OpenAI as a first-class citizen (beyond the current implementation):

1. Update `agent.rs` to branch based on provider:
   - Use `AnthropicClient` for Anthropic
   - Use `OpenAiClient` for OpenAI/Custom
   - Convert tool results between formats

2. Add provider-specific optimizations:
   - Different system prompts per provider
   - Provider-specific token counting
   - Model-specific parameters

3. Test all tools with each provider:
   - Computer control
   - Browser automation
   - Bash execution
   - Voice features

## Contributing

If you enhance the OpenAI integration, please contribute back! The architecture is designed to be extended.
