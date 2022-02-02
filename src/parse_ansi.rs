const COLORS: [&str; 256] = [
    "3f4451",
    "e05561",
    "8cc265",
    "d1b652",
    "4aa5f0",
    "c162de",
    "42b3c2",
    "e6e6e6",
    "4f5666",
    "ff616e",
    "a5e075",
    "f0c85d",
    "4dc4ff",
    "de73ff",
    "4cd1e0",
    "d7dae0",
    "000000",
    "00005f",
    "000087",
    "0000af",
    "0000d7",
    "0000ff",
    "005f00",
    "005f5f",
    "005f87",
    "005faf",
    "005fd7",
    "005fff",
    "008700",
    "00875f",
    "008787",
    "0087af",
    "0087d7",
    "0087ff",
    "00af00",
    "00af5f",
    "00af87",
    "00afaf",
    "00afd7",
    "00afff",
    "00d700",
    "00d75f",
    "00d787",
    "00d7af",
    "00d7d7",
    "00d7ff",
    "00ff00",
    "00ff5f",
    "00ff87",
    "00ffaf",
    "00ffd7",
    "00ffff",
    "5f0000",
    "5f005f",
    "5f0087",
    "5f00af",
    "5f00d7",
    "5f00ff",
    "5f5f00",
    "5f5f5f",
    "5f5f87",
    "5f5faf",
    "5f5fd7",
    "5f5fff",
    "5f8700",
    "5f875f",
    "5f8787",
    "5f87af",
    "5f87d7",
    "5f87ff",
    "5faf00",
    "5faf5f",
    "5faf87",
    "5fafaf",
    "5fafd7",
    "5fafff",
    "5fd700",
    "5fd75f",
    "5fd787",
    "5fd7af",
    "5fd7d7",
    "5fd7ff",
    "5fff00",
    "5fff5f",
    "5fff87",
    "5fffaf",
    "5fffd7",
    "5fffff",
    "870000",
    "87005f",
    "870087",
    "8700af",
    "8700d7",
    "8700ff",
    "875f00",
    "875f5f",
    "875f87",
    "875faf",
    "875fd7",
    "875fff",
    "878700",
    "87875f",
    "878787",
    "8787af",
    "8787d7",
    "8787ff",
    "87af00",
    "87af5f",
    "87af87",
    "87afaf",
    "87afd7",
    "87afff",
    "87d700",
    "87d75f",
    "87d787",
    "87d7af",
    "87d7d7",
    "87d7ff",
    "87ff00",
    "87ff5f",
    "87ff87",
    "87ffaf",
    "87ffd7",
    "87ffff",
    "af0000",
    "af005f",
    "af0087",
    "af00af",
    "af00d7",
    "af00ff",
    "af5f00",
    "af5f5f",
    "af5f87",
    "af5faf",
    "af5fd7",
    "af5fff",
    "af8700",
    "af875f",
    "af8787",
    "af87af",
    "af87d7",
    "af87ff",
    "afaf00",
    "afaf5f",
    "afaf87",
    "afafaf",
    "afafd7",
    "afafff",
    "afd700",
    "afd75f",
    "afd787",
    "afd7af",
    "afd7d7",
    "afd7ff",
    "afff00",
    "afff5f",
    "afff87",
    "afffaf",
    "afffd7",
    "afffff",
    "d70000",
    "d7005f",
    "d70087",
    "d700af",
    "d700d7",
    "d700ff",
    "d75f00",
    "d75f5f",
    "d75f87",
    "d75faf",
    "d75fd7",
    "d75fff",
    "d78700",
    "d7875f",
    "d78787",
    "d787af",
    "d787d7",
    "d787ff",
    "d7af00",
    "d7af5f",
    "d7af87",
    "d7afaf",
    "d7afd7",
    "d7afff",
    "d7d700",
    "d7d75f",
    "d7d787",
    "d7d7af",
    "d7d7d7",
    "d7d7ff",
    "d7ff00",
    "d7ff5f",
    "d7ff87",
    "d7ffaf",
    "d7ffd7",
    "d7ffff",
    "ff0000",
    "ff005f",
    "ff0087",
    "ff00af",
    "ff00d7",
    "ff00ff",
    "ff5f00",
    "ff5f5f",
    "ff5f87",
    "ff5faf",
    "ff5fd7",
    "ff5fff",
    "ff8700",
    "ff875f",
    "ff8787",
    "ff87af",
    "ff87d7",
    "ff87ff",
    "ffaf00",
    "ffaf5f",
    "ffaf87",
    "ffafaf",
    "ffafd7",
    "ffafff",
    "ffd700",
    "ffd75f",
    "ffd787",
    "ffd7af",
    "ffd7d7",
    "ffd7ff",
    "ffff00",
    "ffff5f",
    "ffff87",
    "ffffaf",
    "ffffd7",
    "ffffff",
    "080808",
    "121212",
    "1c1c1c",
    "262626",
    "303030",
    "3a3a3a",
    "444444",
    "4e4e4e",
    "585858",
    "626262",
    "6c6c6c",
    "767676",
    "808080",
    "8a8a8a",
    "949494",
    "9e9e9e",
    "a8a8a8",
    "b2b2b2",
    "bcbcbc",
    "c6c6c6",
    "d0d0d0",
    "dadada",
    "e4e4e4",
    "eeeeee",
];

enum Style {
    Bold,
    Italic,
    Underline,
    Blink,
    FgColor(Color),
    BgColor(Color),
}

enum Color {
    Simple(u8),
    RGB(u8, u8, u8),
}

pub fn parse(input: String) -> String {
    let mut out = String::new();
    let mut split = input.split('\x1b');
    out += split.next().unwrap(); // First part is always just string

    let mut styles: Vec<Style> = vec![];

    while let Some(part) = split.next() {
        if part.chars().next() != Some('[') {
            out += &("\x1b".to_owned() + part)
        }
        let split: Vec<&str> = part.split('m').collect();
        if split.len() == 1 {
            out += &("\x1b".to_owned() + part)
        }

        let mut args = split[0][1..].split(';');
        while let Some(arg) = args.next() {
            if arg == "" { styles.clear(); continue; }
            match arg.parse::<u8>() {
                Ok(0) => styles.clear(),
                Ok(1) => styles.push(Style::Bold),
                Ok(3) => styles.push(Style::Italic),
                Ok(4) => styles.push(Style::Underline),
                Ok(5) => styles.push(Style::Blink),
                Ok(22) => styles.retain(|s| if let Style::Bold = s { false } else { true }),
                Ok(23) => styles.retain(|s| if let Style::Italic = s { false } else { true }),
                Ok(24) => styles.retain(|s| if let Style::Underline = s { false } else { true }),
                Ok(25) => styles.retain(|s| if let Style::Blink = s { false } else { true }),
                Ok(30) => styles.push(Style::FgColor(Color::Simple(0))),
                Ok(31) => styles.push(Style::FgColor(Color::Simple(1))),
                Ok(32) => styles.push(Style::FgColor(Color::Simple(2))),
                Ok(33) => styles.push(Style::FgColor(Color::Simple(3))),
                Ok(34) => styles.push(Style::FgColor(Color::Simple(4))),
                Ok(35) => styles.push(Style::FgColor(Color::Simple(5))),
                Ok(36) => styles.push(Style::FgColor(Color::Simple(6))),
                Ok(37) => styles.push(Style::FgColor(Color::Simple(7))),
                Ok(39) => styles.retain(|s| if let Style::FgColor(_) = s { false } else { true }),
                Ok(40) => styles.push(Style::BgColor(Color::Simple(0))),
                Ok(41) => styles.push(Style::BgColor(Color::Simple(1))),
                Ok(42) => styles.push(Style::BgColor(Color::Simple(2))),
                Ok(43) => styles.push(Style::BgColor(Color::Simple(3))),
                Ok(44) => styles.push(Style::BgColor(Color::Simple(4))),
                Ok(45) => styles.push(Style::BgColor(Color::Simple(5))),
                Ok(46) => styles.push(Style::BgColor(Color::Simple(6))),
                Ok(47) => styles.push(Style::BgColor(Color::Simple(7))),
                Ok(49) => styles.retain(|s| if let Style::BgColor(_) = s { false } else { true }),
                Ok(90) => styles.push(Style::FgColor(Color::Simple(8))),
                Ok(91) => styles.push(Style::FgColor(Color::Simple(9))),
                Ok(92) => styles.push(Style::FgColor(Color::Simple(10))),
                Ok(93) => styles.push(Style::FgColor(Color::Simple(11))),
                Ok(94) => styles.push(Style::FgColor(Color::Simple(12))),
                Ok(95) => styles.push(Style::FgColor(Color::Simple(13))),
                Ok(96) => styles.push(Style::FgColor(Color::Simple(14))),
                Ok(97) => styles.push(Style::FgColor(Color::Simple(15))),
                Ok(100) => styles.push(Style::BgColor(Color::Simple(8))),
                Ok(101) => styles.push(Style::BgColor(Color::Simple(9))),
                Ok(102) => styles.push(Style::BgColor(Color::Simple(10))),
                Ok(103) => styles.push(Style::BgColor(Color::Simple(11))),
                Ok(104) => styles.push(Style::BgColor(Color::Simple(12))),
                Ok(105) => styles.push(Style::BgColor(Color::Simple(13))),
                Ok(106) => styles.push(Style::BgColor(Color::Simple(14))),
                Ok(107) => styles.push(Style::BgColor(Color::Simple(15))),
                Ok(38) | Ok(48) => {
                    if let Some(arg2) = args.next() {
                        if let Ok(arg2) = arg2.parse::<u8>() {
                            if arg2 == 2 {
                                let r = if let Some(arg3) = args.next() {
                                    if let Ok(arg3) = arg3.parse::<u8>() {
                                        arg3
                                    } else { continue; }
                                } else { continue; };
                                let g = if let Some(arg4) = args.next() {
                                    if let Ok(arg4) = arg4.parse::<u8>() {
                                        arg4
                                    } else { continue; }
                                } else { continue; };
                                let b = if let Some(arg5) = args.next() {
                                    if let Ok(arg5) = arg5.parse::<u8>() {
                                        arg5
                                    } else { continue; }
                                } else { continue; };
                                styles.push(
                                    if arg == "38" {
                                        Style::FgColor(Color::RGB(r, g, b))
                                    } else {
                                        Style::BgColor(Color::RGB(r, g, b))
                                    }
                                )
                            } else if arg2 == 5 {
                                if let Some(arg3) = args.next() {
                                    if let Ok(arg3) = arg3.parse::<u8>() {
                                        styles.push(
                                            if arg == "38" {
                                                Style::FgColor(Color::Simple(arg3))
                                            } else {
                                                Style::BgColor(Color::Simple(arg3))
                                            }
                                        )
                                    } else { continue; }
                                } else { continue; }
                            } else { continue; }
                        } else { continue; }
                    } else { continue; }
                },
                _ => continue,
            }
        }

        let mut html_style = String::new();
        let mut html_class = String::new();
        for style in &styles {
            match style {
                Style::Bold => html_style += "font-weight: bold;",
                Style::Italic => html_style += "font-style: italic;",
                Style::Underline => html_style += "text-decoration: underline;",
                Style::Blink => html_class += "ansi_blink ",
                Style::FgColor(color) => match color {
                    Color::Simple(code) => {
                        html_style += &format!("--ansi-fg-color: #{};", COLORS[*code as usize]);
                        html_class += "ansi_color ";
                    },
                    Color::RGB(r, g, b) => {
                        html_style += &format!("--ansi-fg-color: rgb({}, {}, {});", r, g, b);
                        html_class += "ansi_color ";
                    },
                },
                Style::BgColor(color) => match color {
                    Color::Simple(code) => html_style += &format!("background-color: #{};", COLORS[*code as usize]),
                    Color::RGB(r, g, b) => html_style += &format!("background-color: rgb({}, {}, {});", r, g, b),
                },
            }
        }

        out += &format!("<span style=\"{}\" class=\"{}\">{}</span>", html_style, html_class, split[1]);
    }

    return out;
}
