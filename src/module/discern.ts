
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

    const config_file_name = ".nsvrc"
    if (!existsSync(config_file_name)) return
    let version = ""
    if (config_file_name) version = readFileSync(config_file_name, {encoding: "utf-8"}).toString().trim()
    use(version)
}

function discern_enable() {
    user_db.setSync("discern", true)
    ensureFileSync(shellConfigFileDir)
    const profile_content = readFileSync(shellConfigFileDir, {encoding: "utf-8"})
    const discern_reg = /nsv discern/
    if (discern_reg.test(profile_content)) return console.log("nsv: discern enabled")
    const profile_content_list = profile_content.split(EOL)
    if (system === "win") profile_content_list.push(`if (Test-Path "$PWD/.nsvrc") { nsv discern }`)
    else if (system === "linux" || system === "darwin") profile_content_list.push(`[ -s "$PWD/.nsvrc" ] && nsv discern`)
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
