mod app;

use anyhow::Result;
use ui_layout::{Display, ItemStyle, LayoutEngine, LayoutNode, Rect, Style};
use winit::event_loop::EventLoop;

use app::{App, Vertex};

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut root = test_layout_node();
    LayoutEngine::layout(&mut root, 800.0, 600.0);
    let v_and_i = parse_layout(&root, 0.0);
    println!("{:?}", v_and_i);
    let mut app = App::new(v_and_i);
    event_loop.run_app(&mut app)?;

    Ok(())
}

pub fn test_layout_node() -> LayoutNode {
    let toolbar = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle::default(),
        width: None,
        height: Some(40.0),
        padding: 0.0,
    });
    let status = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle::default(),
        width: None,
        height: Some(24.0),
        padding: 0.0,
    });

    let sidebar = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle::default(),
        width: Some(200.0),
        height: None,
        padding: 0.0,
    });
    let editor = LayoutNode::new(Style {
        display: Display::Block,
        item_style: ItemStyle::default(),
        width: None,
        height: None,
        padding: 0.0,
    }); // grow
    let main_area = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: ui_layout::FlexDirection::Row,
            },
            item_style: ItemStyle { flex_grow: 1.0 },
            width: None,
            height: None,
            padding: 0.0,
        },
        vec![sidebar, editor],
    );

    let root = LayoutNode::with_children(
        Style {
            display: Display::Flex {
                flex_direction: ui_layout::FlexDirection::Column,
            },
            item_style: ItemStyle { flex_grow: 0.0 },
            width: None,
            height: None,
            padding: 0.0,
        },
        vec![toolbar, main_area, status],
    );
    root
}

fn parse_layout(root: &LayoutNode, hue: f32) -> (Vec<Vertex>, Vec<u16>) {
    fn collect(
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
        let (v, i) = rect_to_vertices(node.rect, color);
        verts.extend(v);
        // offsets index
        idxs.extend(i.iter().map(|x| x + *base_index));
        *base_index += 4; // 4 vertces per 1 rect

        for (idx, child) in node.children.iter().enumerate() {
            collect(
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

    collect(root, &mut verts, &mut idxs, &mut base_index, hue);
    (verts, idxs)
}

fn rect_to_vertices(rect: Rect, color: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
    let x = rect.x;
    let y = rect.y;
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
