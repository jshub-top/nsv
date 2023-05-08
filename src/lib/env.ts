import { context } from "../context"
import { writeFileSync } from "fs-extra";
import { join, sep, delimiter } from "path";
import process from "process";
import { system, sudoShellContent } from "../../local.json"
import { get_current_node_version } from "./version"
// declare global {
//     interface Context {
//         path: {
//             user: string
//             system: string
//         },
//     }
// }
// context.set("path", {
//     user: process.env["NSV_USER_PATH"],
//     system: process.env["NSV_SYSTEM_PATH"],
// })



export function set_temp_shell(content: string, sudo = false): boolean {
    const temp_file_dir = join(context.get("dir").cache, context.get("temp_file_name"))
    if (sudo) content = sudoShellContent + content
    try {
        writeFileSync(temp_file_dir, content, { encoding: "utf-8" })
        return true
    } catch (err) {
        return false
    }
}

export function format_shell_content(content: string, obj: Object) : string {
    for (const key in obj) {
        content = content.replace(`{{ ${key} }}`, obj[key])
    }
    return content
}

export function format_node_path(version: string) {
    const ditc_node_path = {
        win: (dir: string) => dir,
        default: (dir: string) => `${dir}/bin`
    }
    const { home, node } = context.get("dir")
    const local_node_abs_dir = join(node, version)
    const path = process.env["PATH"]
    const path_list = path.split(delimiter)
    if (get_current_node_version()) path_list.shift()
    const format_dir_fun = ditc_node_path[system] || ditc_node_path["default"]
    path_list.unshift(format_dir_fun(local_node_abs_dir))
    return path_list.join(delimiter)
}
