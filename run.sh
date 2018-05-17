set -e
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 [--gdb] <source_file> <program args>" >&2
    exit 1
fi

if [[ $1 == "--gdb" || $1 == "-g" ]];then
    gdb="true"
    source_file="$2"
    args="${@:3}"
else
    gdb=""
    source_file="$1"
    args="${@:2}"
fi

EXECUTABLE_FILE=/tmp/unix_fun_$(basename "$source_file" | grep -Po '.+(?=.c)') 

gcc -O0 -g -o "$EXECUTABLE_FILE" "$source_file" 

if [[ "$gdb" ]]; then
    gdb --args "$EXECUTABLE_FILE" "$args"
else 
    "$EXECUTABLE_FILE" "$args"
fi

