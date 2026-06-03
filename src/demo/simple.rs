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

fn fixed_box(width: f32, height: f32) -> LayoutNode {
    LayoutNode::new(Style {
        display: block_flow(),
        size: SizeStyle {
            width: Length::Px(width).into(),
            height: Length::Px(height).into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

pub fn simple_fixed_panel() -> LayoutNode {
    LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(12.0).into(),
            spacing: Spacing {
                padding_top: Length::Px(16.0),
                padding_bottom: Length::Px(16.0),
                padding_left: Length::Px(16.0),
                padding_right: Length::Px(16.0),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            fixed_box(260.0, 48.0),
            fixed_box(180.0, 80.0),
            fixed_box(220.0, 36.0),
        ],
    )
}

pub fn simple_inline_flow() -> LayoutNode {
    let fragments = vec![
        ItemFragment::Fragment(Fragment {
            width: 48.0,
            height: 18.0,
        }),
        ItemFragment::Fragment(Fragment {
            width: 120.0,
            height: 24.0,
        }),
        ItemFragment::Fragment(Fragment {
            width: 72.0,
            height: 20.0,
        }),
        ItemFragment::Fragment(Fragment {
            width: 96.0,
            height: 28.0,
        }),
    ];

    LayoutNode::with_children(
        Style {
            display: block_flow(),
            spacing: Spacing {
                padding_top: Length::Px(20.0),
                padding_bottom: Length::Px(20.0),
                padding_left: Length::Px(20.0),
                padding_right: Length::Px(20.0),
                ..Default::default()
            },
            line_height: Length::Px(20.0),
            ..Default::default()
        },
        vec![LayoutNode::with_children(
            Style {
                display: inline_flow(),
                line_height: Length::Px(20.0),
                ..Default::default()
            },
            fragments,
        )],
    )
}

pub fn simple_nested_blocks() -> LayoutNode {
    LayoutNode::with_children(
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
        vec![LayoutNode::with_children(
            Style {
                display: block_flow(),
                spacing: Spacing {
                    margin_top: Length::Px(8.0).into(),
                    margin_bottom: Length::Px(8.0).into(),
                    margin_left: Length::Px(8.0).into(),
                    margin_right: Length::Px(8.0).into(),
                    padding_top: Length::Px(10.0),
                    padding_bottom: Length::Px(10.0),
                    padding_left: Length::Px(10.0),
                    padding_right: Length::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![fixed_box(96.0, 48.0)],
        )],
    )
}
