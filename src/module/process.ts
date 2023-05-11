import { context, RunStatus } from "../context"
import { rm } from "shelljs"

async function chean_nsv_local_file() {
    const status = context.get("runStatus")

    if (status !== RunStatus.normal) {

        // file download
        if (status === RunStatus.download) {
            const req = context.get("download_request")
            const download_file = context.get("download_temp_dir")
            req.abort()
            rm(download_file)

        } else // file extract
        if (status === RunStatus.extract) {
            const cp = context.get("extract_process")
            const extract_file = context.get("extract_dir")
            cp.kill("SIGINT")
            rm("-rf", extract_file)
        }




    }

    process.exit(0)
}
process.on("beforeExit", chean_nsv_local_file)
process.on("SIGINT", chean_nsv_local_file)
