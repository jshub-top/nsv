import { writeJSONSync, readJSONSync, ensureDir, ensureFileSync, existsSync, readFileSync } from "fs-extra";
import { join, sep } from "path";
import { system_and_arch } from "./util/system"
import { version, mainNode } from "../package.json"
import { execSync } from "child_process"
import { EOL } from "os";

const [system, arch] = system_and_arch()
const home = join(__dirname, "../")
function set_local_env() {
    const ditc_system = {
        "win": () => "7z",
        "default": () => {
            const xz_msg = execSync("tar --help | grep xz").toString()
            const is_supper_xz = /J|xz/.test(xz_msg)
            return is_supper_xz && "tar.xz" || "tar.gz"
        }
    }

    const ditc_unzip_order = {
        "win": join(home, "tools/7-Zip/7zr.exe"),
        "default": "tar"
    }

    const ditc_temp_script_content = {
        "powershell": `
            $Env:Path = "{{ content }}"
            $Env:NSV_CURRENT_VERSION = "{{ current_version }}"
        `,
        "bash": `
            export PATH="{{ content }}"
            export NSV_CURRENT_VERSION="{{ current_version }}"
        `,
        "zsh": `
            export PATH="{{ content }}"
            export NSV_CURRENT_VERSION="{{ current_version }}"
        `,
        "fish": `
            set PATH "{{ content }}"
            set NSV_CURRENT_VERSION "{{ current_version }}"
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
        "default": "sudo ",
    }

    let shell = "powershell"
    let shellConfigFileDir = ""
    let shellTempOneOffFile = "nsv_temp_one_off_file"
    if (system === "win") {
        shellConfigFileDir = join(process.env.USERPROFILE, "Documents", "WindowsPowerShell", "Microsoft.PowerShell_profile.ps1")
        shellTempOneOffFile += ".ps1"
    } else
    if (system === "linux" || system === "darwin") {
        const shell_name = process.env.SHELL
        if (/bash/.test(shell_name)) {
            shell = "bash"
            shellConfigFileDir = ".bashrc"
            shellTempOneOffFile += ".sh"
        } else
        if (/zsh/.test(shell_name)) {
            shell = ".zshrc"
            shellConfigFileDir = ".zshrc"
            shellTempOneOffFile += ".sh"
        } else
        if (/fish/.test(shell_name)) {
            shell = "fish"
            shellConfigFileDir = ".config/fish/config.fish"
            shellTempOneOffFile += ".fish"
        }
        shellConfigFileDir = `${process.env.HOME}/${shellConfigFileDir}`
    }

    let prefix = ""
    const npmrc_file_dir = join(process.env["HOME"], ".npmrc")
    if (existsSync(npmrc_file_dir)) {
        const npmrc_content = readFileSync(npmrc_file_dir, {encoding: "utf-8"})
        if (/prefix/.test(npmrc_content)) prefix = npmrc_content.split(EOL).find(v => /prefix/.test(v)).split("=")[1]
    }



    const local = {
        version,
        system,
        arch,
        prefix,
        shell,
        shellConfigFileDir,
        shellTempOneOffFile,
        mainNode: mainNode[system]?.[arch] || mainNode["default"],
        remoteNodeFileExtension: ditc_system[system]?.() || ditc_system["default"](),
        unzipOrder: ditc_unzip_order[system] || ditc_unzip_order["default"],
        sudoShellContent: ditc_sudo_shell_content[system] || ditc_sudo_shell_content["default"],
        tempScriptContent: ditc_temp_script_content[shell],
        tempLocalScriptContent: ditc_temp_local_script_content[system] || ditc_temp_local_script_content["default"],
    }
    writeJSONSync(join(home, "./local.json"), local, {
        encoding: "utf-8"
    })
}

set_local_env()
