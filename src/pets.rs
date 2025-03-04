/**
 * (c) 2025 KittKat
 */

use crate::colors::get_color_set;

pub(crate) fn get_pet(pet: &str) -> (&str, &str) {
    for &(a, p, c) in ALIASES {
        if pet == a {
            return (get_pet_str(p), get_color_set(c));
        }
    }
    (get_pet_str(pet), "")
}

fn get_pet_str(pet: &str) -> &str {
    for &(k, v) in PETS {
        if pet == k {
            return std::str::from_utf8(v).expect("Invalid UTF-8");
        }
    }
    return "unknown";
}

const ALIASES: &[(&str, &str, &str)] = &[
    ("blahaj", "shark", "tra"),
    ("BLAHAJ", "SHARK", "tra"),
    ("rubberduck", "bird", "yel"),
    ("RUBBERDUCK", "BIRD", "yel")
];

const PETS: &[(&str, &[u8])] = &[
    ("cat", include_bytes!("../../resources/cat.ascii")),

    ("bird", include_bytes!("../../resources/bird.ascii")),
    ("dog", include_bytes!("../../resources/dog.ascii")),
    ("mouse", include_bytes!("../../resources/mouse.ascii")),
    ("shark", include_bytes!("../../resources/shark.ascii")),

    ("BIRD", include_bytes!("../../resources/BIRD.ascii")),
    ("SHARK", include_bytes!("../../resources/SHARK.ascii"))
];

