//! 控制流与非规范消息行的迁移回归。

use bmsedit::patch::BmsDoc;

#[test]
fn setrandom_selects_fixed_branch() {
    let doc = BmsDoc::from_bytes(
        b"#SETRANDOM 1\n#IF 1\n#TITLE A\n#ENDIF\n#IF 2\n#TITLE B\n#ENDIF\n#ENDRANDOM\n".to_vec(),
        1,
    )
    .expect("parse");
    assert_eq!(doc.bms.metadata.title.as_deref(), Some("A"));
}

#[test]
fn noncanonical_message_line_is_writable() {
    let mut doc = BmsDoc::from_bytes(b"#1:0101\n".to_vec(), 1).expect("parse");
    let lines = doc.message_lines(0, "01");
    let Some((index, _)) = lines.first() else {
        panic!("message line should be indexed");
    };
    doc.document
        .replace_content(*index, "#1:0202")
        .expect("replace");
    assert_eq!(doc.document.render(), b"#1:0202\n");
}
