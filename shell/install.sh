
# is enable proxy
read -p "nsv: enabled proxy [Y/n]: " proxy_status
proxy_status=$(echo $proxy_status | tr 'A-Z' 'a-z')

nsv_tgz_file_name="nsv.tgz"
nsv_tgz_url="https://github.com/1739616529/nsv/releases/download/v0.0.1/$nsv_tgz_file_name"
nsv_temp_tgz_dir="$HOME/$nsv_tgz_file_name"
github_proxy_url="https://ghproxy.com/"

if [ "$proxy_status" = "" ] || [ "$proxy_status" = "y" ] || [ "$proxy_status" = "yes" ]; then
    nsv_tgz_url="$github_proxy_url/$nsv_tgz_url"
fi

curl "$nsv_tgz_url" -# -o "$nsv_temp_tgz_dir"
tar -xf "$nsv_temp_tgz_dir" -C "$HOME"
rm "$HOME/$nsv_tgz_file_name"
mv "$HOME/package" "$HOME/.nsv"
"$HOME/.nsv/nsv" install
