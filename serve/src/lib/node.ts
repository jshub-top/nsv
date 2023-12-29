import { stat, exists, readJSON, writeJSON } from "fs-extra"
import { request } from "http"
import { join } from "path"



export interface SyncOption {
    origin?: string, // 镜像地址
    version?: string[], // 版本
    arch?: readonly ["x64", "arm64"], // cpu类型
    system?: readonly ["win", "linux", "darwin"], // 操作系统类型

}

export type SyncOptionTrue = MinusOpt<SyncOption>

type MinusOpt<T> = {
    [K in keyof T]-?: T[K]
}

type ArrayToType<T> = T extends readonly (infer U)[] ? U : never;

type ArrayToObj<T extends readonly string[], U> = {
    [K in T[number]]: U
}

export const sync_node_mirror = async function (
    option: SyncOption = {}
) {

    const _option: SyncOptionTrue = {
        origin: "https://nodejs.org", // 镜像地址
        version: ["v21.5.0", "v20.10.0"], // 版本
        arch: ["x64", "arm64"], // cpu类型
        system: ["win", "linux", "darwin"], // 系统类型
        ...option,
    }

    const { origin } = _option

    sync_node_version_json(origin)
    sync_node_version_file(_option)
}


export const sync_node_version_json = async function (
    node_origin: SyncOption["origin"]
) {

    const json_dir = join(__dirname, "../../dist/index.json")

    const json_exists = await exists(json_dir)

    // 如果存在
    if (json_exists) {
        const json_info = await stat(json_dir)

        // 如果最后修改日期大于12小时 重新请求并写入
        if ((Date.now() - json_info.mtimeMs) < 1000 * 60 * 60 * 12) return
    }

    const node_version_url = `${node_origin}/dist/index.json`
    const node_version_json = await fetch(node_version_url).then(res => res.json)

    await writeJSON(json_dir, node_version_json, { encoding: "utf-8" })
}


export const sync_node_version_file = async function (
    { origin, system, arch, version }: SyncOptionTrue
) {

    const ditc_system_file_extends: ArrayToObj<SyncOptionTrue["system"], string> = {
        "win": "7z",
        "darwin": "tar.xz",
        "linux": "tar.xz"
    }
    const sync_node = function (
        version: ArrayToType<SyncOptionTrue["version"]>,
        arch: ArrayToType<SyncOptionTrue["arch"]>,
        system: ArrayToType<SyncOptionTrue["system"]>,
    ) {
        // https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-x64.tar.xz
        const url = `${origin}/dist/${version}/node-${version}-${system}-${arch}.${ditc_system_file_extends[system]}`


        console.log(url)
    }

    [
        version,
        arch,
        system,
    ].

}
