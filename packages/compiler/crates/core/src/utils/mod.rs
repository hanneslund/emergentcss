use anyhow::{bail, Result};

use crate::{ast::Value, utils::colors::get_color_value};
use values::*;

mod colors;
pub mod media_queries;
pub mod pseudo_classes;
mod values;

pub fn get_util_css(property: &str, value: &Option<Value>) -> String {
    match (property, value) {
        // Shortcuts?
        ("center", None) => String::from("display:flex;justify-content:center;align-items:center;"),

        // LAYOUT
        // Display
        ("block", None) => String::from("display:block;"),
        ("inline-block", None) => String::from("display:inline-block;"),
        ("inline", None) => String::from("display:inline;"),
        ("flex", None) => String::from("display:flex;"),
        ("inline-flex", None) => String::from("display:inline-flex;"),
        ("grid", None) => String::from("display:grid;"),
        ("inline-grid", None) => String::from("display:inline-grid;"),
        ("hidden", None) => String::from("display:none;"),
        //Overflow
        ("overflow", Some(value)) => format!("overflow:{};", get_overflow_value(value).unwrap()),
        ("overflow-x", Some(value)) => {
            format!("overflow-x:{};", get_overflow_value(value).unwrap())
        }
        ("overflow-y", Some(value)) => {
            format!("overflow-y:{};", get_overflow_value(value).unwrap())
        }
        // Position
        ("static", None) => String::from("position:static;"),
        ("fixed", None) => String::from("position:fixed;"),
        ("absolute", None) => String::from("position:absolute;"),
        ("relative", None) => String::from("position:relative;"),
        ("sticky", None) => String::from("position:sticky;"),
        // Top / Right / Bottom / Left
        ("t", Some(value)) => {
            format!(
                "top:{};",
                try_value_fns(vec![get_auto_value, get_spacing_value, get_percentage_value], value).unwrap()
            )
        }
        ("r", Some(value)) => {
            format!(
                "right:{};",
                try_value_fns(vec![get_auto_value, get_spacing_value, get_percentage_value], value).unwrap()
            )
        }
        ("b", Some(value)) => {
            format!(
                "bottom:{};",
                try_value_fns(vec![get_auto_value, get_spacing_value, get_percentage_value], value).unwrap()
            )
        }
        ("l", Some(value)) => {
            format!(
                "left:{};",
                try_value_fns(vec![get_auto_value, get_spacing_value, get_percentage_value], value).unwrap()
            )
        }
        // Visibility
        ("visible", None) => String::from("visibility:visible;"),
        ("invisible", None) => String::from("visibility:hidden;"),
        // Z-Index
        ("z", Some(value)) => {
            format!(
                "z-index:{};",
                try_value_fns(vec![get_auto_value, |val| get_number_value(val, "")], value).unwrap()
            )
        }

        // FLEXBOX AND GRID
        ("flex", Some(value)) => format!("flex:{};", get_flex_value(value).unwrap()),
        ("flex-b", Some(value)) => {
            format!(
                "flex-basis:{};",
                try_value_fns(vec![get_spacing_value, get_percentage_value], value).unwrap()
            )
        }
        ("flex-d", Some(value)) => {
            format!("flex-direction:{};", get_flex_dir_value(value).unwrap())
        }
        ("flex-w", Some(value)) => format!("flex-wrap:{};", get_flex_wrap_value(value).unwrap()),
        ("flex-g", Some(value)) => {
            format!("flex-grow:{};", get_flex_shrink_grow_value(value).unwrap())
        }
        ("flex-s", Some(value)) => format!(
            "flex-shrink:{};",
            get_flex_shrink_grow_value(value).unwrap()
        ),

        ("grid-c", Some(value)) => format!(
            "grid-template-columns:{};",
            get_grid_cols_rows_value(value).unwrap()
        ),
        ("grid-r", Some(value)) => format!(
            "grid-template-rows:{};",
            get_grid_cols_rows_value(value).unwrap()
        ),
        ("gap", Some(value)) => format!("gap:{};", get_spacing_value(value).unwrap()),
        ("gap-x", Some(value)) => {
            format!("column-gap:{};", get_spacing_value(value).unwrap())
        }
        ("gap-y", Some(value)) => {
            format!("row-gap:{};", get_spacing_value(value).unwrap())
        }

        ("justify-c", Some(value)) => format!(
            "justify-content:{};",
            get_justify_align_content_value(value).unwrap()
        ),
        ("justify-i", Some(value)) => {
            format!("justify-items:{};", get_justify_items_value(value).unwrap())
        }
        ("justify-s", Some(value)) => format!(
            "justify-self:{};",
            try_value_fns(vec![get_justify_items_value, get_auto_value], value).unwrap()
        ),
        ("align-c", Some(value)) => format!(
            "align-content:{};",
            get_justify_align_content_value(value).unwrap()
        ),
        ("align-i", Some(value)) => {
            format!("align-items:{};", get_align_items_value(value).unwrap())
        }
        ("align-s", Some(value)) => format!(
            "align-self:{};",
            try_value_fns(vec![get_align_items_value, get_auto_value], value).unwrap()
        ),

        // SPACING
        ("p", Some(value)) => {
            format!("padding:{};", get_spacing_value(value).unwrap())
        }
        ("p-t", Some(value)) => format!("padding-top:{};", get_spacing_value(value).unwrap()),
        ("p-r", Some(value)) => format!("padding-right:{};", get_spacing_value(value).unwrap()),
        ("p-b", Some(value)) => format!("padding-bottom:{};", get_spacing_value(value).unwrap()),
        ("p-l", Some(value)) => format!("padding-left:{};", get_spacing_value(value).unwrap()),
        ("p-x", Some(value)) => {
            let spacing = get_spacing_value(value).unwrap();
            format!("padding-left:{};padding-right:{};", spacing, spacing)
        }
        ("p-y", Some(value)) => {
            let spacing = get_spacing_value(value).unwrap();
            format!("padding-top:{};padding-bottom:{};", spacing, spacing)
        }
        ("m", Some(value)) => format!(
            "margin:{};",
            try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap()
        ),
        ("m-t", Some(value)) => format!(
            "margin-top:{};",
            try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap()
        ),
        ("m-r", Some(value)) => format!(
            "margin-right:{};",
            try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap()
        ),
        ("m-b", Some(value)) => format!(
            "margin-bottom:{};",
            try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap()
        ),
        ("m-l", Some(value)) => format!(
            "margin-left:{};",
            try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap()
        ),
        ("m-x", Some(value)) => {
            let spacing = try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap();
            format!("margin-left:{};margin-right:{};", spacing, spacing)
        }
        ("m-y", Some(value)) => {
            let spacing = try_value_fns(vec![get_spacing_value, get_auto_value], value).unwrap();
            format!("margin-top:{};margin-bottom:{};", spacing, spacing)
        }

        // SIZING
        ("w", Some(value)) => format!(
            "width:{};",
            try_value_fns(
                vec![get_spacing_value, get_percentage_value, get_width_value],
                value
            )
            .unwrap()
        ),
        ("h", Some(value)) => format!(
            "height:{};",
            try_value_fns(
                vec![get_spacing_value, get_percentage_value, get_height_value],
                value
            )
            .unwrap()
        ),
        ("max-w", Some(value)) => format!("max-width:{};", get_max_width_value(value).unwrap()),

        // TYPOGRAPHY
        ("font-f", Some(value)) => {
            format!("font-family:{};", get_font_family_value(value).unwrap())
        }
        ("font-s", Some(value)) => {
            match value {
                Value::Raw(_) => format!("font-size:{};", get_font_size_value(value).unwrap()),
                Value::Iden(_) => format!("font-size:{};line-height:{};", get_font_size_value(value).unwrap(), get_font_size_line_height_value(value).unwrap()),
            }}
        ("font-w", Some(value)) => {
            format!("font-weight:{};", get_font_weight_value(value).unwrap())
        }
        ("line-h", Some(value)) => {
            format!(
                "line-height:{};",
                try_value_fns(vec![get_line_height_value, get_spacing_value], value).unwrap()
            )
        }
        ("list-none", None) => String::from("list-style-type:none;"),
        ("list-disc", None) => String::from("list-style-type:disc;"),
        ("list-decimal", None) => String::from("list-style-type:decimal;"),
        ("list-inside", None) => String::from("list-style-position:inside;"),
        ("list-outside", None) => String::from("list-style-position:outside;"),
        ("font-italic", None) => String::from("font-style:italic;"),
        ("font-not-italic", None) => String::from("font-style:normal;"),
        ("antialiased", None) => String::from("-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;"),
        ("subpixel-antialiased", None) => String::from("-webkit-font-smoothing:auto;-moz-osx-font-smoothing:auto;"),

        ("text-a", Some(value)) => {
            format!("text-align:{};", get_text_align_value(value).unwrap())
        }
        ("text-c", Some(value)) => {
            format!("color:{};", get_color_value(value).unwrap())
        }
        ("text-underline", None) => String::from("text-decoration:underline;"),
        ("text-line-through", None) => String::from("text-decoration:line-through;"),
        ("text-no-underline", None) => String::from("text-decoration:none;"),
        ("text-uppercase", None) => String::from("text-transform:uppercase;"),
        ("text-lowercase", None) => String::from("text-transform:lowercase;"),
        ("text-capitalize", None) => String::from("text-transform:capitalize;"),
        ("text-normal-case", None) => String::from("text-transform:none;"),

        // BACKGROUNDS
        ("bg-c", Some(value)) => format!("background-color:{};", get_color_value(value).unwrap()),

        // BORDERS
        ("border-r", Some(value)) => {
            format!("border-radius:{};", get_border_radius_value(value).unwrap())
        }
        ("border-w", Some(value)) => {
            format!("border-width:{};", get_number_value(value, "px").unwrap())
        }
        ("border-lw", Some(value)) => {
            format!("border-left-width:{};", get_number_value(value, "px").unwrap())
        }
        ("border-rw", Some(value)) => {
            format!("border-right-width:{};", get_number_value(value, "px").unwrap())
        }
        ("border-tw", Some(value)) => {
            format!("border-top-width:{};", get_number_value(value, "px").unwrap())
        }
        ("border-bw", Some(value)) => {
            format!("border-bottom-width:{};", get_number_value(value, "px").unwrap())
        }
        ("border-c", Some(value)) => {
            format!("border-color:{};", get_color_value(value).unwrap())
        }
        ("border-solid", None) => String::from("border-style:solid;"),
        ("border-dashed", None) => String::from("border-style:dashed;"),
        ("border-dotted", None) => String::from("border-style:dotted;"),
        ("border-double", None) => String::from("border-style:double;"),
        ("border-hidden", None) => String::from("border-style:hidden;"),
        ("border-none", None) => String::from("border-style:none;"),


        // Transitions & Animation
        // tr-p=none
        // tr-de
        // tr-du
        ("transition-none", None) => String::from("transition-property:none;"),
        ("transition-all", None) => String::from("transition-property:all;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition", None) => String::from("transition-property:background-color,border-color,color,fill,stroke,opacity,box-shadow,transform,filter,backdrop-filter;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition-colors", None) => String::from("transition-property:background-color,border-color,color,fill,stroke;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition-opacity", None) => String::from("transition-property:opacity;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition-shadow", None) => String::from("transition-property:box-shadow;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition-transform", None) => String::from("transition-property:transform;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms;"),
        ("transition-duration", Some(value)) => {
            format!("transition-duration:{};", get_number_value(value, "ms").unwrap())
        },
        ("transition-delay", Some(value)) => {
            format!("transition-delay:{};", get_number_value(value, "ms").unwrap())
        },
        ("transition-ease", Some(value)) => {
            format!("transition-timing-function:{};", get_transition_easing_value(value).unwrap())
        },

        //Interactivity
        ("appearance-none", None) => String::from("appearance:none;"),	

        _ => panic!("CSS error: {} {:?}", property, value),
    }
}

fn try_value_fns(value_fns: Vec<fn(&Value) -> Result<String>>, value: &Value) -> Result<String> {
    for value_fn in value_fns {
        if let Ok(val) = value_fn(value) {
            return Ok(val);
        }
    }

    bail!("Unknown value: {:?}", value)
}
