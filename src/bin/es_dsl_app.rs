fn main() {
    let data = r#"41d528d3f37
aebad9891e8
aa36d638bee
569cd553bb4
235dc09bc1c
a41926b5c4a
51a0e31e0d7
c481967c80c
ce62313ac78"#;
    let arr = data.split("\n");
    for item in arr {
        let msg = r#"{"match_phrase": { "bid": "{}"}},"#;
        let msg = msg.replace("{}", item);
        println!("{}", msg);
    }
}
