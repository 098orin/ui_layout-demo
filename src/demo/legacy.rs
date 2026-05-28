use ui_layout::*;

fn block_flow() -> Display {
    Display {
        outer: OuterDisplay::Block,
        inner: InnerDisplay::Flow,
    }
}

fn inline_flow() -> Display {
    Display {
        outer: OuterDisplay::Inline,
        inner: InnerDisplay::Flow,
    }
}

fn block_flex() -> Display {
    Display {
        outer: OuterDisplay::Block,
        inner: InnerDisplay::Flex,
    }
}

#[allow(dead_code)]
pub fn inline() -> LayoutNode {
    let fragment1 = ItemFragment::Fragment(Fragment {
        width: 60.0,
        height: 20.0,
    });

    let fragment2 = ItemFragment::Fragment(Fragment {
        width: 70.0,
        height: 25.0,
    });

    let fragment3 = ItemFragment::Fragment(Fragment {
        width: 95.0,
        height: 15.0,
    });

    let inline_node = LayoutNode::with_children(
        Style {
            display: inline_flow(),
            ..Default::default()
        },
        vec![fragment1, fragment2, fragment3],
    );

    let inner = LayoutNode::with_children(Style::default(), vec![inline_node]);

    LayoutNode::with_children(Style::default(), vec![inner])
}

#[allow(dead_code)]
pub fn block() -> LayoutNode {
    let def_style = Style {
        spacing: Spacing {
            margin_top: Length::Px(10.0).into(),
            margin_bottom: Length::Px(10.0).into(),
            margin_left: Length::Px(10.0).into(),
            margin_right: Length::Px(10.0).into(),
            ..Default::default()
        },
        size: SizeStyle {
            ..Default::default()
        },
        ..Default::default()
    };
    let mut root = LayoutNode::new(Style {
        spacing: Spacing {
            ..Default::default()
        },
        ..def_style.clone()
    });

    fn push_child(parent: &mut LayoutNode, style: Style, max: usize, current: usize) {
        if current + 1 < max {
            parent.children.push(LayoutNode::new(style.clone()).into());
            push_child(
                parent.children[0].node_mut().unwrap(),
                style.clone(),
                max,
                current + 1,
            );
        } else {
            parent.children.push(
                LayoutNode::new(Style {
                    size: SizeStyle {
                        height: Length::Px(50.0).into(),
                        ..Default::default()
                    },
                    ..style
                })
                .into(),
            );
        }
    }

    for i in 0..10 {
        root.children
            .push(LayoutNode::new(def_style.clone()).into());
        push_child(
            root.children[i].node_mut().unwrap(),
            def_style.clone(),
            5,
            0,
        );
    }

    root
}

pub fn demo_layout_0_6() -> LayoutNode {
    // ── Header ─────────────────────────────
    let header = LayoutNode::new(Style {
        display: block_flow(),
        size: SizeStyle {
            height: Length::Px(50.0).into(),
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
        display: block_flow(),
        size: SizeStyle {
            height: Length::Px(30.0).into(),
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
        display: block_flow(),
        item_style: ItemStyle {
            flex_grow: 1.0,
            ..Default::default()
        },
        size: SizeStyle {
            min_width: Length::Px(30.0).into(),
            max_width: Length::Px(80.0).into(),
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
        display: block_flow(),
        item_style: ItemStyle {
            flex_grow: 2.0,
            ..Default::default()
        },
        size: SizeStyle {
            min_width: Length::Px(70.0).into(),
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
            display: block_flow(),
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
            display: block_flow(),
            size: SizeStyle {
                min_width: Length::Px(100.0).into(),
                min_height: Length::Px(20.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })],
    );

    // ── Align-self test ───────────────────
    let align_self = LayoutNode::new(Style {
        display: block_flow(),
        item_style: ItemStyle {
            align_self: Some(AlignItems::End),
            ..Default::default()
        },
        size: SizeStyle {
            width: Length::Px(20.0).into(),
            height: Length::Px(30.0).into(),
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Margin auto ──────────────────────
    let margin_auto = LayoutNode::new(Style {
        display: block_flow(),
        size: SizeStyle {
            width: Length::Px(20.0).into(),
            height: Length::Px(30.0).into(),
            ..Default::default()
        },
        spacing: Spacing {
            margin_left: LengthOrAuto::Auto,
            margin_right: LengthOrAuto::Auto,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Main content ─────────────────────

    let content_top = LayoutNode::new(Style {
        display: block_flow(),
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
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            item_style: ItemStyle {
                flex_grow: 1.0,
                flex_basis: Length::Px(100.0).into(),
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
                min_height: Length::Px(80.0).into(),
                ..Default::default()
            },
            row_gap: Length::Px(10.0).into(),
            ..Default::default()
        },
        vec![child_auto, align_self, margin_auto],
    );

    let main_content = LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(8.0).into(),
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
            display: block_flex(),
            flex_direction: FlexDirection::Row,
            column_gap: Length::Px(12.0).into(),
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
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(12.0).into(),
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
                min_height: Length::Px(400.0).into(),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![header, main_area, footer],
    )
}
