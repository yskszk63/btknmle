use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead as _;
use std::path::Path;

#[derive(Debug)]
struct Entry {
    name: String,
    alias: Option<String>,
    value: u8,
}

fn parse_line(line: String) -> Entry {
    let mut cols = line.splitn(3, ' ');
    let (name, value, alias) = match (cols.next(), cols.next(), cols.next()) {
        (Some(name), Some(value), Some("-")) => (name, value, None),
        (Some(name), Some(value), Some(alias)) => (name, value, Some(alias)),
        (Some(name), Some(value), None) => (name.clone(), value, Some(name)),
        _ => panic!("parse error"),
    };
    let name = format!("KEY_{}", name);
    let value = u8::from_str_radix(value, 16).unwrap();
    let alias = alias.map(|a| format!("KEY_{}", a));
    Entry { name, value, alias }
}

fn filter_comment(line: String) -> String {
    line.splitn(2, '#').next().unwrap().to_string()
}

fn gen(read: &mut impl io::Read, w: &mut impl io::Write) -> io::Result<()> {
    let entries = io::BufReader::new(read)
        .lines()
        .map(Result::unwrap)
        .map(filter_comment)
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();

    writeln!(w, r#"#[allow(non_camel_case_types)]"#)?; // TODO
    writeln!(w, r#"#[derive(Debug, Clone, PartialEq, Eq, Hash)]"#)?;
    writeln!(w, r#"pub enum KeyboardUsageId {{"#)?;
    for Entry { name, value, .. } in &entries {
        writeln!(w, r#"/// {} ({:#04x})"#, name, value)?;
        writeln!(w, r#"{},"#, name)?;
        writeln!(w, r#""#)?;
    }
    writeln!(w, r#"}}"#)?;

    writeln!(
        w,
        r#"impl std::convert::TryFrom<btknmle_input::KeyCodes> for KeyboardUsageId {{"#
    )?;
    writeln!(w, r#"type Error = crate::NoMappingFound;"#)?;
    writeln!(
        w,
        r#"fn try_from(v: btknmle_input::KeyCodes) -> Result<Self, <Self as std::convert::TryFrom<btknmle_input::KeyCodes>>::Error> {{"#
    )?;
    writeln!(w, r#"match v {{"#)?;
    for Entry { name, alias, .. } in &entries {
        if let Some(alias) = alias {
            writeln!(
                w,
                r#"btknmle_input::KeyCodes::{} => Ok(Self::{}),"#,
                alias, name
            )?;
            writeln!(w, r#""#)?;
        }
    }
    writeln!(w, r#"e => Err(crate::NoMappingFound(e))"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w, r#"}}"#)?;

    writeln!(w, r#"impl From<KeyboardUsageId> for u8 {{"#)?;
    writeln!(w, r#"fn from(v: KeyboardUsageId) -> Self {{"#)?;
    writeln!(w, r#"match v {{"#)?;
    for Entry { name, value, .. } in &entries {
        writeln!(w, r#"KeyboardUsageId::{} => {:#04x},"#, name, value)?;
        writeln!(w, r#""#)?;
    }
    writeln!(w, r#"}}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w, r#"}}"#)?;

    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;
    writeln!(w, r#""#)?;

    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut genrs = File::create(Path::new(&out_dir).join("gen.rs")).unwrap();

    let mut input = File::open("src/keyboard_usage_id.txt").unwrap();

    gen(&mut input, &mut genrs).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/keyboard_usage_id.txt");
}
