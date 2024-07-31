
if test -z "$NSV_HOME"
    set NSV_HOME "$HOME/.nsv"
end
if not test -d "$NSV_HOME"
    mkdir -p "$NSV_HOME"
end




set OS (uname | tr '[:upper:]' '[:lower:]')
set log_file "$NSV_HOME/install.log"

set NSV_PROFILE "$HOME/.config/fish/conf.d/nsv.fish"

function set_profile_content
    set nsv_fish_profile_content "
#!/bin/fish
set timestamp (date +%s%3N)
set -gx NSV_HOME $NSV_HOME
set -gx NSV_MATEFILE \$NSV_HOME/temp/\$timestamp
set -gx PATH \$NSV_MATEFILE \$NSV_HOME/temp/default \$NSV_HOME \$PATH
nsv adapt
    "
    echo "$nsv_fish_profile_content" > "$NSV_PROFILE"
    echo "set nsv profile" >> $log_file
    echo "$nsv_fish_profile_content" >> $log_file
end

function download_file
    echo "download file" >> $log_file
    echo "url: $argv[1]" >> $log_file
    echo "output: $argv[2]" >> $log_file
    set status_code (curl -fsSL -# -o $argv[2] -w "%{http_code}" "$argv[1]")
    if test "$status_code" -ne 200
        echo "download file error"
        echo "status code: $status_code"
        echo "url: $argv[1]"
        echo "output: $argv[2]"

        echo "download file error" >> $log_file
        echo "status code: $status_code" >> $log_file
        echo "url: $argv[1]" >> $log_file
        echo "output: $argv[2]" >> $log_file

        exit 1
    end
end

function get_nsv_file_name
    set arch (uname -m | sed -e 's/x86_64/x64/;s/i86pc/x64/;s/i686/x86/;s/aarch64/arm64/')
    echo "nsv-$arch-$OS"
end

function download_nsv_binary
    set nsv_download_url "https://github.com/1739616529/nsv/releases/download/v0.0.1/$(get_nsv_file_name)"
    set nsv_binary_path "$NSV_HOME/nsv"
    download_file "$nsv_download_url" "$nsv_binary_path"
    chmod 755 $nsv_binary_path
end

download_nsv_binary
# set profile
set_profile_content

echo -e "✨✨✨

    \033[34mnsv install success.\033[0m
    \033[31mPlease reload the user environment variables...\033[0m

✨✨✨
"
echo "nsv install success" >> $log_file
