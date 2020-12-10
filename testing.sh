#!/bin/bash

commands="cargo build ; ./target/debug/rusty-configs init
cargo build ; ./target/debug/rusty-configs read
cargo build ; ./target/debug/rusty-configs write
cargo build ; ./target/debug/rusty-configs delete -p /tmp/test
cargo build ; ./target/debug/rusty-configs delete -i 1
cargo build ; ./target/debug/rusty-configs delete -n test
cargo build ; ./target/debug/rusty-configs add -p /tmp/test4 -g 1
cargo build ; ./target/debug/rusty-configs list version
cargo build ; ./target/debug/rusty-configs list config
"
cmd=$(echo "$commands" | fzf)
[[ ! -z $cmd ]] && echo -e "=========\nrunning command:\n$cmd\n=========" && bash -c "$cmd"
