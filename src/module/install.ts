
import { join, resolve } from "path";
import { EOL } from "os";
import { system, shell, shellConfigFileDir } from "../../local.json"
import { ensureFileSync, readFile, writeFile } from "fs-extra";
import { context } from "../context"
export async function install () {
    let content = ""
    if ("NSV_HOME" in process.env) return console.log("nsv: installed")
    ensureFileSync(shellConfigFileDir)
    const shell_config_file_content = await readFile(shellConfigFileDir, { encoding: "utf-8" }).then(v => v.toString().split(EOL))
    // linux macos
    if (system === "linux" || system === "darwin") {
        shell_config_file_content.push(`export NSV_HOME=$HOME/.nsv`)
        shell_config_file_content.push(`[ -s "$NSV_HOME/nsv.sh" ] && . "$NSV_HOME/nsv.sh"`)
        shell_config_file_content.push(`export PATH=$NSV_HOME/local/node/bin:$PATH`)
        shell_config_file_content.push(``)
        content = `source ${shellConfigFileDir}`
    } else
    if (system === "win") {
        const home = context.get("dir").home
        shell_config_file_content.push(`$Env:NSV_HOME = "${home}"`)
        shell_config_file_content.push(`$Env:PATH = "$Env:NSV_HOME;$Env:NSV_HOME\\local\\node;$Env:PATH"`)
        shell_config_file_content.push("")
    }

    writeFile(shellConfigFileDir, shell_config_file_content.join(EOL), { encoding: "utf-8" })
    content && writeFile(join(context.get("dir").cache, context.get("temp_file_name")), content)
}

export async function uninstall () {
    ensureFileSync(shellConfigFileDir)
    let shell_config_file_content = await readFile(shellConfigFileDir, { encoding: "utf-8" }).then(v => v.toString().split(EOL).filter(v => !/NSV_HOME/.test(v)))
    writeFile(shellConfigFileDir, shell_config_file_content.join(EOL), { encoding: "utf-8" })
}
