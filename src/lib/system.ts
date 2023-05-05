import { context } from "../context";


export function system_and_arch(): [string, string] {
    let arch = process.arch as string
    if (! /arm|x(32|64)/.test(arch)) arch = ""

    let system = process.platform as string
    if (! /win32|linux|darwin/.test(system)) system = ""
    return [ system.replace(/\d+/, ""), arch ]
}

export function remoteNodeFileExtension () {

}
