use std::process::Command;


#[test]
fn echo() {
    Command::new("./maelstrom test -w echo --bin ~/go/bin/maelstrom-echo --node-count 1 --time-limit 10")
}
