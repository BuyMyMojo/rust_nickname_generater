#![allow(rustdoc::invalid_rust_codeblocks)] // The code block is only for visual purposes
//! All of the templates built into this lib are meant to resemble a litle bit of rust code.
//!
//! Use `get_all_templates()` to get them all as a Vec or `get_template_by_name()` to get a specific one by name.
//!
//! Current list of example outputs:
//!
//! ```rs
//! username(foo: bar);
//! username.consume_coffee();
//! username.count_brain_cells() //0
//! username.parse::<u64>();
//! #![allow(username)]
//! &mut username
//! &username
//! #![deny(warnings)] yuna();
//! *username
//! Err(username);
//! username(cookies: Vec<cookie>)
//! #![github(username)]
//! Arc<username>
//! Box<username>
//! Mutex<username>
//! !username
//! Ok(username)
//! Ok::<(), username>(())
//! username || something
//! () -> username
//! Self::username();
//! Some("username");
//! unsafe {username();}
//! username::MAX
//! username::MIN
//! &'a evelyn
//! assert!("username");
//! #[cfg(feature = "username"]
//! dbg!("username")
//! #![derive(username)]
//! for x in username
//! impl Debug for username
//! impl Display for username
//! 0..=username
//! username::new();
//! panic!("username");
//! poise::username
//! print!("username");
//! println!("username");
//! 0..username
//! serenity::username
//! std::username
//! tokio::username
//! tracing::username
//! vec![username]
//!
//! ```
use crate::template_struct::*;

/// A list of function looking templates
pub const FUNCTION_TEMPLTES: [NameTemplate; 10] = [
    NameTemplate {
        name: "Deny warnings",
        contents: "#![deny(warnings)] {{ username }}();",
        example: "#![deny(warnings)] yuna();",
        info: NameInfo {
            len: 22,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "impl Debug",
        contents: "impl Debug for {{ username }}",
        example: "impl Debug for Mojo",
        info: NameInfo {
            len: 15,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "impl Display",
        contents: "impl Display for {{ username }}",
        example: "impl Display for Mojo",
        info: NameInfo {
            len: 17,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "Function that takes cookies",
        contents: "{{ username }}(cookies: Vec<cookie>)",
        example: "mojo(cookies: Vec<cookie>)",
        info: NameInfo {
            len: 22,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "(foo: bar) function",
        contents: "{{ username }}(foo: bar);",
        example: "mojo(foo: bar);",
        info: NameInfo {
            len: 11,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "Unsafe function",
        contents: "unsafe {{{ username }}();}",
        example: "unsafe {mojo();}",
        info: NameInfo {
            len: 12,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: ".parse::<u64>();",
        contents: "{{ username }}.parse::<u64>();",
        example: "mojo.parse::<u64>();",
        info: NameInfo {
            len: 16,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: "Self::();",
        contents: "Self::{{ username }}();",
        example: "Self::mojo();",
        info: NameInfo {
            len: 9,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: ".consume_coffee();",
        contents: "{{ username }}.consume_coffee();",
        example: "mojo.consume_coffee();",
        info: NameInfo {
            len: 19,
            name_type: NameType::Function,
        },
    },
    NameTemplate {
        name: ".count_brain_cells() //0",
        contents: "{{ username }}.count_brain_cells() //0",
        example: "mojo.count_brain_cells() //0",
        info: NameInfo {
            len: 24,
            name_type: NameType::Function,
        },
    },
];

/// A list of variable looking templates
pub const VARIABLE_TEMPLTES: [NameTemplate; 12] = [
    NameTemplate {
        name: "Var with &'a lifetime",
        contents: "&'a {{ username }}",
        example: "&'a evelyn",
        info: NameInfo {
            len: 3,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Borrowed var",
        contents: "&{{ username }}",
        example: "&mojo",
        info: NameInfo {
            len: 1,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Inside an arc",
        contents: "Arc<{{ username }}>",
        example: "Arc<mojo>",
        info: NameInfo {
            len: 5,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Inside an box",
        contents: "Box<{{ username }}>",
        example: "Box<mojo>",
        info: NameInfo {
            len: 5,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Inside an mutex",
        contents: "Mutex<{{ username }}>",
        example: "Mutex<mojo>",
        info: NameInfo {
            len: 7,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Borrowed mutable var",
        contents: "&mut {{ username }}",
        example: "&mut mojo",
        info: NameInfo {
            len: 5,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Dereferenced var",
        contents: "*{{ username }}",
        example: "*mojo",
        info: NameInfo {
            len: 1,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Inverted bool",
        contents: "!{{ username }}",
        example: "!mojo",
        info: NameInfo {
            len: 1,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Return type",
        contents: "() -> {{ username }}",
        example: "() -> mojo",
        info: NameInfo {
            len: 6,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Or something",
        contents: "{{ username }} || something",
        example: "mojo || something",
        info: NameInfo {
            len: 13,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Var MAX",
        contents: "{{ username }}::MAX",
        example: "mojo::MAX",
        info: NameInfo {
            len: 5,
            name_type: NameType::Var,
        },
    },
    NameTemplate {
        name: "Var MIN",
        contents: "{{ username }}::MIN",
        example: "mojo::MIN",
        info: NameInfo {
            len: 5,
            name_type: NameType::Var,
        },
    },
];

/// A list of templates with the username inside a function
pub const FUNCTION_VARIABLE_TEMPLTES: [NameTemplate; 10] = [
    NameTemplate {
        name: "Some();",
        contents: r#"Some("{{ username }}");"#,
        example: r#"Some("BuyMyMojo");"#,
        info: NameInfo {
            len: 9,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "Ok()",
        contents: r#"Ok({{ username }})"#,
        example: r#"Ok(BuyMyMojo")"#,
        info: NameInfo {
            len: 4,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "Ok::<(), >(())",
        contents: "Ok::<(), {{ username }}>(())",
        example: "Ok::<(), Mojo>(())",
        info: NameInfo {
            len: 14,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "Err();",
        contents: r#"Err({{ username }});"#,
        example: r#"Err("BuyMyMojo");"#,
        info: NameInfo {
            len: 6,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "new()",
        contents: "{{ username }}::new();",
        example: "BuyMyMojo::new();",
        info: NameInfo {
            len: 8,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "for loop",
        contents: "for x in {{ username }}",
        example: "for x in BuyMyMojo",
        info: NameInfo {
            len: 9,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "range",
        contents: "0..{{ username }}",
        example: "0..BuyMyMojo",
        info: NameInfo {
            len: 3,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "inclusive range",
        contents: "0..={{ username }}",
        example: "0..=BuyMyMojo",
        info: NameInfo {
            len: 4,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "print!",
        contents: r#"print!("{{ username }}");"#,
        example: r#"print!("mojo");"#,
        info: NameInfo {
            len: 11,
            name_type: NameType::FunctionVar,
        },
    },
    NameTemplate {
        name: "println!",
        contents: r#"println!("{{ username }}");"#,
        example: r#"println!("mojo");"#,
        info: NameInfo {
            len: 13,
            name_type: NameType::FunctionVar,
        },
    },
];

/// A list of templates with the username inside a macro
pub const MACRO_TEMPLTES: [NameTemplate; 13] = [
    NameTemplate {
        name: "Allow use",
        contents: "#![allow({{ username }})]",
        example: "#![allow(mojo)]",
        info: NameInfo {
            len: 11,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "cfg feature",
        contents: r#"#[cfg(feature = "{{ username }}"]"#,
        example: r#"#[cfg(feature = "mojo"]"#,
        info: NameInfo {
            len: 19,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "Github macro",
        contents: "#![github({{ username }})]",
        example: "#![github(mojo)]",
        info: NameInfo {
            len: 12,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "derive",
        contents: "#![derive({{ username }})]",
        example: "#![derive(mojo)]",
        info: NameInfo {
            len: 16,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "assert!();",
        contents: r#"assert!("{{ username }}");"#,
        example: r#"assert!("BuyMyMojo");"#,
        info: NameInfo {
            len: 12,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "vec!",
        contents: "vec![{{ username }}]",
        example: "vec![mojo]",
        info: NameInfo {
            len: 6,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "dbg!",
        contents: r#"dbg!("{{ username }}")"#,
        example: r#"dbg!("mojo")"#,
        info: NameInfo {
            len: 8,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "panic!",
        contents: r#"panic!("{{ username }}");"#,
        example: r#"panic!("mojo");"#,
        info: NameInfo {
            len: 11,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "tokio::",
        contents: r#"tokio::{{ username }}"#,
        example: r#"tokio::mojo"#,
        info: NameInfo {
            len: 7,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "std::",
        contents: r#"std::{{ username }}"#,
        example: r#"std::mojo"#,
        info: NameInfo {
            len: 5,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "tracing::",
        contents: r#"tracing::{{ username }}"#,
        example: r#"tracing::mojo"#,
        info: NameInfo {
            len: 9,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "poise::",
        contents: r#"poise::{{ username }}"#,
        example: r#"poise::mojo"#,
        info: NameInfo {
            len: 7,
            name_type: NameType::Macro,
        },
    },
    NameTemplate {
        name: "serenity::",
        contents: r#"serenity::{{ username }}"#,
        example: r#"serenity::mojo"#,
        info: NameInfo {
            len: 10,
            name_type: NameType::Macro,
        },
    },
];

// ! need to redo custom templates
// pub const EXAMPLE_CUSTOM_TEMPLTES: [NameTemplate; 1] = [NameTemplate {
//     name: "Example custom template",
//     contents: "{{ username }} is cool",
//     example: "jeff is cool",
//     info: NameInfo {
//         len: 8,
//         name_type: NameType::Custom("Example"),
//     },
// }];
