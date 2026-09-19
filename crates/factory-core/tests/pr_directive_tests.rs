use factory_core::PRDirective;

#[test]
fn test_parse_darkgravity_mentions() {
    // Standard @darkgravity case-insensitive
    let directive = PRDirective::parse("Hey @darkgravity can you explain this change?");
    assert_eq!(
        directive,
        Some(PRDirective::Interact {
            prompt: "can you explain this change?".to_string()
        })
    );

    let directive_upper = PRDirective::parse("Please check this @DarkGravity");
    assert_eq!(
        directive_upper,
        Some(PRDirective::Interact {
            prompt: "Please check this".to_string()
        })
    );
}

#[test]
fn test_parse_antigravity_alias() {
    let directive = PRDirective::parse("@antigravity /validate");
    assert_eq!(directive, Some(PRDirective::Validate));

    let directive_query = PRDirective::parse("Hello @antigravity review the security posture");
    assert_eq!(
        directive_query,
        Some(PRDirective::Interact {
            prompt: "review the security posture".to_string()
        })
    );
}

#[test]
fn test_parse_slash_commands_with_and_without_tag() {
    let d1 = PRDirective::parse("@dark-gravity /spec create user auth");
    assert_eq!(
        d1,
        Some(PRDirective::Spec {
            prompt: "create user auth".to_string()
        })
    );

    let d2 = PRDirective::parse("@darkgravity /refine add error handling");
    assert_eq!(
        d2,
        Some(PRDirective::Refine {
            instruction: "add error handling".to_string()
        })
    );

    let d3 = PRDirective::parse("@darkgravity /retry");
    assert_eq!(d3, Some(PRDirective::Retry));

    let d4 = PRDirective::parse("@darkgravity /status");
    assert_eq!(d4, Some(PRDirective::Status));
}

#[test]
fn test_ignore_comments_without_tag() {
    let d = PRDirective::parse("Looks good to me, ready to merge!");
    assert_eq!(d, None);

    let d2 = PRDirective::parse("We should optimize this loop for performance.");
    assert_eq!(d2, None);
}
