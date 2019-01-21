use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;

mod xproto;

struct Extra {
    xcb_name_map: HashMap<String, String>,
    xlib_name_map: HashMap<String, String>,
}

const XCB_NAME_MAP_PATH: &'static str = "xcb-name-map.txt";
const XLIB_NAME_MAP_PATH: &'static str = "xlib-name-map.txt";
const XPROTO_XML_PATH: &'static str = "proto/src/xproto.xml";

impl Extra {
    pub fn new() -> io::Result<Self> {
        return Ok(Extra {
            xcb_name_map: load_map(XCB_NAME_MAP_PATH)?,
            xlib_name_map: load_map(XLIB_NAME_MAP_PATH)?,
        });

        fn load_map<P: AsRef<path::Path> + ?Sized>(
            path: &P,
        ) -> io::Result<HashMap<String, String>> {
            let mut map: HashMap<String, String> = HashMap::new();
            for line in io::BufReader::new(fs::File::open(path).unwrap()).lines() {
                let line = line?;
                if line == "" {
                    continue;
                }
                let mut part_iter = line.split(" ");
                let from = part_iter.next().unwrap();
                let to = part_iter.next().unwrap();
                assert_eq!(part_iter.next(), None);
                map.insert(from.to_string(), to.to_string());
            }
            Ok(map)
        }
    }

    pub fn get_xcb_name<'a>(&'a self, name: &'a str) -> &'a str {
        match self.xcb_name_map.get(name) {
            Some(ref name) => name,
            None => {
                println!("cargo:warning=Could not find xcb name for {}.", name);
                name
            }
        }
    }

    pub fn get_xlib_name<'a>(&'a self, name: &'a str) -> &'a str {
        match self.xlib_name_map.get(name) {
            Some(ref name) => name,
            None => {
                println!("cargo:warning=Could not find xlib name for {}.", name);
                name
            }
        }
    }
}

fn main() {
    println!(r#"cargo:rerun-if-changed="{}""#, XCB_NAME_MAP_PATH);
    println!(r#"cargo:rerun-if-changed="{}""#, XLIB_NAME_MAP_PATH);
    println!(r#"cargo:rerun-if-changed="{}""#, XPROTO_XML_PATH);

    let extra = Extra::new().unwrap();

    let root: xproto::Root = serde_xml_rs::from_reader(io::BufReader::new(
        fs::File::open(XPROTO_XML_PATH).unwrap(),
    ))
    .unwrap();

    let mut out = io::BufWriter::new(fs::File::create("src/lib.rs").unwrap());

    out.write_all(
        br##"// This file is automatically generated.

#![allow(non_camel_case_types)]

use x11::xlib;

"##,
    )
    .unwrap();

    for typedef in root.items.iter().filter_map(xproto::RootItem::as_typedef) {
        write_typedef(&mut out, &extra, &typedef).unwrap();
    }

    for xidtype in root.items.iter().filter_map(xproto::RootItem::as_xidtype) {
        write_xidtype(&mut out, &extra, &xidtype).unwrap();
    }

    writeln!(out).unwrap();

    for xidunion in root.items.iter().filter_map(xproto::RootItem::as_xidunion) {
        write_xidunion(&mut out, &extra, &xidunion).unwrap();
    }

    writeln!(out).unwrap();

    for enum_ in root.items.iter().filter_map(xproto::RootItem::as_enum) {
        write_enum(&mut out, &extra, &enum_).unwrap();
    }

    writeln!(out).unwrap();

    for struct_ in root.items.iter().filter_map(xproto::RootItem::as_struct) {
        write_struct(&mut out, &extra, &struct_).unwrap();
    }
}

fn write_typedef<W: io::Write>(
    out: &mut W,
    extra: &Extra,
    typedef: &xproto::Typedef,
) -> io::Result<()> {
    writeln!(
        out,
        "pub type {alias} = {ty};",
        alias = extra.get_xcb_name(&typedef.alias),
        // NOTE(mickvangelderen): We assume the type definitions are simple.
        ty = extra.get_xcb_name(&typedef.ty)
    )?;
    Ok(())
}

fn write_xidtype<W: io::Write>(
    out: &mut W,
    extra: &Extra,
    xidtype: &xproto::XIDType,
) -> io::Result<()> {
    writeln!(
        out,
        "pub type {xcb_name} = xlib::{xlib_name};",
        xcb_name = extra.get_xcb_name(&xidtype.name),
        xlib_name = extra.get_xlib_name(&xidtype.name)
    )?;
    Ok(())
}

fn write_xidunion<W: io::Write>(
    out: &mut W,
    extra: &Extra,
    xidunion: &xproto::XIDUnion,
) -> io::Result<()> {
    let union_name = extra.get_xcb_name(&xidunion.name);
    writeln!(
        out,
        r##"#[repr(C)]
pub union {name} {{"##,
        name = union_name
    )?;
    for ty in &xidunion.types {
        let xcb_ty = extra.get_xcb_name(ty);
        let field = &xcb_ty[4..xcb_ty.len() - 2];
        writeln!(out, "    pub {field}: {ty},", ty = xcb_ty, field = field)?;
    }
    writeln!(
        out,
        r"}}
"
    )?;
    Ok(())
}

fn write_enum<W: io::Write>(
    out: &mut W,
    extra: &Extra,
    enum_: &xproto::Enum,
) -> io::Result<()> {
    writeln!(
        out,
        r##"#[repr(u32)]
pub enum {name} {{"##,
        name = extra.get_xcb_name(&enum_.name),
    )?;
    writeln!(out, r"}}
")?;
    Ok(())
}

fn write_struct<W: io::Write>(
    out: &mut W,
    extra: &Extra,
    struct_: &xproto::Struct,
) -> io::Result<()> {
    let struct_name = extra.get_xcb_name(&struct_.name);

    let mut pad_index = 0;
    writeln!(
        out,
        r##"#[derive(Debug)]
#[repr(C)]
pub struct {name} {{"##,
        name = struct_name
    )?;
    for item in struct_.items.iter() {
        match item {
            xproto::StructItem::Field(field) => {
                let field_ty = extra.get_xcb_name(&field.ty);
                writeln!(
                    out,
                    "    pub {name}: {ty},",
                    name = field.name,
                    ty = field_ty
                )?;
            }
            xproto::StructItem::Pad(pad) => {
                if let Some(ref bytes) = pad.bytes {
                    writeln!(
                        out,
                        "    pad{index}: [u8; {bytes}],",
                        index = pad_index,
                        bytes = bytes
                    )?;
                    pad_index += 1;
                }
                if let Some(_) = pad.align {
                    // NOTE(mickvangelderen): align property is not documented
                    // in the xproto xml specification and it seems like the
                    // padding already properly aligns the fields. Could verify
                    // this but that would mean recursively determining the
                    // sizes of the fields which would make this script a lot
                    // more complicated.
                    println!("cargo:warning=Ignored align on {}", struct_name);
                }
            }
            xproto::StructItem::List(_) => {
                // TODO: Do something with list?
                writeln!(out, "    // list")?;
            }
        }
    }
    writeln!(
        out,
        r##"}}
"##
    )?;
    Ok(())
}