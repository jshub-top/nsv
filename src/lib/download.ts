import { request, Agent } from "https";
import { ClientRequest } from "http"
import { RunStatus, context } from "../context";
import url from "url";
import { createWriteStream } from "fs-extra";

declare global {
    interface Context {
        download_request: ClientRequest|null
        download_temp_dir: string
    }
}


export interface DownloadFileCallback {
    total: number
    size: number
    current: number
    type: "start"|"update"|"end"
}

export function download(uri: string, save_dir: string, cb: (r: DownloadFileCallback) => void = () => {}): Promise<void> {
    const url_option = url.parse(uri)
    const proxy = context.get("proxy")
    if (proxy) {
        url_option["agent"] = Agent
        Agent["proxy"] = proxy
    }
    return new Promise((resolve, reject) => {

        const req = request(url_option, (res => {
            const file_total = +res.headers["content-length"]
            let current = 0
            cb({total: file_total, current, size: 0, type: "start"})
            res.on("data", (chunk) => {
                current += chunk.length
                cb({
                    total: file_total,
                    size: chunk.length,
                    current,
                    type: "update",
                })
            })
            res.on("close", () => {
                context.set("runStatus", RunStatus.normal)
                context.set("download_request", null)
                cb({
                    total: file_total,
                    size: 0,
                    current: file_total,
                    type: "end",
                })
            })
            res.on("end", resolve)
            res.on("error", reject)
            const file_write_stream = createWriteStream(save_dir)
            file_write_stream.on("error", reject)
            res.pipe(file_write_stream)
        }))
        req.end()
        context.set("runStatus", RunStatus.download)
        context.set("download_temp_dir", save_dir)
        context.set("download_request", req)

    })
}
