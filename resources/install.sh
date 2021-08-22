#!/bin/sh

__J="$(pwd)/$(dirname "$0")/j"
"$__J"

cd () {
    "$__J --add $1 &" &>/dev/null
    builtin cd "$1"
}
${J_CUSTOM_CMD:-j} () {
    cd "$("$__J" "$1")"
}
