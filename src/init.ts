import { writeJSONSync, readJSONSync } from "fs-extra";
import { join } from "path";
import { system_and_arch } from "./lib/system"
import { context } from "./context"
import { exec } from "child_process"
import { version } from "../package.json"


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
        "win": `$Env:Path = "{{ content }}"`,
        "default": `export PATH="{{ content }}"`
    }

    let shell = "powershell"
    let shellConfigFileDir = ""
    if (system === "linux") {
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
        remoteNodeFileExtension: ditc_system[system] || ditc_system["default"],
        unzipOrder: ditc_unzip_order[system] || ditc_unzip_order["default"],
        tempScriptContent: ditc_temp_script_content[system] || ditc_temp_script_content["default"]
    }
    writeJSONSync(join(home, "./local.json"), local, {
        encoding: "utf-8"
    })
}


function run_init_shell() {
    if (process.env["GITHUB_ENV"] !== void 0) return
    const ditc_init_shell = {
        win: (dir: string) => {
            return `Powershell ${dir}/nsv.ps1 install`
        },
        default: (dir: string) => {
            return `${dir}/nsv.sh install`
        }
    }
    const exec_order = ditc_init_shell[system]?.(home) || ditc_init_shell["default"](home)
    const cp = exec(exec_order)
    cp.stdout.pipe(process.stdout)
    cp.stderr.pipe(process.stdout)

}

set_local_env()
run_init_shell()
