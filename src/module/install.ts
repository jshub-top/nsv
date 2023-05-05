
import { join, resolve } from "path";
import { EOL } from "os";
import { system, shell, shellConfigFileDir } from "../../local.json"
import { readFile, writeFile } from "fs-extra";
import { context } from "../context"
export async function install () {
    let content = ""
    if ("NSV_HOME" in process.env) return console.log("nsv: installed")

    // linux macos
    if (system === "linux" || system === "darwin") {
        const shell_config_file_content = await readFile(shellConfigFileDir, { encoding: "utf-8" }).then(v => v.toString().split(EOL))
        shell_config_file_content.push(`export NSV_HOME=$HOME/.nsv`)
        shell_config_file_content.push(`[ -s "$NSV_HOME/nsv.sh" ] && . "$NSV_HOME/nsv.sh"`)
        shell_config_file_content.push(`export PATH=$NSV_HOME/local/bin:$PATH`)
        shell_config_file_content.push(``)
        content = `source ${shellConfigFileDir}`
        await writeFile(shellConfigFileDir, shell_config_file_content.join(EOL), { encoding: "utf-8" })
    } else
    if (system === "win") {
        const home = context.get("dir").home
        content = `
            $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")

            if (-not $isAdmin) {
                Start-Process powershell.exe "-File $PSCommandPath" -Verb RunAs
                Exit 0
            }

            function Get-Path-Crs ($path) {

                $local_env =  Get-Item -Path $path
                $local_env.GetValue(
                    "Path",
                    "",
                    [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames
                )
            }

            $state = 0

            $user_path_dir = "HKCU:\\Environment"
            $user_path_value = Get-Path-Crs $user_path_dir
            Write-Output $user_path_value
            if ( $user_path_value -notmatch "%NSV_HOME%" ) {
                 Set-ItemProperty -Path $user_path_dir -Name 'NSV_HOME' -Value ${home} -Type String
                 Set-ItemProperty -Path $user_path_dir -Name 'Path' -Value "%NSV_HOME%;$user_path_value" -Type ExpandString
                $state += 1
            }


            $system_path_dir = "HKLM:\\SYSTEM\\ControlSet001\\Control\\Session Manager\\Environment\\"
            $system_path_value = Get-Path-Crs $system_path_dir
            Write-Output $system_path_value
            if ($system_path_value -notmatch "%NSV_LOCAL_NODE%") {
                 Set-ItemProperty -Path $system_path_dir -Name 'NSV_LOCAL_NODE' -Value "${join(home, "local/node")}" -Type String
                 Set-ItemProperty -Path $system_path_dir -Name 'Path' -Value "%NSV_LOCAL_NODE%;$system_path_value" -Type ExpandString
                $state += 2
            }

            Write-Output "$state"

            if ($state -eq 0) {
                return
            }
            $answer = Read-Host "Restart the system for the environment variables to take effect. Do you want logout user now? (Y/n)"
            if (!($answer -eq "y") -and !($answer -eq "yes") -and !($answer.Length -eq 0)) {
                exit 0
            }
            Logoff
            exit 0
        `

    }

    content && writeFile(join(context.get("dir").cache, context.get("temp_file_name")), content)

}

export async function uninstall () {
    let content = ""
    if (system === "linux" || system === "darwin") {
        const shell_config_file_content = await readFile(shellConfigFileDir, { encoding: "utf-8" }).then(v => v.toString().split(EOL).filter(v => !/NSV_HOME/.test(v)))
        content = `source ${shellConfigFileDir}`
        writeFile(shellConfigFileDir, shell_config_file_content.join(EOL), { encoding: "utf-8" })
    }
    else if (system === "win") {
        content = `
            $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")

            if (-not $isAdmin) {
                Start-Process powershell.exe "-File $PSCommandPath" -Verb RunAs
                Exit 0
            }

            function Get-Path-Crs ($path) {

                $local_env =  Get-Item -Path $path
                $local_env.GetValue(
                    "Path",
                    "",
                    [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames
                )
            }

            $user_path_dir = "HKCU:\\Environment"
            $user_path_value = Get-Path-Crs $user_path_dir
            if ( $user_path_value -match "%NSV_HOME%" ) {
                Remove-ItemProperty -Path $user_path_dir -Name 'NSV_HOME'
                $new_user_path_value = $user_path_value.Split(';') |  Where-Object {
                    $_ -notlike "%NSV_HOME%"
                }
                $new_user_path_value = $new_user_path_value -join ";"
                Set-ItemProperty -Path $user_path_dir -Name 'Path' -Value "$new_user_path_value" -Type ExpandString
                $state_num += 1
            }


            $system_path_dir = "HKLM:\\SYSTEM\\ControlSet001\\Control\\Session Manager\\Environment\\"
            $system_path_value = Get-Path-Crs $system_path_dir
            if ($system_path_value -match "%NSV_LOCAL_NODE%") {
                Remove-ItemProperty -Path $system_path_dir -Name 'NSV_LOCAL_NODE'
                $new_system_path_value = $system_path_value.Split(';') |  Where-Object {
                    $_ -notlike "%NSV_LOCAL_NODE%"
                }
                $new_system_path_value = $new_system_path_value -join ";"
                Set-ItemProperty -Path $system_path_dir -Name 'Path' -Value "$new_system_path_value" -Type ExpandString
                $state_num += 2
            }

            if ($state -eq 0) {
                return
            }
            $answer = Read-Host "Restart the system for the environment variables to take effect. Do you want logout user now? (Y/n)"
            $answer = $answer.ToLower()
            if (!($answer -eq "y") -and !($answer -eq "yes") -and !($answer.Length -eq 0)) {
                exit 0
            }
            Logoff
            exit 0
        `

    }
    writeFile(`${context.get("dir").cache}/${context.get("temp_file_name")}`, content)
}
