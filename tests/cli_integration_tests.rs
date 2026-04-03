/// CLI 命令集成测试

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    #[test]
    fn test_cli_help() {
        // 测试帮助命令
        let output = Command::new("cargo")
            .args(&["run", "--bin", "de", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Dual Engine CLI"));
        assert!(stdout.contains("run"));
        assert!(stdout.contains("switch"));
        assert!(stdout.contains("status"));
    }

    #[test]
    fn test_cli_status() {
        // 测试状态命令
        let output = Command::new("cargo")
            .args(&["run", "--bin", "de", "--", "status"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Current engine"));
        assert!(stdout.contains("Available engines"));
    }

    #[test]
    fn test_cli_version() {
        // 测试版本显示（无命令时）
        let output = Command::new("cargo")
            .args(&["run", "--bin", "de"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Dual-Engine"));
    }

    #[test]
    fn test_cli_invalid_engine() {
        // 测试无效引擎错误处理
        let output = Command::new("cargo")
            .args(&["run", "--bin", "de", "--", "switch", "invalid_engine"])
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Unknown engine"));
    }
}