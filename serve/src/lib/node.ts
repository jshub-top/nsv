import { stat, exists, readJSON, writeJSON } from "fs-extra"
import { request, get } from "http"
import { join } from "path"
import { combineArrays, download_file } from "../util"
import { arch } from "os";
import dayjs from "dayjs";


export interface SyncOption {
    origin?: string, // 镜像地址
    version?: string[], // 版本
    arch?: "x64" | "arm64", // cpu类型
    system?: "win" | "linux" | "darwin", // 操作系统类型
    static_dir?: string // 静态资源目录
}

export type SyncOptionTrue = MinusOpt<SyncOption>

type MinusOpt<T> = {
    [K in keyof T]-?: T[K]
}

type ArrayToType<T> = T extends readonly (infer U)[] ? U : never;

type CharToObj<T extends string, U> = {
    [K in T]: U
}

export const sync_node_mirror = async function (
    option: SyncOption = {}
) {

    const _option: SyncOptionTrue = {
        origin: "https://nodejs.org", // 镜像地址
        version: ["v21.5.0", "v20.10.0"], // 版本
        arch: arch() as SyncOptionTrue["arch"], // cpu类型
        system: process.platform as SyncOptionTrue["system"], // 系统类型
        static_dir: join(__dirname, "../../dist"), // 静态资源目录
        ...option,
    }

    sync_node_version_json(_option)
    sync_node_version_file(_option)
}


export const sync_node_version_json = async function (
    { origin, static_dir }: SyncOptionTrue
) {

    const json_dir = join(static_dir, "index.json")

    const json_exists = await exists(json_dir)

    // 如果存在
    if (json_exists) {
        const json_info = await stat(json_dir)

        // 如果最后修改日期大于12小时 重新请求并写入
        if ((Date.now() - json_info.mtimeMs) < 1000 * 60 * 60 * 12) {
            console.log(`node version json file latest update time is: ${dayjs(json_info.mtime).format("YYYY-MM-DD HH:mm:ss")}`)
            return
        }
    }

    const node_version_url = `${origin}/dist/index.json`
    const node_version_json = await fetch(node_version_url).then(res => res.json())

    await writeJSON(json_dir, node_version_json, { encoding: "utf-8" })
}


export const sync_node_version_file = async function (
    { origin, system, arch, version, static_dir }: SyncOptionTrue
) {

    const ditc_system_file_extends: CharToObj<SyncOptionTrue["system"], string> = {
        "win": "7z",
        "darwin": "tar.xz",
        "linux": "tar.xz"
    }
    const sync_node = async function (
        version: ArrayToType<SyncOptionTrue["version"]>,
        arch: ArrayToType<SyncOptionTrue["arch"]>,
        system: ArrayToType<SyncOptionTrue["system"]>,
    ) {

        const file_name = `node-${version}-${system}-${arch}.${ditc_system_file_extends[system]}`

        // https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-x64.tar.xz
        const node_url = `${origin}/dist/${version}/${file_name}`

        const node_file_dir = join(static_dir, version, file_name)

        const shar256_url = `${origin}/dist/${version}/SHASUMS256.txt`
        const shar256_filr_dir = join(static_dir, version, "SHASUMS256.txt")


        if (! await exists(node_file_dir)) {

            console.log(`node ${version} downloading`)
            await download_file(
                node_url,
                node_file_dir,
            )
            console.log(`node ${version} downloded`)
        }

        if (! await exists(shar256_filr_dir)) {

            console.log(`node sha256 ${version} downloading`)
            await download_file(
                shar256_url,
                shar256_filr_dir,
            )
            console.log(`node sha256 ${version} downloded`)
        }



    }

    const version_info = combineArrays([
        version,
        [arch],
        [system],
    ])
    version_info.map(([version, arch, system]) => {
        sync_node(version, arch as ArrayToType<SyncOptionTrue["arch"]>, system as ArrayToType<SyncOptionTrue["system"]>)
    })
}

