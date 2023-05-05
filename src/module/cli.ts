import { Command } from "commander";
import { use, local } from "./version"
import { discern } from "./discern"
import { version as app_version } from "../../local.json"
import { install, uninstall } from "./install"
export function run() {
    const program = new Command()


    program
        .option("-v --version")
        .description("printf nsv version")
        .action(version)

    program
        .command("use")
        .description("use node version")
        .argument("<string>", "use node version. (v14, 14, v14.xx.xx, 14.xx.xx)")
        .action(use);

    program
        .command("local")
        .description("lasting you select node version")
        .argument("<string>", "use node version. (v14, 14, v14.xx.xx, 14.xx.xx)")
        .action(local);

    program
        .command("discern")
        .option("-e --enable", "enable auto discern node version")
        .option("-d --disable", "disable auto discern node version")
        .option("-s --status", "discern status")
        .description("discern your project node config version.")
        .action(discern);

    program
        .command("install")
        .description("install")
        .action(install);

    program
        .command("uninstall")
        .description("uninstall")
        .action(uninstall);




    program.parse()




    function version() {
        console.log(app_version)
    }
}

