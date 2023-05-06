import { memo, regex_vanilla_string } from "../util";
import { context } from "../context";
import { readdirSync, statSync } from "fs-extra";
import { source, version } from "../../config.json";
import { get } from "./http";
import { system, arch, unzipOrder } from "../../local.json";
import { spawn, exec, ChildProcessWithoutNullStreams } from "child_process";
import { delimiter, sep } from "path";
export interface RemoteNodeVersion {
    version: string
    files: string[]
    lts: string | boolean
    security: boolean
}

export interface UnzipFileInfoCallback {
    current: number
    total: number
    type: "start"|"end"|"update"
}

export const version_regexp = "\\d+\\.\\d+\.\\d+"
export const version_regexp_test = regex_vanilla_string(new RegExp(version_regexp))

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
    return local_version_list.filter(version_regexp_test)
}

export const get_current_node_version = () => {
    const path = process.env["PATH"]
    const first_path = path.split(delimiter)[0]
    let current_version = ""
    const env_current_version = process.env["NSV_CURRENT_VERSION"]
    if (env_current_version && new RegExp(env_current_version).test(first_path)) current_version = env_current_version
    return current_version
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
            return spawn(unzipOrder, ["-xvf", file_dir, "-C", output_dir])
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
            cb({ total, current, type: "end" })
            resolve({ total, current: total, type: "end" })
        })
        cp.stdout.on("error", reject)

    })

}
