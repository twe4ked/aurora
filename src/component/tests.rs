use super::*;
use crate::Shell;

#[test]
fn test_squash_keep_1() {
    let components = vec![
        // Group 1
        Some(Component::Static("a keep".to_string())),
        Some(Component::Computed("b keep".to_string())),
        // Group 2 (Squash)
        Some(Component::Color("red".to_string())),
        None,
        Some(Component::Static("c squash".to_string())),
        Some(Component::ColorReset("reset".to_string())),
        // Group 3
        Some(Component::Color("green".to_string())),
        Some(Component::Static("d keep".to_string())),
        // Group 4
        Some(Component::Color("blue".to_string())),
        Some(Component::Static("e keep".to_string())),
        Some(Component::Computed("f keep".to_string())),
    ];
    let expected = vec![
        // Group 1
        Component::Static("a keep".to_string()),
        Component::Computed("b keep".to_string()),
        // Group 2 (Squash)
        // XXX: Component::Color("red".to_string()),
        // XXX: None,
        // XXX: Component::Static("c squash".to_string()),
        // XXX: Component::ColorReset("reset".to_string()),
        // Group 3
        Component::Color("green".to_string()),
        Component::Static("d keep".to_string()),
        // Group 4
        Component::Color("blue".to_string()),
        Component::Static("e keep".to_string()),
        Component::Computed("f keep".to_string()),
    ];
    assert_eq!(squash(components), expected);
}

#[test]
fn test_squash_keep_empty_end() {
    let components = vec![
        // Group 1
        Some(Component::Static("a keep".to_string())),
        Some(Component::Computed("b keep".to_string())),
        // Group 2
        Some(Component::Color("blue".to_string())),
        Some(Component::Static("c squash".to_string())),
        None,
    ];
    let expected = vec![
        // Group 1
        Component::Static("a keep".to_string()),
        Component::Computed("b keep".to_string()),
        // Group 2
        // XXX: Component::Color("blue".to_string()),
        // XXX: Some(Component::Static("c squash".to_string())),
        // XXX: None,
    ];
    assert_eq!(squash(components), expected);
}

#[test]
fn test_filter_just_static() {
    let group = vec![Some(Component::Static("a keep".to_string()))];
    assert!(should_keep_group(&group));
}

#[test]
fn test_filter_just_static_ignores_color() {
    let group = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a keep".to_string())),
        Some(Component::ColorReset("reset".to_string())),
    ];
    assert!(should_keep_group(&group));
}

#[test]
fn test_filter_static_and_cwd() {
    let group = vec![
        Some(Component::Static("a keep".to_string())),
        Some(Component::Computed("b keep".to_string())),
    ];
    assert!(should_keep_group(&group));
}

#[test]
fn test_filter_static_and_cwd_ignores_color() {
    let group = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a keep".to_string())),
        Some(Component::Computed("b keep".to_string())),
        Some(Component::ColorReset("reset".to_string())),
    ];
    assert!(should_keep_group(&group));
}

#[test]
fn test_filter_static_and_empty() {
    let group = vec![Some(Component::Static("a keep".to_string())), None];
    assert!(!should_keep_group(&group));
}

#[test]
fn test_filter_static_and_empty_removes_colors() {
    let group = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a keep".to_string())),
        None,
        Some(Component::ColorReset("reset".to_string())),
    ];
    assert!(!should_keep_group(&group));
}

#[test]
fn test_filter_static_cwd_and_empty() {
    let group = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a keep".to_string())),
        Some(Component::Computed("b keep".to_string())),
        None,
        Some(Component::ColorReset("reset".to_string())),
    ];
    assert!(should_keep_group(&group));
}

#[test]
fn test_components_from_tokens() {
    use std::collections::HashMap;

    let mut options = HashMap::new();
    options.insert("foo".to_string(), "bar".to_string());

    let mut context = Context::new(Shell::Zsh, 0, None);
    let result = components_from_tokens(
        vec![Token::Component {
            name: token::Component::Jobs,
            options,
        }],
        &mut context,
    );

    assert_eq!(
        result.unwrap_err().to_string(),
        "error: invalid options: foo=bar"
    );
}

#[test]
fn test_into_groups_single_group() {
    let components = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a".to_string())),
        Some(Component::Static("b".to_string())),
        None,
        Some(Component::ColorReset("reset".to_string())),
    ];

    let groups = into_groups(components);

    assert_eq!(
        groups,
        vec![vec![
            Some(Component::Color("green".to_string())),
            Some(Component::Static("a".to_string())),
            Some(Component::Static("b".to_string())),
            None,
            Some(Component::ColorReset("reset".to_string())),
        ],]
    );
}

#[test]
fn test_into_groups_two_groups() {
    let components = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a".to_string())),
        Some(Component::Static("b".to_string())),
        None,
        Some(Component::ColorReset("reset".to_string())),
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a".to_string())),
    ];

    let groups = into_groups(components);

    assert_eq!(
        groups,
        vec![
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a".to_string())),
                Some(Component::Static("b".to_string())),
                None,
                Some(Component::ColorReset("reset".to_string())),
            ],
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a".to_string())),
            ],
        ]
    );
}

#[test]
fn test_into_groups_two_groups_no_reset() {
    let components = vec![
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a".to_string())),
        Some(Component::Static("b".to_string())),
        None,
        Some(Component::Color("green".to_string())),
        Some(Component::Static("a".to_string())),
    ];

    let groups = into_groups(components);

    assert_eq!(
        groups,
        vec![
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a".to_string())),
                Some(Component::Static("b".to_string())),
                None,
            ],
            vec![
                Some(Component::Color("green".to_string())),
                Some(Component::Static("a".to_string())),
            ],
        ]
    );
}

#[test]
fn test_into_groups_two_groups_no_color() {
    let components = vec![
        Some(Component::Static("a".to_string())),
        Some(Component::ColorReset("reset".to_string())),
        Some(Component::Static("b".to_string())),
    ];

    let groups = into_groups(components);

    assert_eq!(
        groups,
        vec![
            vec![
                Some(Component::Static("a".to_string())),
                Some(Component::ColorReset("reset".to_string())),
            ],
            vec![Some(Component::Static("b".to_string())),],
        ]
    );
}
