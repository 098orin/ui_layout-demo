mod app;

use anyhow::Result;
use ui_layout::*;
use winit::event_loop::EventLoop;

use app::{App, Vertex};

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let root = test_layout_node();
    let mut app = App::new(root);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn test_layout_node() -> LayoutNode {
    // ── Toolbar ─────────────────────────────
    let toolbar = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Some(40.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_left: 8.0,
            padding_right: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Status bar ──────────────────────────
    let status = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            height: Some(24.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_left: 8.0,
            padding_right: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Sidebar ─────────────────────────────
    let sidebar = LayoutNode::new(Style {
        display: Display::Block,
        size: SizeStyle {
            width: Some(200.0),
            min_width: Some(160.0),
            max_width: Some(280.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 8.0,
            padding_left: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Editor (main flexible area) ─────────
    let editor = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle {
            flex_grow: 1.0,
            flex_basis: Some(300.0),
            ..Default::default()
        },
        spacing: Spacing {
            padding_top: 8.0,
            padding_left: 8.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ── Main area (Row flex) ─────────────────
    let main_area = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Row,
            },
            item_style: ItemStyle {
                flex_grow: 1.0,
                ..Default::default()
            },
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            row_gap: 0.0,
            column_gap: 8.0, // gap between [ sidebar / editor ]
            spacing: Spacing {
                padding_top: 4.0,
                padding_bottom: 4.0,
                padding_left: 4.0,
                padding_right: 4.0,
                ..Default::default()
            },
            ..Default::default()
        },
        vec![sidebar, editor],
    );

    // ── Root (Column flex) ──────────────────
    LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: FlexDirection::Column,
            },
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            row_gap: 4.0, // gap between [ toolbar / main_area / status ]
            column_gap: 0.0,
            ..Default::default()
        },
        vec![toolbar, main_area, status],
    )
}

pub fn parse_layout(root: &LayoutNode, hue: f32) -> (Vec<Vertex>, Vec<u16>) {
    fn collect(
        fixed_pos: (f32, f32),
        node: &LayoutNode,
        verts: &mut Vec<Vertex>,
        idxs: &mut Vec<u16>,
        base_index: &mut u16,
        hue: f32,
    ) {
        let mut color = hsl_to_rgb(hue % 1.0, 0.6, 0.6);
        if hue == 0.0 {
            color = [0.0, 0.0, 1.0, 1.0];
        }
        let (v, i) = rect_to_vertices(fixed_pos, node.rect, color);
        verts.extend(v);
        // offsets index
        idxs.extend(i.iter().map(|x| x + *base_index));
        *base_index += 4; // 4 vertces per 1 rect

        let fixed_pos = (fixed_pos.0 + node.rect.x, fixed_pos.1 + node.rect.y);

        for (idx, child) in node.children.iter().enumerate() {
            collect(
                fixed_pos,
                child,
                verts,
                idxs,
                base_index,
                hue + 0.1 * (idx as f32 + 1.0),
            );
        }
    }
    let mut verts = vec![];
    let mut idxs = vec![];
    let mut base_index = 0;

    collect(
        (0.0, 0.0),
        root,
        &mut verts,
        &mut idxs,
        &mut base_index,
        hue,
    );
    (verts, idxs)
}

fn rect_to_vertices(fixed_pos: (f32, f32), rect: Rect, color: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
    let x = rect.x + fixed_pos.0;
    let y = rect.y + fixed_pos.1;
    let w = rect.width;
    let h = rect.height;

    let verts = vec![
        Vertex {
            position: [x, y, 0.0],
            color,
        }, // left-top
        Vertex {
            position: [x + w, y, 0.0],
            color,
        }, // right-top
        Vertex {
            position: [x + w, y + h, 0.0],
            color,
        }, // right-bottom
        Vertex {
            position: [x, y + h, 0.0],
            color,
        }, // left-bottom
    ];

    // index (0-3-2, 0-2-1)
    let idxs: Vec<u16> = vec![0, 3, 2, 0, 2, 1];

    (verts, idxs)
}

// HSL → RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 4] {
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }

    [
        hue2rgb(p, q, h + 1.0 / 3.0),
        hue2rgb(p, q, h),
        hue2rgb(p, q, h - 1.0 / 3.0),
        1.0,
    ]
}
