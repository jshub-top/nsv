
set __temp_shell_dir "$NSV_HOME/cache/nsv_temp_one_off_file.fish"

set NSV_HOME $HOME/.nsv
set PATH $NSV_HOME/local/node/bin $PATH

function nsv
    "$NSV_HOME/cache/node/bin/node" "$NSV_HOME/dist/index.js" $argv
    if test -e $__temp_shell_dir
        . $__temp_shell_dir
        rm $__temp_shell_dir
    end
end
