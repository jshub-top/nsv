use std::{env, path::PathBuf};

use util::platform::Env::Shell;
pub fn get_shell_profile_path(shell: &Shell) -> PathBuf {
    match shell {
        Shell::Bash => PathBuf::from(env::var("HOME").unwrap()).join(".bashrc"),
        Shell::Zsh => PathBuf::from(env::var("HOME").unwrap()).join(".zshrc"),
        Shell::Sh => PathBuf::from(env::var("HOME").unwrap()).join(".bashrc"),
        Shell::Fish => PathBuf::from(env::var("HOME").unwrap())
            .join(".config/fish")
            .join("config.fish"),
        Shell::PowershellDesktop => PathBuf::from(env::var("USERPROFILE").unwrap())
            .join("Documents\\WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1"),
        Shell::PowershellCore => PathBuf::from(env::var("USERPROFILE").unwrap())
            .join("Documents\\PowerShell")
            .join("Microsoft.PowerShell_profile.ps1"),
        // Shell::Cmd => {
        //     panic!("cmd is not supported")
        // }
    }
}
pub fn get_nsv_config_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(env::var("USERPROFILE").unwrap()).join("Documents\\nsv")
    }

    #[cfg(unix)]
    {
        PathBuf::from(env::var("HOME").unwrap()).join(".config/nsv")
    }
}
