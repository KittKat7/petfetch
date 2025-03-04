/**
 * (c) 2025 KittKat
 */
pub(crate) fn get_color_code(color: &str) -> &str {
    for &(c, e) in COLORS {
        if color == c {
            return e;
        }
    }
    return "";
}

pub(crate) fn get_color_set(color_set: &str) -> &str {
    if color_set == "" {
        return DEFAULT_COLOR;
    }

    for &(c, s) in COLOR_SETS {
        if color_set == c {
            return s;
        }
    }

    return &color_set;
}

pub(crate) const DEFAULT_COLOR: &str = "whi";

const COLORS: &[(&str, &str)] = &[
    ("red", "\x1b[38;5;9m"),
    ("ora", "\x1b[38;5;3m"),
    ("yel", "\x1b[38;5;11m"),
    ("gre", "\x1b[38;5;10m"),
    ("blu", "\x1b[38;5;14m"),
    ("pur", "\x1b[38;5;13m"),
    ("whi", "\x1b[38;5;15m"),

    ("cya", "\x1b[38;5;14m"),
    ("pin", "\x1b[38;5;13m"),
    ("res", "\x1b[0m")
];

const COLOR_SETS: &[(&str, &str)] = &[
    ("tra", "cya,pin,whi,pin,cya"),
    ("gay", "red,ora,yel,gre,blu,pur"),
    ("rainbow", "red,ora,yel,gre,blu,pur")
];

