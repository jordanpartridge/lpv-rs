//! Mirror of `laravel/agent-detector` as bundled in the lean-package-validator
//! 6.3.2 phar, used to switch to JSON output when run by a coding agent.

const AGENT_ENV_VARS: &[&str] = &[
    "CURSOR_AGENT",
    "GROK_PLUGIN_ROOT",
    "GROK_PLUGIN_DATA",
    "GEMINI_CLI",
    "CODEX_SANDBOX",
    "CODEX_CI",
    "CODEX_THREAD_ID",
    "CODEX_SANDBOX_NETWORK_DISABLED",
    "AUGMENT_AGENT",
    "OPENCODE_CLIENT",
    "OPENCODE",
    "AMP_CURRENT_THREAD_ID",
    "CLAUDECODE",
    "CLAUDE_CODE",
    "REPL_ID",
    "COPILOT_MODEL",
    "COPILOT_ALLOW_ALL",
    "COPILOT_GITHUB_TOKEN",
    "COPILOT_CLI",
    "ANTIGRAVITY_AGENT",
    "ANTIGRAVITY_CLI_ALIAS",
    "PI_CODING_AGENT",
    "MATTERHORN_SESSION_ID",
    "JUNIE_DATA",
    "JUNIE_SHIM_PATH",
    "KIRO_AGENT_PATH",
    "KIMI_PLUGIN_ROOT",
    "CLINE_ACTIVE",
    "GOOSE_TERMINAL",
    "OPENCLAW_SHELL",
];

fn var(name: &str) -> Option<String> {
    std::env::var_os(name).map(|v| v.to_string_lossy().into_owned())
}

pub fn is_agentic_run() -> bool {
    if var("AI_AGENT").is_some_and(|v| !v.trim().is_empty()) {
        return true;
    }
    if AGENT_ENV_VARS
        .iter()
        .any(|name| std::env::var_os(name).is_some())
    {
        return true;
    }
    std::path::Path::new("/opt/.devin").exists()
}
