
set __temp_shell_dir "$NSV_HOME/cache/nsv_temp_one_off_file.fish"
"$NSV_HOME/cache/node/bin/node" "$NSV_HOME/dist/index.js" $argv
if test -e $__temp_shell_dir
    . $__temp_shell_dir
    
end
set NSV_STATUS

function nsv

end