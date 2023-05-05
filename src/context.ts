import config from "../config.json";
import { join } from "path";
import { get, set } from "lodash";
import * as process from "process";
declare global {
    interface Context {
        temp_file_name: string
        proxy: string,
        is_admin: boolean
        dir: {
            home: string
            cache: string
            node: string
            local: string
        }
    }
}

class ContextClass<T extends Object> {
    private readonly _cache: T

    constructor(data: any) {
        this._cache = data as T
    }


    public get<K extends keyof T>(key: K): T[K] {
        return get(this._cache, key)
    }

    public set<K extends keyof T>(key: K | string, data: any) {
        set(this._cache, key, data)
    }

}

function main_context() {
    const dir_home = process.env["NSV_HOME"] || join(__dirname, "../")
    const dir_cache = join(dir_home, config.path.cache)
    const dir_node = join(dir_home, config.path.node)
    const dir_local = join(dir_home, config.path.local)
    const is_admin = process.env["isAdmin"] === "True"
    const context_data = {
        is_admin,
        temp_file_name: process.env["NSV_TEMP_SCRIPT_NAME"],
        proxy: process.env["http_proxy"] || process.env["https_proxy"] || process.env["HTTP_PROXY"] || process.env["HTTPS_PROXY"] || "",
        dir: {
            home: dir_home,
            cache: dir_cache,
            node: dir_node,
            local: dir_local,
        }
    }
    return new ContextClass<Context>(context_data)
}
export const context = main_context()


