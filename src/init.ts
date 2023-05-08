import { writeJSONSync, readJSONSync, ensureDir } from "fs-extra";
import { join } from "path";
import { system_and_arch } from "./lib/system"
import { context } from "./context"
import { version, mainNode } from "../package.json"


const [system, arch] = system_and_arch()
const {home} = context.get("dir")

function set_local_env() {
    const ditc_system = {
        "win": "7z",
        "default": "tar.xz"
    }

    const ditc_unzip_order = {
        "win": join(home, "tools/7-Zip/7zr.exe"),
        "default": "tar"
    }

    const ditc_temp_script_content = {
        "win": `
            $Env:Path = "{{ content }}"
            $Env:NSV_CURRENT_VERSION = "{{ current_version }}"
        `,
        "default": `
            export PATH="{{ content }}"
            export NSV_CURRENT_VERSION="{{ current_version }}"
        `
    }

    const ditc_temp_local_script_content = {
        "win": `New-Item -ItemType SymbolicLink -Value "{{ target }}" -Path "{{ output }}"`,
        "default": `ln -s "{{ target }}" "{{ output }}"`
    }

    const ditc_sudo_shell_content = {
        "win": `$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")

            if (-not $isAdmin) {
                Start-Process powershell.exe "-File $PSCommandPath" -Verb RunAs
                Exit 0
            }
            Read-Host "debug"
        `,
        "default": "sudo",
    }

    let shell = "powershell"
    let shellConfigFileDir = ""
    if (system === "win") {
        shellConfigFileDir = join(process.env.USERPROFILE, "Documents/WindowsPowerShell", "Microsoft.PowerShell_profile.ps1")
    } else
    if (system === "linux" || system === "darwin") {
        const shell_name = process.env.SHELL
        if (/bash/.test(shell_name)) {
            shell = "bash"
            shellConfigFileDir = ".bashrc"
        } else
        if (/zsh/.test(shell_name)) {
            shell = ".zshrc"
            shellConfigFileDir = ".zshrc"
        } else
        if (/fish/.test(shell_name)) {
            shell = "fish"
            shellConfigFileDir = ".config/fish/config.fish"
        }
        shellConfigFileDir = `${process.env.HOME}/${shellConfigFileDir}`
    }

    const local = {
        version,
        system,
        arch,
        shell,
        shellConfigFileDir,
        mainNode: mainNode[system]?.[arch] || mainNode["default"],
        remoteNodeFileExtension: ditc_system[system] || ditc_system["default"],
        unzipOrder: ditc_unzip_order[system] || ditc_unzip_order["default"],
        sudoShellContent: ditc_sudo_shell_content[system] || ditc_sudo_shell_content["default"],
        tempScriptContent: ditc_temp_script_content[system] || ditc_temp_script_content["default"],
        tempLocalScriptContent: ditc_temp_local_script_content[system] || ditc_temp_local_script_content["default"],
    }
    writeJSONSync(join(home, "./local.json"), local, {
        encoding: "utf-8"
    })
}

set_local_env()
