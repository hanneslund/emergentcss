use anyhow::{bail, Result};

pub fn get_media_query(name: &str) -> Result<String> {
    Ok(match name {
        "sm" => String::from("(min-width:640px)"),
        "md" => String::from("(min-width:768px)"),
        "lg" => String::from("(min-width:1024px)"),
        "xl" => String::from("(min-width:1280px)"),
        "2xl" => String::from("(min-width:1536px)"),
        "dark" => String::from("(prefers-color-scheme: dark)"),
        "portrait" => String::from("(orientation: portrait)"),
        "landscape" => String::from("(orientation: landscape)"),
        "motion-safe" => String::from("(prefers-reduced-motion: no-preference)"),
        "motion-reduce" => String::from("(prefers-reduced-motion: reduce)"),
        "print" => String::from("print"),
        _ => bail!("Unknown media query: {}", name),
    })
}
