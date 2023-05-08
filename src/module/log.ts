import { user_db } from "./config"


export function log(msg: string) {
    console.log("")
}





function logger() {
    const is_enable_logger = user_db.get("logger")
    if (is_enable_logger === false) return

    let log = ""

    process.stdout.on("data", (data) => {
        console.log("111")
        console.log(data)
        console.log("111")
    })

}
logger()
