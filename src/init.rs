pub enum Shell {
    Bash,
    Powershell,
}

impl Shell {
    pub fn detect() -> Self {
        if cfg!(target_os = "windows") {
            Shell::Powershell
        } else {
            Shell::Bash
        }
    }
}

pub fn snippet(shell: &Shell) -> &'static str {
    match shell {
        Shell::Bash => r#"# Add the following to ~/.bashrc or ~/.bash_profile:
function l() {
    local output
    output=$(linker "$@")
    local code=$?
    if [ $code -eq 2 ]; then
        cd "$output"
    elif [ $code -eq 0 ] && [ -n "$output" ]; then
        echo "$output"
    elif [ $code -ne 0 ]; then
        echo "$output" >&2
        return 1
    fi
}
"#,
        Shell::Powershell => r#"# Add the following to your PowerShell profile ($PROFILE):
function l {
    $result = & linker @args
    $code = $LASTEXITCODE
    if ($code -eq 2) {
        Set-Location ($result -join "")
    } elseif ($code -eq 0) {
        $result
    } else {
        $result | ForEach-Object { Write-Error $_ }
    }
}
"#,
    }
}
