import { context } from "../context"
import { writeFileSync } from "fs-extra";
import { join, sep, delimiter } from "path";
import process from "process";
import { tempScriptContent } from "../../local.json"
declare global {
    interface Context {
        path: {
            user: string
            system: string
        },
    }
}
context.set("path", {
    user: process.env["NSV_USER_PATH"],
    system: process.env["NSV_SYSTEM_PATH"],
})



export function set_temp_shell(content: string): boolean {
    const temp_file_dir = join(context.get("dir").cache, context.get("temp_file_name"))
    try {
        writeFileSync(temp_file_dir, content, { encoding: "utf-8" })
        return true
    } catch (err) {
        return false
    }
}

export function get_temp_shell_content() {
    return tempScriptContent
}

export function format_shell_content(content: string, obj: Object) : string {
    for (const key in obj) {
        content = content.replace(`{{ ${key} }}`, obj[key])
    }
    return content
}

export function format_