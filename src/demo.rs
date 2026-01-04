use ui_layout::*;

pub fn demo_layout_0_4() -> LayoutNode {
    // ── Header ─────────────────────────────
    let header = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Some(50.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 8.0,
            padding_bottom: 8.0,
            padding_left: 16.0,
            padding_right: 16.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Footer ─────────────────────────────
    let footer = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Some(30.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 4.0,
            padding_bottom: 4.0,
            padding_left: 16.0,
            padding_right: 16.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Sidebar ────────────────────────────
    let sidebar = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            width: Some(200.0),
            min_width: Some(150.0),
            max_width: Some(300.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 8.0,
            padding_bottom: 8.0,
            padding_left: 8.0,
            padding_right: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Auto sizing content ───────────────
    let child_auto = LayoutNode::with_children(
        Style {
            display: Display::Block,
            spacing: Spacing {
                padding_top: 12.0,
                padding_bottom: 12.0,
                padding_left: 12.0,
                padding_right: 12.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![LayoutNode::new(Style {
            display: Display::Block,
            size: SizeStyle {
                min_width: Some(100.0),
                min_height: Some(20.0),
                ..Default::default()
            },
            ..Default::default()
        })],
    );

    // ── Main content ───────────────────────
    let content_top = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Some(120.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 8.0,
            padding_bottom: 8.0,
            padding_left: 8.0,
            padding_right: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let content_bottom = LayoutNode::with_children(
        Style {
            display: Display::Block,
            item_style: ItemStyle {
                flex_grow: 1.0,
                flex_basis: Some(100.0),
                ..Default::default()
            },
            spacing: Spacing {
                padding_top: 8.0,
                padding_bottom: 8.0,
                padding_left: 8.0,
                padding_right: 8.0,
                ..Default::default()
            },
            size: SizeStyle {
                min_height: Some(80.0),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![child_auto],
    );

    let main_content = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            row_gap: 8.0,
            column_gap: 0.0,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: 8.0,
                padding_bottom: 8.0,
                padding_left: 8.0,
                padding_right: 8.0,
                ..Default::default()
            },
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![content_top, content_bottom],
    );

    // ── Main area (Row flex) ───────────────
    let main_area = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Row,
            },
            row_gap: 0.0,
            column_gap: 12.0,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: 8.0,
                padding_bottom: 8.0,
                padding_left: 8.0,
                padding_right: 8.0,
                ..Default::default()
            },
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![sidebar, main_content],
    );

    // ── Root (Column flex) ─────────────────
    LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            row_gap: 12.0,
            column_gap: 0.0,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            spacing: Spacing {
                padding_top: 8.0,
                padding_bottom: 8.0,
                padding_left: 8.0,
                padding_right: 8.0,
                ..Default::default()
            },
            size: SizeStyle {
                min_height: Some(400.0),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![header, main_area, footer],
    )
}
