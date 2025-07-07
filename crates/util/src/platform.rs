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
        _ =>  Err(anyhow::anyhow!(format!("{}, unsupported operating OS", std::env::consts::OS))),
    }
}

pub fn get_arch() -> Result<Arch> {
    match std::env::consts::ARCH {
        "x86" => Ok(Arch::X86),
        "x86_64" => Ok(Arch::X64),
        "aarch64" => Ok(Arch::Arm64),
        _ => Err(anyhow::anyhow!(format!("{}, unsupported architecture", std::env::consts::ARCH))),
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
        nix::unistd::geteuid().is_root()
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
        Powershell,
        Cmd,
    }

    pub fn get_shell() -> Result<Shell> {

        println!("PSModulePath: {:?}", std::env::var("PSVersionTable"));

        #[cfg(target_os = "windows")]
        {
            use std::env;
            if env::var("PSModulePath").is_ok() {
                return Ok(Shell::Powershell);
            }

            if env::var("ComSpec").is_ok() && env::var("PROMPT").is_ok() {
                return Ok(Shell::Cmd);
            }
            return Err(anyhow::anyhow!("shell parsing failed"));
        }

        #[cfg(unix)]
        {
            use std::env;

            if env::var("SHELL").is_ok() {
                return Ok(Shell::Zsh);
            }
        }
    }
}
