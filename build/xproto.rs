use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "$value")]
    pub items: Vec<RootItem>,
}

#[derive(Debug, Deserialize)]
pub enum RootItem {
    #[serde(rename = "struct")]
    Struct(Struct),
    #[serde(rename = "xidtype")]
    XIDType(XIDType),
    #[serde(rename = "xidunion")]
    XIDUnion(XIDUnion),
    #[serde(rename = "typedef")]
    Typedef(Typedef),
    #[serde(rename = "enum")]
    Enum(Enum),
    #[serde(rename = "event")]
    Event(Event),
    #[serde(rename = "eventcopy")]
    EventCopy(EventCopy),
    #[serde(rename = "union")]
    Union(Union),
    #[serde(rename = "error")]
    Error(Error),
    #[serde(rename = "errorcopy")]
    ErrorCopy(ErrorCopy),
    #[serde(rename = "request")]
    Request(Request),
}

impl RootItem {
    #[inline]
    pub fn as_xidtype(&self) -> Option<&XIDType> {
        match *self {
            RootItem::XIDType(ref x) => Some(x),
            _ => None,
        }
    }

    pub fn as_xidunion(&self) -> Option<&XIDUnion> {
        match *self {
            RootItem::XIDUnion(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_enum(&self) -> Option<&Enum> {
        match *self {
            RootItem::Enum(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_typedef(&self) -> Option<&Typedef> {
        match *self {
            RootItem::Typedef(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_struct(&self) -> Option<&Struct> {
        match *self {
            RootItem::Struct(ref x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Struct {
    pub name: String,
    #[serde(rename = "$value")]
    pub items: Vec<StructItem>,
}

#[derive(Debug, Deserialize)]
pub enum StructItem {
    #[serde(rename = "field")]
    Field(Field),
    #[serde(rename = "pad")]
    Pad(Pad),
    #[serde(rename = "list")]
    List(List),
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Deserialize)]
pub struct Pad {
    pub bytes: Option<String>,
    pub align: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct List {}

#[derive(Debug, Deserialize)]
pub struct XIDType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct XIDUnion {
    pub name: String,
    #[serde(rename = "type")]
    pub types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Typedef {
    #[serde(rename = "oldname")]
    pub ty: String,
    #[serde(rename = "newname")]
    pub alias: String,
}

#[derive(Debug, Deserialize)]
pub struct Enum {
    pub name: String,
    #[serde(rename = "item")]
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "$value")]
    pub item: VariantItem,
}

#[derive(Debug, Deserialize)]
pub enum VariantItem {
    #[serde(rename = "value")]
    Value(String),
    #[serde(rename = "bit")]
    Bit(String),
}

#[derive(Debug, Deserialize)]
pub struct Event {}

#[derive(Debug, Deserialize)]
pub struct EventCopy {}

#[derive(Debug, Deserialize)]
pub struct Union {}

#[derive(Debug, Deserialize)]
pub struct Error {}

#[derive(Debug, Deserialize)]
pub struct ErrorCopy {}

#[derive(Debug, Deserialize)]
pub struct Request {}
