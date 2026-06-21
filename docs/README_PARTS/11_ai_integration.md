## 🤖 AI & LLM Integration (Strictly Opt-In)

For developers and power users who want AI assistance without adding bloated Electron apps or embedded runtimes to the GUI, we maintain a standalone, headless MCP (Model Context Protocol) server.

By running the **MorBlogger MCP Engine**, you can connect your CLI agent (like Antigravity or Grok) or desktop IDE directly to the MorBlogger core. The AI can read your theme manifests, respect your structural constraints, and hot-reload your active UI workspace. 

Communication between the UI and the MCP server is handled via a zero-dependency, local Unix file drop. 

🔗 **Get the MCP Engine:** [mor-blogger-theme-editor-mcp](https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp)

*Note: This is strictly an opt-in feature. The core MorBlogger UI remains 100% offline, local, and free of AI telemetry by default. If the AI bridge toggle is off in your preferences, the editor remains completely blind to external processes.*
