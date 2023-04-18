// import { request, RequestOptions, globalAgent } from "http";
import { request, RequestOptions, globalAgent } from "https";
import { context } from "../context";
import url from "url";
export function get<T>(uri: string,) {
    const url_option = url.parse(uri)
    return req<T>(url_option)
}


function req<T>(option: RequestOptions | url.URL): Promise<{ code: number, data: T }> {
    const proxy = context.get("proxy")
    if (proxy) {
        option["agent"] = globalAgent
        globalAgent["proxy"] = proxy
    }
    return new Promise((resolve, reject) => {
        request(option, (res) => {
            let data = ""
            const code = res.statusCode
            res.on("data", (chunk) => {
                data += chunk
            })
            res.on("end", () => {
                resolve({ code, data: JSON.parse(data) })
            })
            res.on("error", reject)
        }).end()
    })
}
