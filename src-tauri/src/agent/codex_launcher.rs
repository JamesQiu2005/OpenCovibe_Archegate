use crate::agent::claude_stream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexCommand {
    pub program: String,
    pub args: Vec<String>,
    pub managed_by_aiflow: bool,
}

pub fn build_codex_command(tool_args: Vec<String>) -> CodexCommand {
    build_codex_command_for(
        claude_stream::which_binary("aiflow"),
        Some(claude_stream::resolve_codex_path()),
        tool_args,
    )
}

fn build_codex_command_for(
    aiflow_path: Option<String>,
    codex_path: Option<String>,
    tool_args: Vec<String>,
) -> CodexCommand {
    if let Some(program) = aiflow_path {
        let mut args = vec!["codex".to_string()];
        args.extend(tool_args);
        return CodexCommand {
            program,
            args,
            managed_by_aiflow: true,
        };
    }

    CodexCommand {
        program: codex_path.unwrap_or_else(|| "codex".to_string()),
        args: tool_args,
        managed_by_aiflow: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aiflow_launcher_prefixes_codex_tool_name() {
        let command = build_codex_command_for(
            Some("/usr/local/bin/aiflow".into()),
            Some("/usr/local/bin/codex".into()),
            vec!["app-server".into(), "--enable".into(), "feature".into()],
        );

        assert_eq!(command.program, "/usr/local/bin/aiflow");
        assert_eq!(
            command.args,
            vec!["codex", "app-server", "--enable", "feature"]
        );
        assert!(command.managed_by_aiflow);
    }

    #[test]
    fn direct_codex_is_the_fallback_when_aiflow_is_unavailable() {
        let command = build_codex_command_for(
            None,
            Some("/usr/local/bin/codex".into()),
            vec!["exec".into(), "--json".into()],
        );

        assert_eq!(command.program, "/usr/local/bin/codex");
        assert_eq!(command.args, vec!["exec", "--json"]);
        assert!(!command.managed_by_aiflow);
    }
}
