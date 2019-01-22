use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "$value")]
    pub items: Vec<RootItem>,
}

#[derive(Clone, Debug, Deserialize)]
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
    pub fn as_struct(&self) -> Option<&Struct> {
        match *self {
            RootItem::Struct(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_xid_type(&self) -> Option<&XIDType> {
        match *self {
            RootItem::XIDType(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_xid_union(&self) -> Option<&XIDUnion> {
        match *self {
            RootItem::XIDUnion(ref x) => Some(x),
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
    pub fn as_enum(&self) -> Option<&Enum> {
        match *self {
            RootItem::Enum(ref x) => Some(x),
            _ => None,
        }
    }
    #[inline]
    pub fn as_event(&self) -> Option<&Event> {
        match *self {
            RootItem::Event(ref x) => Some(x),
            _ => None,
        }
    }
    #[inline]
    pub fn as_event_copy(&self) -> Option<&EventCopy> {
        match *self {
            RootItem::EventCopy(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_union(&self) -> Option<&Union> {
        match *self {
            RootItem::Union(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_error(&self) -> Option<&Error> {
        match *self {
            RootItem::Error(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_error_copy(&self) -> Option<&ErrorCopy> {
        match *self {
            RootItem::ErrorCopy(ref x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn as_request(&self) -> Option<&Request> {
        match *self {
            RootItem::Request(ref x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Struct {
    pub name: String,
    #[serde(rename = "$value")]
    pub items: Vec<StructItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum StructItem {
    #[serde(rename = "field")]
    Field(Field),
    #[serde(rename = "pad")]
    Pad(Pad),
    #[serde(rename = "list")]
    List(List),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pad {
    pub bytes: Option<String>,
    pub align: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct List {
    // TODO
}

#[derive(Clone, Debug, Deserialize)]
pub struct XIDType {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct XIDUnion {
    pub name: String,
    #[serde(rename = "type")]
    pub types: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Typedef {
    #[serde(rename = "oldname")]
    pub ty: String,
    #[serde(rename = "newname")]
    pub alias: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Enum {
    pub name: String,
    #[serde(rename = "item")]
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "$value")]
    pub item: VariantItem,
}

#[derive(Clone, Debug, Deserialize)]
pub enum VariantItem {
    #[serde(rename = "value")]
    Value(String),
    #[serde(rename = "bit")]
    Bit(u32),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Event {
    pub name: String,
    #[serde(rename = "number")]
    pub code: u8,
    #[serde(rename = "$value")]
    pub items: Vec<EventItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum EventItem {
    #[serde(rename = "field")]
    Field(Field),
    #[serde(rename = "pad")]
    Pad(Pad),
    #[serde(rename = "list")]
    List(List),
    #[serde(rename = "doc")]
    Doc(Doc),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Doc {
    // TODO
}

#[derive(Clone, Debug, Deserialize)]
pub struct EventCopy {
    pub name: String,
    #[serde(rename = "number")]
    pub code: u8,
    #[serde(rename = "ref")]
    pub source: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Union {
    pub name: String,
    #[serde(rename = "list")]
    pub items: Vec<UnionItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UnionItem {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub count: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Error {
    pub name: String,
    #[serde(rename = "number")]
    pub code: u8,
    #[serde(rename = "$value")]
    pub items: Vec<ErrorItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ErrorItem {
    #[serde(rename = "field")]
    Field(Field),
    #[serde(rename = "pad")]
    Pad(Pad),
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorCopy {
    pub name: String,
    #[serde(rename = "number")]
    pub code: u8,
    #[serde(rename = "ref")]
    pub source: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    // TODO
}
