use std::borrow::Cow;

/// Escape a filter literal.
///
/// Literal values appearing in an LDAP filter can contain any character,
/// but some characters (parentheses, asterisk, backslash, NUL) must be
/// escaped in the filter's string representation. This function does the
/// escaping.
///
/// The argument, `lit`, can be owned or borrowed. The function doesn't
/// allocate the return value unless there's need to escape the input.
pub fn ldap_escape<'a, S: Into<Cow<'a, str>>>(lit: S) -> Cow<'a, str> {
    #[inline]
    fn needs_escape(c: u8) -> bool {
        c == b'\\' || c == b'*' || c == b'(' || c == b')' || c == 0
    }

    #[inline]
    fn xdigit(c: u8) -> u8 {
        c + if c < 10 { b'0' } else { b'a' - 10 }
    }

    let lit = lit.into();
    let mut vec_push = false;
    let mut output = Vec::with_capacity(lit.len() + 12); // guess: 4 escaped chars
    for (i, &c) in lit.as_bytes().iter().enumerate() {
        if needs_escape(c) {
            if !vec_push {
                output.extend(lit[..i].as_bytes());
                vec_push = true;
            }
            output.push(b'\\');
            output.push(xdigit(c >> 4));
            output.push(xdigit(c & 0xF));
        } else if vec_push {
            output.push(c);
        }
    }
    if vec_push {
        Cow::Owned(unsafe { String::from_utf8_unchecked(output) })
    } else {
        lit.into()
    }
}
