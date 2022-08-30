//!
//! [!["ay that's where the 'stralian accent comes through"](https://raw.githubusercontent.com/BuyMyMojo/rust_nickname_generater/master/images/TheReasonTheNameIsSpeltLikeThatOhMyThisIsALongFileName.png)](https://s3.buymymojo.net/ShareX/2022/08/01/17/rust%20nickname%20genera.wav)
//!
//! > Yes I am australian :)
//!
//! This is a super simple lib I made for practice.
//!
//! The usernames generated are based on the names we all have in the [Serenity/Poise discord](https://discord.gg/serenity-rs) and the [rust community discord](https://discord.gg/rust-lang-communit)
//!
//! ## Basic use:
//! ```rust
//! use rust_nickname_generater::generate_random_name;
//!
//! // Generate a name that will fit in Discord
//! println!("{}", generate_random_name("mojo".to_string(), 32).unwrap());
//! ```

use new_string_template::template::Template;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use regex::Regex;
use std::{collections::HashMap, ops::Add};

pub mod error;
pub mod template_struct;
pub mod templates;

use crate::error::{Error, Result};

use crate::template_struct::{NameTemplate, NameType};
use crate::templates::{
    FUNCTION_TEMPLTES, FUNCTION_VARIABLE_TEMPLTES, MACRO_TEMPLTES, VARIABLE_TEMPLTES,
};

// ? I don't think I need to make this public?
// The following regex requires at least one space between "{{" and "}}" and allows variables with spaces
static STRING_TEMPLATE_MATCHER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?mi)\{\{\s+([^\}]+)\s+\}\}").unwrap());

/// Given a useranem and `NameTemplate` output the rendered name
///
/// ```rust
/// use rust_nickname_generater::template_struct::*;
/// use rust_nickname_generater::{generate_name, get_template_by_name};
///
/// // Search for specific template by name
/// let template: Option<NameTemplate> = get_template_by_name("Deny warnings".to_string());
///
/// let nickname = generate_name("Mojo".to_string(), template.unwrap()).unwrap();
///
///  assert_eq!(nickname, "#![deny(warnings)] Mojo();")
///
/// ```
pub fn generate_name(username: String, template: NameTemplate) -> Result<String> {
    let templ = Template::new(template.contents).with_regex(&STRING_TEMPLATE_MATCHER);
    let data = {
        let mut map = HashMap::new();
        map.insert("username", username);
        map
    };

    match templ.render(&data) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

/// Generate a random name that fits within a specified character limit
///
/// ```rust
/// use rust_nickname_generater::generate_random_name;
///
/// // Generate a name that will fit in Discord
/// println!("{}", generate_random_name("mojo".to_string(), 32).unwrap());
/// ```
pub fn generate_random_name(username: String, char_limit: u32) -> Result<String> {
    // Make sure name isn't too long/limit is too small
    if username.len() > char_limit.try_into().unwrap() {
        return Err(Error::LengthLimit);
    }

    let all_templates = get_all_templates();
    let mut valid_templates: Vec<NameTemplate> = Vec::new();

    for temp in all_templates {
        let total_len: u32 = temp.info.len.add(username.len() as u32);

        // Only push templates that can possibly fit within limit after being rendered
        if total_len.le(&char_limit) {
            valid_templates.push(temp);
        }
    }

    // Error if the name is too large
    if valid_templates.is_empty() {
        return Err(Error::NoValidName);
    }

    let template = valid_templates.choose(&mut rand::thread_rng()).unwrap();

    generate_name(username, *template)
}

/// Returns a Vec<String> of all template names
#[must_use]
pub fn get_all_template_names() -> Vec<String> {
    let templates = get_all_templates();

    let mut names: Vec<String> = Vec::new();

    for temp in templates {
        names.push(temp.name.to_string())
    }

    names.sort();

    names
}

/// Returns a Vec<String> of all template examples
#[must_use]
pub fn get_all_template_examples() -> Vec<String> {
    let templates = get_all_templates();

    let mut examples: Vec<String> = Vec::new();

    for temp in templates {
        examples.push(temp.example.to_string())
    }

    examples.sort();

    examples
}

// TODO: Add fuzzy search?
/// Returns an option with a template with the given name
#[must_use]
pub fn get_template_by_name(name: String) -> Option<NameTemplate<'static>> {
    let templates = get_all_templates();

    // Search for specific template by name
    templates.into_iter().find(|x| *x.name == name)
}

/// Returns a Vec of all built in templates
///
/// ```rust
/// use rust_nickname_generater::get_all_templates;
/// use rust_nickname_generater::template_struct::NameTemplate;
///
/// let templates: Vec<NameTemplate> = get_all_templates();
///
/// println!("{:?}", templates.first());
/// ```
#[must_use]
pub fn get_all_templates() -> Vec<NameTemplate<'static>> {
    let mut templates: Vec<NameTemplate> = Vec::new();

    for temp in FUNCTION_TEMPLTES {
        templates.push(temp)
    }

    for temp in VARIABLE_TEMPLTES {
        templates.push(temp)
    }

    for temp in FUNCTION_VARIABLE_TEMPLTES {
        templates.push(temp)
    }

    for temp in MACRO_TEMPLTES {
        templates.push(temp)
    }

    templates
}

/// Returns a Vec of templates matching specified type
///
/// ```rust
/// use rust_nickname_generater::get_templates_of_type;
/// use rust_nickname_generater::template_struct::{NameType, NameTemplate};
///
/// let templates: Vec<NameTemplate> = get_templates_of_type(NameType::Function);
///
/// println!("{:?}", templates.first());
/// ```
#[must_use]
pub fn get_templates_of_type(name_type: NameType) -> Vec<NameTemplate<'static>> {
    let mut templates: Vec<NameTemplate> = Vec::new();

    match name_type {
        NameType::Macro => {
            let con = MACRO_TEMPLTES;

            for temp in con {
                templates.push(temp)
            }
        }
        NameType::Var => {
            let con = VARIABLE_TEMPLTES;

            for temp in con {
                templates.push(temp)
            }
        }
        NameType::FunctionVar => {
            let con = FUNCTION_VARIABLE_TEMPLTES;

            for temp in con {
                templates.push(temp)
            }
        }
        NameType::Function => {
            let con = FUNCTION_TEMPLTES;

            for temp in con {
                templates.push(temp)
            }
        }
    }

    templates
}

#[cfg(test)]
mod tests {
    use super::error::Error;
    use super::*;

    #[test]
    fn function_template() {
        let templates = get_templates_of_type(NameType::Function);
        let temp = templates.first().unwrap();
        let result = generate_name("yuna".to_string(), *temp).unwrap();
        assert_eq!(result, "#![deny(warnings)] yuna();".to_string());
    }

    #[test]
    fn var_template() {
        let templates = get_templates_of_type(NameType::Var);
        let temp = templates.first().unwrap();
        let result = generate_name("evelyn".to_string(), *temp).unwrap();
        assert_eq!(result, "&'a evelyn".to_string());
    }

    #[test]
    fn func_var_template() {
        let templates = get_templates_of_type(NameType::FunctionVar);
        let temp = templates.first().unwrap();
        let result = generate_name("BuyMyMojo".to_string(), *temp).unwrap();
        assert_eq!(result, r#"Some("BuyMyMojo");"#.to_string());
    }

    #[test]
    fn macro_template() {
        let templates = get_templates_of_type(NameType::Macro);
        let temp = templates.first().unwrap();
        let result = generate_name("mojo".to_string(), *temp).unwrap();
        assert_eq!(result, "#![allow(mojo)]".to_string());
    }

    #[test]
    fn too_small_line_limite() {
        let result = generate_random_name("mojo".to_string(), 1);
        assert_eq!(result.err(), Some(Error::LengthLimit));
    }

    #[test]
    fn no_valid_template() {
        let result =
            generate_random_name("ThisNameWon'tHaveAValidUsernameTemplate".to_string(), 39);
        assert_eq!(result.err(), Some(Error::NoValidName));
    }

    #[test]
    fn random_name() {
        let result = generate_random_name("mojo".to_string(), 32);
        assert!(result.is_ok());
    }

    #[test]
    fn list_add_examples() {
        let result = get_all_template_examples();
        println!("{:#?}", result);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_all_templates() {
        let result = get_all_template_names();
        let mut fails = Vec::new();

        for name in result {
            let temp = get_template_by_name(name).unwrap();

            match generate_name("username".to_string(), temp) {
                Ok(n) => match n.as_str() {
                    "#![deny(warnings)] username();" => {
                        println!("#![deny(warnings)] yuna();")
                    }
                    "&'a username" => {
                        println!("&'a evelyn")
                    }
                    n => {
                        println!("{n}");
                    }
                },
                Err(e) => fails.push(e),
            }
        }

        assert!(fails.is_empty());
    }

    #[test]
    fn search_for_specific_template() {
        // Search for specific template by name
        let template: Option<NameTemplate> = get_template_by_name("Deny warnings".to_string());

        let nickname = generate_name("Mojo".to_string(), template.unwrap()).unwrap();

        assert_eq!(nickname, "#![deny(warnings)] Mojo();")
    }
}
