
import { join } from "path";
import { context } from "../context";
import { remote_version_list } from "../lib/version"
import { format_shell_content, set_temp_shell, format_node_path } from "../lib/env"
import { source } from "../../config.json"
import { remoteNodeFileExtension, tempScriptContent, tempLocalScriptContent } from "../../local.json"
import { download } from "../lib/download"
import { existsSync, readdirSync, removeSync, renameSync, emptyDirSync } from "fs-extra"
import { progress } from "../lib/progress"
import { unzip_file } from "../lib/version"

export async function use(version: string) {
    let use_version = use_path_node_version(version)
    if (!use_version) {
        await use_remote_node_version(version)
        use_version = use_path_node_version(version)
        if ( use_version === void 0 ) throw new Error(`use path node version ${use_version} error. please push issuse to https://github.com/1739616529/nsv/issues/new`)
    }
    console.log(`v${use_version}`)
}

export async function local(version: string) {
    let use_version = use_local_node_version(version)
    if (!use_version) {
        await use_remote_node_version(version)
        use_version = use_local_node_version(version)
        if ( use_version === void 0 ) throw new Error(`use local node version ${use_version} error. please push issuse to https://github.com/1739616529/nsv/issues/new`)
    }
    console.log(`v${use_version}`)
}
export function test_local_node_version(version: string): [ string, boolean ] {
    if (version[0] === "v") version = version.substring(1, version.length)
    const regex = new RegExp(`^${ version }`)
    const local_version_list = readdirSync(context.get("dir").node)
    const local_version = Object.values(local_version_list).find(v => regex.test(v)) || ""
    return [ local_version as string, !!local_version ]
}


export function use_path_node_version(version: string): string {
    const [ local_version, is_have ] = test_local_node_version(version)
    if ( !is_have ) return
    const path_list = format_node_path(local_version)
    const content = format_shell_content(tempScriptContent, {
        content: path_list
    })
    set_temp_shell(content)
    return local_version

}

export function use_local_node_version(version: string): string {
    const [ local_version, is_have ] = test_local_node_version(version)
    if ( !is_have ) return
    const { node, local, home } = context.get("dir")
    const local_node_abs_dir = join(node, local_version)
    const output_dir = join(local, "node")
    const content = format_shell_content(tempLocalScriptContent, {
        target: local_node_abs_dir,
        output: output_dir,
    })
    set_temp_shell(content, true)
    emptyDirSync(local)
    return local_version
}


export async function use_remote_node_version(version: string) {
    const remote_node_list = await remote_version_list()
    const { cache, node } = context.get("dir")
    const active_node = remote_node_list.find(v => v.matchVersion(version))
    const remote_node_file_name = `${active_node.remoteFileName}.${remoteNodeFileExtension}`
    const download_url = `${source.download}/v${active_node.version}/${remote_node_file_name}`
    const save_dir = `${cache}/${remote_node_file_name}`
    if (!existsSync(save_dir)) {
        await download(download_url, save_dir, progress("Downloading".padEnd(12, " "))).catch((err) => {
            removeSync(save_dir)
            throw new Error(err.toString())
        })
    }

    const node_abs_dir = join(node, active_node.version)
    if (!existsSync(node_abs_dir)) {
        await unzip_file(save_dir, context.get("dir").node, progress("Extracting".padEnd(12, " ")))
    }
    renameSync(join(node, active_node.remoteFileName), node_abs_dir)
}

