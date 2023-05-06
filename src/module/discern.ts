
import { context } from "../context"
import { ensureFileSync, existsSync, readFileSync, readJsonSync, writeFileSync } from "fs-extra"
import { use } from "./version"
import { user_db } from "./config"
import { system, shellConfigFileDir } from "../../local.json"
import { join } from "path"
import { EOL } from "os"
export function discern(opt: any) {

    if ("status" in opt) {
        console.log("discern enable: ", user_db.get("discern"))
        return
    }

    if ("enable" in opt) {
        discern_enable()
        return
    }

    if ("disable" in opt) {
        discern_disable()
        return
    }

    const is_node_project = existsSync("package.json");
    if (is_node_project === false) return
    const config_file_name_list = [".nsvrc", ".nvmrc"]
    const config_file_name = config_file_name_list.find(existsSync)
    let version = ""
    if (config_file_name) version = readFileSync(config_file_name, {encoding: "utf-8"}).toString().trim()
    else version = readJsonSync("package.json")?.nsv?.version || ""
    if (!version) return
    use(version)

}

function discern_enable() {
    user_db.setSync("discern", true)
    ensureFileSync(shellConfigFileDir)
    const profile_content = readFileSync(shellConfigFileDir, {encoding: "utf-8"})
    const discern_reg = /nsv discern/
    if (discern_reg.test(profile_content)) return console.log("nsv: discern enabled")
    const profile_content_list = profile_content.split(EOL)
    if (system === "win") profile_content_list.push(`if (Path-Test "$PWD/package.json") { nsv discern }`)
    else if (system === "linux" || system === "darwin") profile_content_list.push(`if [ -f "$PWD/package.json" ]; then . nsv discern; fi`)
    writeFileSync(shellConfigFileDir, profile_content_list.join(EOL), {encoding: "utf-8"})

}
function discern_disable() {
    user_db.setSync("discern", false)
    ensureFileSync(shellConfigFileDir)
    const profile_content = readFileSync(shellConfigFileDir, {encoding: "utf-8"})
    const discern_reg = /nsv discern/
    if (!discern_reg.test(profile_content)) return console.log("nsv: discern disabled")
    const profile_content_list = profile_content.split(EOL).filter(v => !discern_reg.test(v))
    writeFileSync(shellConfigFileDir, profile_content_list.join(EOL), {encoding: "utf-8"})
}
