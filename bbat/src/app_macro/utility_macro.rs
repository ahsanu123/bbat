// Reference: https://stackoverflow.com/a/39017042/19270838
macro_rules! nameof {
    ($name: ident) => {{
        let _ = &$name;
        stringify!($name)
    }};
    ($pathname: path) => {{
        stringify!($pathname)
    }};
}

pub(crate) use nameof;
