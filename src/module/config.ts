
import { existsSync, readJSONSync, writeJSONSync, writeJson } from "fs-extra"
import { get, set } from "lodash";
import { join } from "path";
import { context } from "../context";

export interface JSONDBConfig {
    sync?: true
    data?: Object
}

class JSONDB<T extends Object = Object> {
    private _db: T
    private _path: string
    private _config: JSONDBConfig = {sync: true, data: {}}
    constructor(path: string, config?: JSONDBConfig) {
        this._path = path
        this._config = {...this._config, ...config}
        this._db = this.init()
    }

    private init(): T {
        const is_exist_db = existsSync(this._path)
        if (!is_exist_db) {
            if (!this._config.sync) return
            const db_data = this._config.data || {}
            writeJSONSync(this._path, db_data)
        }
        return readJSONSync(this._path)
    }

    public sync() {
        writeJson(this._path, this._db)
    }

    public get(dir: string) {
        return get(this._db, dir)
    }

    public set(dir: string, data: any) {
        return set(this._db, dir, data)
    }

    public setSync(dir: string, data: any) {
        this.set(dir, data)
        this.sync()
    }
}

export const user_db = new JSONDB(join(context.get("dir").home, "user-config.json"), {
    data: {
        discern: false,
    }
})
