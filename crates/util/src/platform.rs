use std::path::PathBuf;

use anyhow::Result;

pub enum OS {
    Windows,
    Linux,
    MacOS,
    Unknown(String),
}

pub enum Arch {
    X86,
    X64,
    Arm64,
    Unknown(String),
}

pub fn get_os() -> Result<OS> {
    match std::env::consts::OS {
        "windows" => Ok(OS::Windows),
        "linux" => Ok(OS::Linux),
        "macos" => Ok(OS::MacOS),
        _ => Err(anyhow::anyhow!(format!(
            "{}, unsupported operating OS",
            std::env::consts::OS
        ))),
    }
}

pub fn get_arch() -> Result<Arch> {
    match std::env::consts::ARCH {
        "x86" => Ok(Arch::X86),
        "x86_64" => Ok(Arch::X64),
        "aarch64" => Ok(Arch::Arm64),
        _ => Err(anyhow::anyhow!(format!(
            "{}, unsupported architecture",
            std::env::consts::ARCH
        ))),
    }
}

pub fn get_is_admin() -> bool {
    #[cfg(target_os = "windows")]
    {
        use is_elevated::is_elevated;
        is_elevated()
    }

    #[cfg(unix)]
    {
        false
    }
}

pub mod Env {
    use anyhow::Result;

    #[derive(Debug)]
    pub enum Shell {
        Sh,
        Zsh,
        Bash,
        Fish,
        PowershellDesktop,
        PowershellCore,
        // Cmd,
    }

    pub fn get_powershell_version() -> Result<u32> {
        use std::process::Command;
        let output = Command::new("powershell")
            .args(&[
                "-Command",

                "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Write-Host \"-->$($PSVersionTable.PSVersion.Major)<--\"",
            ])
            .output()
            .unwrap();
        if output.status.code().unwrap() != 0 {
            return Err(anyhow::anyhow!("powershell version parsing failed"));
        }

        let stdout = String::from_utf8(output.stdout).unwrap();
        let version = stdout
            .split("-->")
            .nth(1)
            .ok_or(anyhow::anyhow!("powershell version parsing failed"))?
            .split("<--")
            .next()
            .map(|s| s.to_string())
            .ok_or(anyhow::anyhow!("powershell version parsing failed"))?;

        Ok(version.parse::<u32>()?)
    }

    pub fn get_shell() -> Result<Shell> {
        #[cfg(target_os = "windows")]
        {


            let ps_version = get_powershell_version();
            if ps_version.is_err() {
                return Ok(Shell::PowershellDesktop);
                // return Ok(Shell::Cmd);
            }

            let version = ps_version.unwrap();

            match version {
                version if version <= 5 => return Ok(Shell::PowershellDesktop),
                version if version >= 6 => return Ok(Shell::PowershellCore),
                _ => return Ok(Shell::PowershellDesktop),
            }

        }

        #[cfg(unix)]
        {
            use std::env;
            use std::path::Path;

            let shell = env::var("SHELL").unwrap();
            let shell_path = Path::new(&shell);
            let shell_name = shell_path.file_name().unwrap().to_str().unwrap();
            match shell_name {
                "zsh" => return Ok(Shell::Zsh),
                "bash" => return Ok(Shell::Bash),
                "fish" => return Ok(Shell::Fish),
                "sh" => return Ok(Shell::Sh),
                _ => return Err(anyhow::anyhow!("shell parsing failed")),
            }
        }

    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        Ok(PathBuf::from(std::env::var("USERPROFILE")?))
    }

    #[cfg(unix)]
    {
        Ok(PathBuf::from(std::env::var("HOME")?))
    }
}

pub fn get_powershell_version() {
    use std::process::Command;

    let output = Command::new("cmd").args(&["/c", "321aa"]).output().unwrap();

    println!("code: {:?}", output.status.code().unwrap());
    println!("stdout: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {:?}", String::from_utf8(output.stderr).unwrap());
}
