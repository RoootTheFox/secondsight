use std::io::{self, Write};

pub fn secondsightify(t: &str) -> String {
    if t.chars().any(|x| (0xe0000 < (x as u32) && (x as u32) < 0xe007f)) {
        return t.chars().map(|x| {
            if 0xe0000 < (x as u32) && (x as u32) < 0xe007f {
                char::from_u32((x as u32) - 0xe0000).unwrap().to_string()
            } else {
                x.to_string()
            }
        }).collect();
    } else {
        return t.chars().map(|x| {
            if 0x00 < (x as u32) && (x as u32) < 0x7f {
                char::from_u32((x as u32) + 0xe0000).unwrap().to_string()
            } else {
                x.to_string()
            }
        }).collect();
    }
}
