function get_json_value {
    temp=$(echo "$1" | grep -o "\"$2\": \"[^\"]*\"" | sed "s/\"$2\": \"\(.*\)\"/\1/")
    echo $temp
}

function get_next_json() {
    value=$(echo "$1" | grep -o "\"$2\": {[^}]*}" | sed "s/\"$2\": //")
    echo $value
}



function parse_json() {
    local json=$(echo "$1" | tr -d '\n')
    local key_list=("${@:2}")
    local key_list_len=${#key_list[@]}
    local value=""
    for ((i=1; i<=key_list_len; i++))
    do
        local item=${key_list[$(($i - 1))]}
        if [ $key_list_len == $i ]; then
            value=$(get_json_value "$json" "$item")
        else
            json=$(get_next_json "$json" "$item")
        fi
    done

    echo $value
}
