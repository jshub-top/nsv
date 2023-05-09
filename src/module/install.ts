
import { join, dirname } from "path";
import { EOL } from "os";
import { system, shell, shellConfigFileDir } from "../../local.json"
import { copyFile, ensureFileSync, readFileSync, writeFile, constants } from "fs-extra";
import { context } from "../context"
import { rm } from "shelljs";

const test_reg = /NSV_HOME|nsv discern/
export function install () {
    let content = ""
    const shellConfigFileStr = readFileSync(shellConfigFileDir, {encoding: "utf-8"})
    if (test_reg.test(shellConfigFileStr)) return console.log("nsv: installed")
    ensureFileSync(shellConfigFileDir)
    const shell_config_file_content = shellConfigFileStr.split(EOL)
    // linux macos
    if (system === "linux" || system === "darwin") {

        // fish
        if (shell === "fish") {
            shell_config_file_content.push(`set NSV_HOME $HOME/.nsv`)
            shell_config_file_content.push(`set PATH $NSV_HOME/local/node/bin $PATH`)
            copyFile(join(__dirname, "../../nsv.fish"), join(process.env["HOME"], ".config/fish/functions/nsv.fish"))
        } else {
            shell_config_file_content.push(`export NSV_HOME=$HOME/.nsv`)
            shell_config_file_content.push(`[ -s "$NSV_HOME/nsv.sh" ] && . "$NSV_HOME/nsv.sh"`)
            shell_config_file_content.push(`export PATH=$NSV_HOME/local/node/bin:$PATH`)
        }
        
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

export function uninstall () {
    ensureFileSync(shellConfigFileDir)
    let shell_config_file_content = readFileSync(shellConfigFileDir, { encoding: "utf-8" }).toString().split(EOL).filter(v => !test_reg.test(v))
    writeFile(shellConfigFileDir, shell_config_file_content.join(EOL), { encoding: "utf-8" })
    if(shell === "fish") {
        rm(join(dirname(shellConfigFileDir), "functions", "nsv.fish"))
    }
}
