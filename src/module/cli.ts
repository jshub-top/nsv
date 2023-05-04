import { Command } from "commander";
import { use_path_node_version, use_remote_node_version, use_local_node_version } from "./version"
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

    async function use(version: string, option: any) {
        let use_version = use_path_node_version(version)
        if (!use_version) {
            await use_remote_node_version(version)
            use_version = use_path_node_version(version)
            if ( use_version === void 0 ) throw new Error(`use path node version ${use_version} error. please push issuse to https://github.com/1739616529/nsv/issues/new`)
        }
        console.log(`v${use_version}`)
    }

    async function local(version: string, option: any) {
        let use_version = use_local_node_version(version)
        if (!use_version) {
            await use_remote_node_version(version)
            use_version = use_local_node_version(version)
            if ( use_version === void 0 ) throw new Error(`use local node version ${use_version} error. please push issuse to https://github.com/1739616529/nsv/issues/new`)
        }
        console.log(`v${use_version}`)
    }

}

