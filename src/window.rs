use std::error::Error;
use x11rb::protocol::xproto::{Atom, AtomEnum, ConnectionExt, GetPropertyReply, Window};
use x11rb::rust_connection::RustConnection;

// mostly taken from https://www.reddit.com/r/rust/comments/f7yrle/get_information_about_current_window_xorg/

pub fn get_active_window_class_and_name(
    conn: &RustConnection,
    root: u32,
) -> Result<(String, String), Box<dyn Error>> {
    let net_active_window = get_or_intern_atom(&conn, b"_NET_ACTIVE_WINDOW");
    let net_wm_name = get_or_intern_atom(&conn, b"_NET_WM_NAME");
    let utf8_string = get_or_intern_atom(&conn, b"UTF8_STRING");

    let focus = match find_active_window(&conn, root, net_active_window) {
        None => {
            return Err("no active window selected".into());
        }
        Some(x) => x,
    };

    let (wm_class, string): (Atom, Atom) = (AtomEnum::WM_CLASS.into(), AtomEnum::STRING.into());

    // get the property from the window that we need
    let class = conn.get_property(false, focus, wm_class, string, 0, u32::MAX)?;
    let name = conn.get_property(false, focus, net_wm_name, utf8_string, 0, u32::max_value())?;

    let (class, name) = (class.reply()?, name.reply()?);
    let (_, class) = parse_wm_class(&class);
    let name = parse_string_property(&name);

    Ok((String::from(class), String::from(name)))
}

pub fn get_or_intern_atom(conn: &RustConnection, name: &[u8]) -> Atom {
    let result = conn
        .intern_atom(false, name)
        .expect("Failed to intern atom")
        .reply()
        .expect("Failed receive interned atom");

    result.atom
}

fn find_active_window(
    conn: &RustConnection,
    root: Window,
    net_active_window: Atom,
) -> Option<Window> {
    let window: Atom = AtomEnum::WINDOW.into();
    let active_window = conn
        .get_property(false, root, net_active_window, window, 0, 1)
        .expect("Failed to get X11 property")
        .reply()
        .expect("Failed to receive X11 property reply");

    if active_window.format == 32 && active_window.length == 1 {
        active_window
            .value32()
            .expect("Invalid message. Expected value with format = 32")
            .next()
    } else {
        // Query the input focus
        Some(
            conn.get_input_focus()
                .expect("Failed to get input focus")
                .reply()
                .expect("Failed to receive X11 input focus")
                .focus,
        )
    }
}

fn parse_wm_class(property: &GetPropertyReply) -> (&str, &str) {
    if property.format != 8 {
        return (
            "Malformed property: wrong format",
            "Malformed property: wrong format",
        );
    }
    let value = &property.value;
    // The property should contain two null-terminated strings. Find them.
    if let Some(middle) = value.iter().position(|&b| b == 0) {
        let (instance, class) = value.split_at(middle);
        // Skip the null byte at the beginning
        let mut class = &class[1..];
        // Remove the last null byte from the class, if it is there.
        if class.last() == Some(&0) {
            class = &class[..class.len() - 1];
        }
        let instance = std::str::from_utf8(instance);
        let class = std::str::from_utf8(class);
        (
            instance.unwrap_or("Invalid utf8"),
            class.unwrap_or("Invalid utf8"),
        )
    } else {
        ("Missing null byte", "Missing null byte")
    }
}

fn parse_string_property(property: &GetPropertyReply) -> &str {
    std::str::from_utf8(&property.value).unwrap_or("Invalid utf8")
}
