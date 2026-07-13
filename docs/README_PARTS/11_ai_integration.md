## 🤖 AI & LLM Integration (Strictly Opt-In)

![MCP integration architecture](docs/diagrams/mcp_integration.drawio.png)

For developers and power users who want AI assistance without embedding a runtime inside the GUI, we maintain a standalone, headless MCP (Model Context Protocol) server in a separate repo.

By running the **MorBlogger MCP Engine**, you can connect your CLI agent (like Antigravity or Grok) or desktop IDE directly to MorBlogger core. The AI can read your theme manifests, respect structural constraints, and hot-reload your active UI workspace when the AI bridge toggle is on.

Communication between the UI and the MCP server uses a local file-drop bridge (see `restore_drop_bridge` in the Dioxus app). Install plugins via `mbt inject` or the in-app Plugin Manager.

🔗 **Get the MCP Engine:** [mor-blogger-theme-editor-mcp](https://github.com/MoribundInstitute/mor-blogger-theme-editor-mcp)

*Note: This is strictly opt-in. The core MorBlogger UI is offline and local by default. If the AI bridge toggle is off in your preferences, the editor ignores external MCP processes.*