use anyhow::{bail, Result};

use crate::ast::Value;

pub fn get_spacing_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "0" => String::from("0px"),
            "px" => String::from("1px"),
            val => {
                let nr = val.parse::<f32>()?;
                format!("{}rem", nr / 4.0)
            }
        }),
    }
}

pub fn get_overflow_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "auto" => String::from("auto"),
            "hidden" => String::from("hidden"),
            "clip" => String::from("clip"),
            "visible" => String::from("visible"),
            "scroll" => String::from("scroll"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_justify_align_content_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "start" => String::from("flex-start"),
            "end" => String::from("flex-end"),
            "center" => String::from("center"),
            "between" => String::from("space-between"),
            "around" => String::from("space-around"),
            "evenly" => String::from("space-evenly"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_justify_items_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "start" => String::from("start"),
            "end" => String::from("end"),
            "center" => String::from("center"),
            "stretch" => String::from("stretch"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_align_items_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "start" => String::from("flex-start"),
            "end" => String::from("flex-end"),
            "center" => String::from("center"),
            "baseline" => String::from("baseline"),
            "stretch" => String::from("stretch"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_percentage_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => {
            if iden == "full" {
                return Ok(String::from("100%"));
            };

            let nrs: Vec<&str> = iden.split('/').collect();
            if nrs.len() != 2 {
                bail!("Unexpected percentage format");
            }
            let nr1 = nrs[0].parse::<f32>()?;
            let nr2 = nrs[1].parse::<f32>()?;

            Ok(format!("{}%", nr1 / nr2 * 100.0))
        }
    }
}

pub fn get_auto_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "auto" => String::from("auto"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_width_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "screen" => String::from("100vw"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_height_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "screen" => String::from("100vh"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_max_width_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "0" => String::from("0rem"),
            "none" => String::from("none"),
            "xs" => String::from("20rem"),
            "sm" => String::from("24rem"),
            "md" => String::from("28rem"),
            "lg" => String::from("32rem"),
            "xl" => String::from("36rem"),
            "2xl" => String::from("42rem"),
            "3xl" => String::from("48rem"),
            "4xl" => String::from("56rem"),
            "5xl" => String::from("64rem"),
            "6xl" => String::from("72rem"),
            "7xl" => String::from("80rem"),
            "full" => String::from("100%"),
            "min" => String::from("min-content"),
            "max" => String::from("max-content"),
            "fit" => String::from("fit-content"),
            "prose" => String::from("65ch"),
            "screen-sm" => String::from("640px"),
            "screen-md" => String::from("768px"),
            "screen-lg" => String::from("1024px"),
            "screen-xl" => String::from("1280px"),
            "screen-2xl" => String::from("1536px"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_flex_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "1" => String::from("1 1 0%"),
            "auto" => String::from("1 1 auto"),
            "initial" => String::from("0 1 auto"),
            "none" => String::from("none"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_flex_dir_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "row" => String::from("row"),
            "-row" => String::from("row-reverse"),
            "col" => String::from("column"),
            "-col" => String::from("column-reverse"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_flex_wrap_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "wrap" => String::from("wrap"),
            "-wrap" => String::from("wrap-reverse"),
            "nowrap" => String::from("nowrap"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_flex_shrink_grow_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "1" => String::from("1"),
            "0" => String::from("0"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_grid_cols_rows_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => {
            let nr = iden.parse::<u16>()?;
            Ok(format!("repeat({}, minmax(0, 1fr))", nr))
        }
    }
}

pub fn get_font_family_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "sans" => String::from(
                r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji""#,
            ),
            "serif" => {
                String::from(r#"ui-serif, Georgia, Cambria, "Times New Roman", Times, serif"#)
            }
            "mono" => String::from(
                r#"ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#,
            ),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_font_size_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "xs" => String::from("0.75rem"),
            "sm" => String::from("0.875rem"),
            "base" => String::from("1rem"),
            "lg" => String::from("1.125rem"),
            "xl" => String::from("1.25rem"),
            "2xl" => String::from("1.5rem"),
            "3xl" => String::from("1.875rem"),
            "4xl" => String::from("2.25rem"),
            "5xl" => String::from("3rem"),
            "6xl" => String::from("3.75rem"),
            "7xl" => String::from("4.5rem"),
            "8xl" => String::from("6rem"),
            "9xl" => String::from("8rem"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_font_size_line_height_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "xs" => String::from("1rem"),
            "sm" => String::from("1.25rem"),
            "base" => String::from("1.5rem"),
            "lg" => String::from("1.75rem"),
            "xl" => String::from("1.75rem"),
            "2xl" => String::from("2rem"),
            "3xl" => String::from("2.25rem"),
            "4xl" => String::from("2.5rem"),
            "5xl" => String::from("1"),
            "6xl" => String::from("1"),
            "7xl" => String::from("1"),
            "8xl" => String::from("1"),
            "9xl" => String::from("1"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_font_weight_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "thin" | "100" => String::from("100"),
            "extralight" | "200" => String::from("200"),
            "light" | "300" => String::from("300"),
            "normal" | "400" => String::from("400"),
            "medium" | "500" => String::from("500"),
            "semibold" | "600" => String::from("600"),
            "bold" | "700" => String::from("700"),
            "extrabold" | "800" => String::from("800"),
            "black" | "900" => String::from("900"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_text_align_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "left" => String::from("left"),
            "center" => String::from("center"),
            "right" => String::from("right"),
            "justify" => String::from("justify"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_border_radius_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "none" => String::from("0px"),
            "sm" => String::from("0.125rem"),
            "md" => String::from("0.25rem"),
            "lg" => String::from("0.375rem"),
            "xl" => String::from("0.5rem"),
            "2xl" => String::from("0.75rem"),
            "3xl" => String::from("1rem"),
            "4xl" => String::from("1.5rem"),
            "full" => String::from("9999px"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_number_value(value: &Value, unit: &str) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => {
            let nr = iden.parse::<f32>()?;
            Ok(format!("{}{}", nr, unit))
        }
    }
}

pub fn get_transition_easing_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "linear" => String::from("linear"),
            "in" => String::from("cubic-bezier(0.4,0,1,1)"),
            "out" => String::from("cubic-bezier(0,0,0.2,1)"),
            "in-out" => String::from("cubic-bezier(0.4,0,0.2,1)"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}

pub fn get_line_height_value(value: &Value) -> Result<String> {
    match value {
        Value::Raw(raw_value) => Ok(raw_value.clone()),
        Value::Iden(iden) => Ok(match &**iden {
            "none" => String::from("1"),
            "tight" => String::from("1.25"),
            "snug" => String::from("1.375"),
            "normal" => String::from("1.5"),
            "relaxed" => String::from("1.625"),
            "loose" => String::from("2"),
            _ => bail!("Unknown value: {}", iden),
        }),
    }
}
