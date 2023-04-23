import { which } from "shelljs";

export function command_exist(command: string) {
    return which(command);
}