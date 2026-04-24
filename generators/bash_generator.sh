#!/bin/bash

BASE62="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

random_string() {
    local length=$1
    local result=""
    for i in $(seq 1 $length); do
        result+=${BASE62:$((RANDOM % 62)):1}
    done
    echo "$result"
}

base62_encode() {
    local num=$1
    local result=""
    if [ $num -eq 0 ]; then
        echo "${BASE62:0:1}"
        return
    fi
    while [ $num -gt 0 ]; do
        result="${BASE62:$((num % 62)):1}$result"
        num=$((num / 62))
    done
    echo "$result"
}

timestamp_part() {
    echo -n "$(date +%s)" | base62_encode
}

generate_clid() {
    local format=$1
    local with_ts=$2
    
    case $format in
        "CLID-1")
            local p1=$(random_string 4)
            local p2=$(random_string 4)
            local p3=$(random_string 4)
            local p4=$(random_string 4)
            local p5=$(random_string 4)
            local p6=$(random_string 4)
            if [ "$with_ts" = "true" ]; then
                p1=$(timestamp_part)
            fi
            echo "$p1-$p2-$p3-$p4-$p5-$p6"
            ;;
        "CLID-2")
            if [ "$with_ts" = "true" ]; then
                echo "$(timestamp_part)$(random_string 4)$(random_string 4)$(random_string 4)$(random_string 4)$(random_string 4)"
            else
                echo "$(random_string 4)$(random_string 4)$(random_string 4)$(random_string 4)$(random_string 4)$(random_string 4)"
            fi
            ;;
        "CLID-3")
            if [ "$with_ts" = "true" ]; then
                echo "$(timestamp_part)$(random_string 15)"
            else
                echo "$(random_string 21)"
            fi
            ;;
        "CLID-4")
            if [ "$with_ts" = "true" ]; then
                echo "$(timestamp_part)$(random_string 14)"
            else
                echo "$(random_string 20)"
            fi
            ;;
        "CLID-5")
            if [ "$with_ts" = "true" ]; then
                echo "$(timestamp_part)$(random_string 10)"
            else
                echo "$(random_string 16)"
            fi
            ;;
    esac
}

echo "=== CLID Generator ==="
echo "CLID-1:      $(generate_clid "CLID-1" "false")"
echo "CLID-1-TS:   $(generate_clid "CLID-1" "true")"
echo "CLID-2:      $(generate_clid "CLID-2" "false")"
echo "CLID-2-TS:   $(generate_clid "CLID-2" "true")"
echo "CLID-3:      $(generate_clid "CLID-3" "false")"
echo "CLID-3-TS:   $(generate_clid "CLID-3" "true")"
echo "CLID-4:      $(generate_clid "CLID-4" "false")"
echo "CLID-4-TS:   $(generate_clid "CLID-4" "true")"
echo "CLID-5:      $(generate_clid "CLID-5" "false")"
echo "CLID-5-TS:   $(generate_clid "CLID-5" "true")"
