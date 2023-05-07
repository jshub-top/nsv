
import inquirer from "inquirer";
import console_menu from "console-menu";
import { readdirSync} from "fs-extra";
import { context } from "../context"
import { get_current_node_version, get_local_node_version_list } from "../lib/version";
export function run () {

    const current_version = get_current_node_version()
    const local_version_list = get_local_node_version_list()

    let current_version_index = 0
    if (current_version) current_version_index = local_version_list.findIndex(v => v === current_version)

}
