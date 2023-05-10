import { memo, regex_vanilla_string } from "../util";
import { RunStatus, context } from "../context";
import { readdirSync, statSync } from "fs-extra";
import { source, version } from "../../config.json";
import { get } from "./http";
import { system, arch, unzipOrder, mainNode } from "../../local.json";
import { spawn, exec, ChildProcessWithoutNullStreams, ChildProcess  } from "child_process";
import { delimiter, sep } from "path";
import { compare, validate } from "compare-versions";
import { which } from "shelljs";

declare global {
    interface Context {
        extract_process: ChildProcess|null
        extract_dir: string
    }
}
export interface RemoteNodeVersion {
    version: string
    files: string[]
    lts: string | boolean
    security: boolean
}

export interface VersionCommandOption {
    remove?: boolean
}

export interface UnzipFileInfoCallback {
    current: number
    total: number
    type: "start"|"end"|"update"
}

export const version_reg = /\d+\.\d+\.\d+/
export const remote_version_list = (remote_url: string = source.version) => {
    return memo("remote_version_list", async () => {
        const { data } = await get<RemoteNodeVersion[]>(remote_url)
        const ret: NodeVersion[] = []
        data.some((v, i) => {
            if (i > version.len) return true
            ret.push(new NodeVersion(v))
        })
        return ret
    })
}

export const get_local_node_version_list = () => {
    const local_version_list = readdirSync(context.get("dir").node)
    return local_version_list.filter(validate)
}

export const get_current_node_version = () => {
    const path = process.env["PATH"]
    const first_path = path.split(delimiter)[0]
    let current_version = ""
    if (new RegExp(context.get("dir").node).test(first_path)) current_version = git_version_by_dir(first_path)
    return current_version
}


export function git_version_by_dir(dir: string) {
    const dir_arr = dir.split(sep)
    return dir_arr.find(v => version_reg.test(v))
}

export class NodeVersion {
    private _vs: RemoteNodeVersion

    private _version: string
    public get version () {
        if (this._version) return this._version
        return this._version = this._vs.version.replace("v", "")
    }

    private _title = ""
    public get title () {
        if (this._title) return this._title
        const { version, lts } = this._vs
        let title = version
        if (lts !== false) title += ` -- ${lts}`
        return this._title = title
    }

    private _remoteFileName = ""
    public get remoteFileName() {
        if (this._remoteFileName) return this._remoteFileName
        return this._remoteFileName = `node-${this._vs.version}-${system}-${arch}`
    }

    constructor( vs: RemoteNodeVersion )  {
        this._vs = vs
    }


    public matchVersion( version: string ) {
        if (version[0] === "v") return this._vs.version.startsWith(version)
        return this.version.startsWith(version)
    }

}


export function unzip_file(file_dir: string, output_dir: string, cb: (info: UnzipFileInfoCallback) => void = () => {}): Promise<UnzipFileInfoCallback> {
    const ditc_unzip = {
        "win": () => {

            return spawn(unzipOrder, ["x", "-bb1", `-o${output_dir}`, '-y', file_dir])
        },
        "default": () => {
            return exec(`${unzipOrder} -xvf ${file_dir} -C ${output_dir}`)
        }
    }
    return new Promise((resolve, reject) => {
        const total = statSync(file_dir).size
        const cp: ChildProcessWithoutNullStreams = ditc_unzip[system]?.() || ditc_unzip["default"]()
        let current = 0
        cb({total, current, type: "start"})
        cp.stdout.on("data", (chunk) => {
            current += chunk.length * 98
            cb({ total, current, type: "update" })
        })

        cp.stdout.on("close", () => {
            context.set("runStatus", RunStatus.normal)
            cb({ total, current, type: "end" })
            resolve({ total, current: total, type: "end" })
        })
        cp.stdout.on("error", reject)
        
        context.set("runStatus", RunStatus.extract)
        context.set("extract_dir", output_dir)
        context.set("extract_process", cp)
    })
}


export function is_le_mine_node_version(version: string): boolean {
    return compare(mainNode, version, "<=")
}

export function check_valid_version(version: string, cb: (...args: any[]) => any) {
    console.log(version)

}
export function vaild_version(version: string, option: VersionCommandOption, version_cb: (...args: any) => void, option_cb: (...args: any[]) => void) {

    // remove
    if (option.remove) {
        option_cb(version, option)


    } else
    {
        const _version = version.replace("v", "")

        const is_ok = validate(_version)
        if (!is_ok) throw new Error("nsv: Please enter the valid version number.")

        if (!is_le_mine_node_version(_version)) throw new Error(`nsv: Your computer does not support the lower version.  low version is: ==> ${mainNode} <==`)
        context.set("currentVersion", _version)
        version_cb(_version)
    }



}
