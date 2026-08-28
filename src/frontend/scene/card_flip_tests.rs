use super::{CardFlipPhases, card_flip_phases, card_flip_shader_payload};

#[test]
fn shader_leads_reveal() {
    let showcase = card_flip_phases(0.74, true, true);
    assert!(showcase.shader > 0.9);
    assert_eq!(showcase.surface, 0.0);
    assert_eq!(showcase.content, 0.0);

    let handoff = card_flip_phases(0.82, true, true);
    assert_eq!(handoff.shader, 1.0);
    assert!(handoff.surface < 0.5);
    assert_eq!(handoff.content, 0.0);

    let readable = card_flip_phases(0.94, true, true);
    assert_eq!(readable.shader, 1.0);
    assert_eq!(readable.surface, 1.0);
    assert!(readable.content > 0.5);
    assert_eq!(
        card_flip_phases(1.0, true, true),
        CardFlipPhases { shader: 1.0, surface: 1.0, content: 1.0 }
    );
}

#[test]
fn phases_monotonic_bounded() {
    for (shader, back) in [(true, true), (true, false), (false, true), (false, false)] {
        let mut previous = card_flip_phases(0.0, shader, back);
        for step in 1..=100 {
            let current = card_flip_phases(step as f32 / 100.0, shader, back);
            for value in [current.shader, current.surface, current.content] {
                assert!((0.0..=1.0).contains(&value));
            }
            assert!(current.shader >= previous.shader);
            assert!(current.surface >= previous.surface);
            assert!(current.content >= previous.content);
            previous = current;
        }
    }
}

#[test]
fn phases_skippable() {
    let no_shader = card_flip_phases(0.5, false, true);
    assert_eq!(no_shader.shader, 0.0);
    assert!(no_shader.surface > 0.5);

    let before_handoff = card_flip_phases(0.81, true, false);
    assert_eq!(before_handoff.surface, 0.0);
    let after_handoff = card_flip_phases(0.82, true, false);
    assert_eq!(after_handoff.surface, 1.0);
    assert_eq!(after_handoff.content, 1.0);
}

#[test]
fn inactive_shader_payload() {
    assert_eq!(card_flip_shader_payload(0.0, 4, 2), [0.0; 4]);
    assert_eq!(card_flip_shader_payload(0.001, 4, 2), [0.0; 4]);
    let active = card_flip_shader_payload(0.5, 4, 2);
    assert_eq!(active[0], 0.5);
    assert!(active[1] > 0.0);
    assert_eq!(active[2], 2.0);
    let terminal = card_flip_shader_payload(1.0, 4, 2);
    assert_eq!(terminal[0], 1.0);
    assert_eq!(terminal[2], 2.0);
}
