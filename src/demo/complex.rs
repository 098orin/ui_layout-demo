use ui_layout::*;

fn block_flow() -> Display {
    Display {
        outer: OuterDisplay::Block,
        inner: InnerDisplay::Flow,
    }
}

fn block_flex() -> Display {
    Display {
        outer: OuterDisplay::Block,
        inner: InnerDisplay::Flex,
    }
}

fn panel(grow: f32, basis: f32, min_height: f32) -> LayoutNode {
    LayoutNode::new(Style {
        display: block_flow(),
        item_style: ItemStyle {
            flex_grow: grow,
            flex_basis: Length::Px(basis).into(),
            ..Default::default()
        },
        size: SizeStyle {
            min_height: Length::Px(min_height).into(),
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
    })
}

pub fn complex_dashboard() -> LayoutNode {
    let header = LayoutNode::new(Style {
        display: block_flow(),
        size: SizeStyle {
            height: Length::Px(56.0).into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let sidebar = LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(8.0).into(),
            item_style: ItemStyle {
                flex_basis: Length::Px(150.0).into(),
                ..Default::default()
            },
            size: SizeStyle {
                min_width: Length::Px(96.0).into(),
                max_width: Length::Px(180.0).into(),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![
            panel(0.0, 0.0, 44.0),
            panel(0.0, 0.0, 44.0),
            panel(1.0, 80.0, 80.0),
        ],
    );

    let cards = LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Row,
            column_gap: Length::Px(10.0).into(),
            align_items: AlignItems::Stretch,
            ..Default::default()
        },
        vec![
            panel(1.0, 120.0, 80.0),
            panel(1.0, 120.0, 80.0),
            panel(1.0, 120.0, 80.0),
        ],
    );

    let content = LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(12.0).into(),
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![cards, panel(1.0, 180.0, 180.0), panel(0.0, 90.0, 90.0)],
    );

    let body = LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Row,
            column_gap: Length::Px(12.0).into(),
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![sidebar, content],
    );

    LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(12.0).into(),
            spacing: Spacing {
                padding_top: Length::Px(14.0),
                padding_bottom: Length::Px(14.0),
                padding_left: Length::Px(14.0),
                padding_right: Length::Px(14.0),
                ..Default::default()
            },
            size: SizeStyle {
                min_height: Length::Px(520.0).into(),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![header, body],
    )
}

pub fn complex_gallery() -> LayoutNode {
    let mut rows = Vec::new();

    for row in 0..3 {
        let mut cells = Vec::new();
        for col in 0..4 {
            cells.push(panel(
                1.0 + col as f32 * 0.2,
                80.0,
                70.0 + row as f32 * 18.0,
            ));
        }

        rows.push(LayoutNode::with_children(
            Style {
                display: block_flex(),
                flex_direction: FlexDirection::Row,
                column_gap: Length::Px(8.0).into(),
                align_items: if row % 2 == 0 {
                    AlignItems::Stretch
                } else {
                    AlignItems::Center
                },
                ..Default::default()
            },
            cells,
        ));
    }

    LayoutNode::with_children(
        Style {
            display: block_flex(),
            flex_direction: FlexDirection::Column,
            row_gap: Length::Px(10.0).into(),
            spacing: Spacing {
                padding_top: Length::Px(16.0),
                padding_bottom: Length::Px(16.0),
                padding_left: Length::Px(16.0),
                padding_right: Length::Px(16.0),
                ..Default::default()
            },
            ..Default::default()
        },
        rows,
    )
}

pub fn complex_nested_stress() -> LayoutNode {
    fn branch(depth: usize, max_depth: usize) -> LayoutNode {
        if depth == max_depth {
            return panel(0.0, 40.0, 28.0 + depth as f32 * 4.0);
        }

        LayoutNode::with_children(
            Style {
                display: block_flex(),
                flex_direction: if depth % 2 == 0 {
                    FlexDirection::Row
                } else {
                    FlexDirection::Column
                },
                row_gap: Length::Px(6.0).into(),
                column_gap: Length::Px(6.0).into(),
                item_style: ItemStyle {
                    flex_grow: 1.0,
                    flex_basis: Length::Px(60.0).into(),
                    ..Default::default()
                },
                spacing: Spacing {
                    padding_top: Length::Px(6.0),
                    padding_bottom: Length::Px(6.0),
                    padding_left: Length::Px(6.0),
                    padding_right: Length::Px(6.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![
                branch(depth + 1, max_depth),
                branch(depth + 1, max_depth),
                panel(0.5, 50.0, 36.0),
            ],
        )
    }

    branch(0, 4)
}
