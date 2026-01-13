use ui_layout::*;

pub fn demo_layout_0_6() -> LayoutNode {
    // ── Header ─────────────────────────────
    let header = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Length::Px(50.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: Length::Px(8.0),
            padding_bottom: Length::Px(8.0),
            padding_left: Length::Px(16.0),
            padding_right: Length::Px(16.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Footer ─────────────────────────────
    let footer = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Length::Px(30.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: Length::Px(4.0),
            padding_bottom: Length::Px(4.0),
            padding_left: Length::Px(16.0),
            padding_right: Length::Px(16.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Sidebars ───────────────────────────
    let left_sidebar = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle {
            flex_grow: 1.0,
            ..Default::default()
        },
        size: SizeStyle {
            min_width: Length::Px(30.0),
            max_width: Length::Px(80.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: Length::Px(8.0),
            padding_bottom: Length::Px(8.0),
            padding_left: Length::Px(8.0),
            padding_right: Length::Px(8.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let right_sidebar = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle {
            flex_grow: 2.0,
            ..Default::default()
        },
        size: SizeStyle {
            min_width: Length::Px(70.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: Length::Px(8.0),
            padding_bottom: Length::Px(8.0),
            padding_left: Length::Px(8.0),
            padding_right: Length::Px(8.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Auto sizing content ───────────────
    let child_auto = LayoutNode::with_children(
        Style {
            display: Display::Block,
            spacing: Spacing {
                padding_top: Length::Px(12.0),
                padding_bottom: Length::Px(12.0),
                padding_left: Length::Px(12.0),
                padding_right: Length::Px(12.0),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![LayoutNode::new(Style {
            display: Display::Block,
            size: SizeStyle {
                min_width: Length::Px(100.0),
                min_height: Length::Px(20.0),
                ..Default::default()
            },
            ..Default::default()
        })],
    );

    // ── Align-self test ───────────────────
    let align_self = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle {
            align_self: Some(AlignItems::End),
            ..Default::default()
        },
        size: SizeStyle {
            width: Length::Px(20.0),
            height: Length::Px(30.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Main content ─────────────────────
    let content_top = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Length::Px(120.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: Length::Px(8.0),
            padding_bottom: Length::Px(8.0),
            padding_left: Length::Px(8.0),
            padding_right: Length::Px(8.0),
            ..Default::default()
        },
        ..Default::default()
    });

    let content_bottom = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            item_style: ItemStyle {
                flex_grow: 1.0,
                flex_basis: Length::Px(100.0),
                ..Default::default()
            },
            spacing: Spacing {
                padding_top: Length::Px(8.0),
                padding_bottom: Length::Px(8.0),
                padding_left: Length::Px(8.0),
                padding_right: Length::Px(8.0),
                ..Default::default()
            },
            size: SizeStyle {
                min_height: Length::Px(80.0),
                ..Default::default()
            },
            row_gap: Length::Px(10.0),
            ..Default::default()
        },
        vec![child_auto, align_self],
    );

    let main_content = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            row_gap: Length::Px(8.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: Length::Px(8.0),
                padding_bottom: Length::Px(8.0),
                padding_left: Length::Px(8.0),
                padding_right: Length::Px(8.0),
                ..Default::default()
            },
            item_style: ItemStyle {
                flex_grow: 3.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![content_top, content_bottom],
    );

    // ── Main area (Row flex) ─────────────
    let main_area = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Row,
            },
            column_gap: Length::Px(12.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: Length::Px(8.0),
                padding_bottom: Length::Px(8.0),
                padding_left: Length::Px(8.0),
                padding_right: Length::Px(8.0),
                ..Default::default()
            },
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![left_sidebar, main_content, right_sidebar],
    );

    // ── Root (Column flex) ───────────────
    LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            row_gap: Length::Px(12.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: Length::Px(8.0),
                padding_bottom: Length::Px(8.0),
                padding_left: Length::Px(8.0),
                padding_right: Length::Px(8.0),
                ..Default::default()
            },
            size: SizeStyle {
                min_height: Length::Px(400.0),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![header, main_area, footer],
    )
}
