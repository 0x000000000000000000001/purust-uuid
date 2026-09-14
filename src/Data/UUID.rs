fn uuid_throw(name: &str, message: &str) -> ! {
    Purs_Effect_Exception::purust_exception_raise(
        Purs_Effect_Exception::Effect_Exception_errorWithName(message.into(), name.into()),
    )
}

pub fn Data_UUID_getUUIDImpl() -> crate::UnknownType {
    purust_core::Value::Func1(purust_core::Func1::Static(|_| {
        purust_core::Value::String(uuid::Uuid::new_v4().to_string())
    }))
}

pub fn Data_UUID_validateV4UUID(value: String) -> bool {
    // Despite the legacy FFI name, JS uuid.validate accepts versions 1–8,
    // NIL and MAX, but not Rust's additional braced/simple/URN formats.
    let bytes = value.as_bytes();
    if bytes.len() != 36 || !bytes.iter().enumerate().all(|(i, b)| {
        if [8, 13, 18, 23].contains(&i) { *b == b'-' } else { b.is_ascii_hexdigit() }
    }) { return false; }
    value == "00000000-0000-0000-0000-000000000000"
        || value.eq_ignore_ascii_case("ffffffff-ffff-ffff-ffff-ffffffffffff")
        || ((b'1'..=b'8').contains(&bytes[14]) && b"89ab".contains(&bytes[19].to_ascii_lowercase()))
}

fn uuid_name_and_namespace(name: String, namespace: String) -> (String, uuid::Uuid) {
    // uuid's stringToBytes uses encodeURIComponent: lone surrogates throw
    // URIError before namespace validation, rather than becoming U+FFFD.
    let name = String::from_utf16(&purust_core::purust_string_to_utf16(&name))
        .unwrap_or_else(|_| uuid_throw("URIError", "URI malformed"));
    if !Data_UUID_validateV4UUID(namespace.clone()) { uuid_throw("TypeError", "Invalid UUID"); }
    let namespace = uuid::Uuid::parse_str(&namespace)
        .unwrap_or_else(|_| uuid_throw("TypeError", "Invalid UUID"));
    (name, namespace)
}

pub fn Data_UUID_getUUID3Impl(name: String, namespace: String) -> String {
    let (name, namespace) = uuid_name_and_namespace(name, namespace);
    uuid::Uuid::new_v3(&namespace, name.as_bytes()).to_string()
}

pub fn Data_UUID_getUUID5Impl(name: String, namespace: String) -> String {
    let (name, namespace) = uuid_name_and_namespace(name, namespace);
    uuid::Uuid::new_v5(&namespace, name.as_bytes()).to_string()
}
