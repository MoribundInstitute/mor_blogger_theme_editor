#!/usr/bin/env bash
# Minimal MCP stdio stub for Mission 5 extensibility testing.
set -euo pipefail

SYSTEM_PROMPT="You are the MorBlogger Layout QA daemon. Audit theme XML for Blogger namespace integrity, flag missing <b:includable> blocks, and never suggest deploying broken templates."

while IFS= read -r line; do
  case "$line" in
    *initialize*)
      printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"mission5_echo_mcp","version":"0.1.0"},"instructions":"'"${SYSTEM_PROMPT//\"/\\\"}"'"}}'
      ;;
    *tools/list*)
      printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"echo_layout_task","description":"Returns the injected Mission 5 automated layout QA task.","inputSchema":{"type":"object","properties":{}}}]}}'
      ;;
    *tools/call*)
      printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"MISSION5_TASK: Run diagnostics analyzer before export."}]}}'
      ;;
    *)
      printf '%s\n' '{"jsonrpc":"2.0","id":0,"result":{}}'
      ;;
  esac
done