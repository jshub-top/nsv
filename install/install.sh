#!/bin/sh

if [[ -z "$NSV_HOME" ]]; then
    NSV_HOME=~/.nsv
fi
if [ ! -d "$NSV_HOME" ]; then
    mkdir -p "$NSV_HOME"
fi

OS=$(uname | tr '[:upper:]' '[:lower:]')
log_file="$NSV_HOME/install.log"

NSV_PROFILE="$HOME/.nsv_profile_sh"


set_profile_content() {

    sh_profile_content="
[[ -f ~/.nsv_profile_sh ]] && source ~/.nsv_profile_sh"
    echo "$sh_profile_content" >> ~/.bashrc
    echo "set sh profile" >> $log_file
    echo "$sh_profile_content" >> $log_file

    nsv_sh_profile_content="
#!/bin/sh
timestamp=\$(date +%s%3N)
export NSV_HOME=$NSV_HOME
export NSV_MATEFILE=\$NSV_HOME/temp/\$timestamp
export PATH=\$NSV_MATEFILE:\$NSV_HOME/temp/default:\$NSV_HOME:\$PATH
nsv adapt
    "
    echo "$nsv_sh_profile_content" > "$NSV_PROFILE"
    echo "set nsv profile" >> $log_file
    echo "$nsv_sh_profile_content" >> $log_file

}


download_file() {
    echo "download file" >> $log_file
    echo "url: $1" >> $log_file
    echo "output: $2" >> $log_file
    status_code=$(curl -fsSL -# -o $2 -w "%{http_code}" "$1")
    if [ "$status_code" -ne 200 ]; then

        echo "download file error"
        echo "status code: $status_code"
        echo "url: $1"
        echo "output: $2"

        echo "download file error" >> $log_file
        echo "status code: $status_code" >> $log_file
        echo "url: $1" >> $log_file
        echo "output: $2" >> $log_file

        exit 1

    fi
}

get_nsv_file_name() {
    arch="$(uname -m | sed -e 's/x86_64/x64/;s/i86pc/x64/;s/i686/x86/;s/aarch64/arm64/')"
    echo "nsv-$arch-$OS"
}

download_nsv_binary() {
    nsv_download_url="https://github.com/1739616529/nsv/releases/download/v0.0.1/$(get_nsv_file_name)"
    nsv_binary_path="$NSV_HOME/nsv"
    download_file "$nsv_download_url" "$nsv_binary_path"
    chmod 755 $nsv_binary_path
}




download_nsv_binary
# set profile
set_profile_content

echo -e "✨✨✨

    \033[34mnsv install success.\033[0m
    \033[31mPlease reload the user environment variables...\033[0m

✨✨✨
"
echo "nsv install soccess" >> $log_file
