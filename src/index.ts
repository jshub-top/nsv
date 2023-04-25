
function run_main1() {
    const argv_len = process.argv.length

    // 如果大于2 调用 cli
    if (argv_len > 2) {
        import("./module/cli").then(v => v.run())
        return
    }

    // 否则调用 交互命令
}
console.log("js running")
run_main1()
