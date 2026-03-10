# Taskhomie (Computer Use AI Agent)
<img width="846" height="606" alt="Screenshot 2025-12-29 at 2 06 38 AM" src="https://github.com/user-attachments/assets/b5b7de82-ec58-424f-af68-e9287a6422d6" />

Local AI agent that controls your computer. Give it natural language instructions and watch it take screenshots, move your mouse, click, type, and run terminal commands.

Built with Tauri, React, and Rust.

**Now supports OpenAI-compatible APIs!** Use OpenAI GPT models, Ollama, LM Studio, vLLM, Azure OpenAI, and more. See [OPENAI_SETUP.md](OPENAI_SETUP.md) for setup instructions.

## Demo

https://github.com/user-attachments/assets/8edd92a7-7d3e-472a-9e48-3b561f0257d6

Here, I used it to autonomously read and reply to tweets, lol. This is purely for demonstration/research, you should not attempt to do the same, lol.

## Modes

**Computer Use Mode** - Takes over your screen. Sees what you see via screenshots and controls your cursor and keyboard directly. Use when the task spans multiple apps or needs full desktop access. You step away while it works.

**Background Mode** - Runs async while you do other things. Uses Chrome DevTools Protocol for web automation and terminal for everything else. Doesn't touch your mouse or keyboard. Faster and more reliable for web + terminal tasks.

## Setup

**Requirements:**
- Rust & Cargo
- Node.js & npm
- Anthropic API key **OR** OpenAI API key (or custom provider)

```bash
# install deps
npm install

# add your api key(s)
echo "ANTHROPIC_API_KEY=your-key-here" > .env
# Or for OpenAI:
echo "OPENAI_API_KEY=your-key-here" >> .env

# run dev
npm run tauri dev

# or build for production
npm run tauri build
```

### Using Custom Providers (Ollama, LM Studio, etc.)

See [OPENAI_SETUP.md](OPENAI_SETUP.md) for detailed setup instructions for:
- Ollama (local models)
- LM Studio
- Azure OpenAI
- vLLM
- Other OpenAI-compatible APIs

On macOS, you'll need to grant accessibility permissions when prompted (System Settings → Privacy & Security → Accessibility).

## Shortcuts

- `⌃⇧C` - push-to-talk → computer use mode
- `⌃⇧B` - push-to-talk → background mode
- `⌘⇧H` - help mode (screenshot + quick prompt)
- `⌘⇧S` - stop agent

## Stack

- **Frontend**: React, TypeScript, Tailwind, Zustand, Framer Motion
- **Backend**: Rust, Tauri 2, Tokio
- **Models**: 
  - Anthropic: Haiku, Sonnet, Opus
  - OpenAI: GPT-4o, GPT-4 Turbo, o1, o3-mini
  - Custom: Any OpenAI-compatible API (Ollama, LM Studio, vLLM, etc.)

## Contributing

PRs welcome. Hit me up on Twitter @ishanxnagpal.

## License

[Apache License 2.0](LICENSE)
