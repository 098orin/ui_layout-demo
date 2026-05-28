pub mod app;
pub mod demo;

use anyhow::Result;
use app::{App, Vertex};
use ui_layout::*;
use winit::event_loop::EventLoop;

pub fn run_layout(root: LayoutNode) -> Result<()> {
    let _ = env_logger::try_init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(root);
    event_loop.run_app(&mut app)?;

    Ok(())
}

pub fn run_demo(name: &str) -> Result<()> {
    let Some(demo) = demo::find(name) else {
        anyhow::bail!("unknown demo: {name}");
    };

    run_layout((demo.build)())
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
        let color = hsl_to_rgb(hue % 1.0, 0.6, 0.6);

        for box_model in node.layout_box.iter() {
            let (v, i) = rect_to_vertices(fixed_pos, box_model.padding_box, color);
            verts.extend(v);
            idxs.extend(i.iter().map(|x| x + *base_index));
            *base_index += 4;
        }

        if let LayoutBox::BlockBox(box_model) = &node.layout_box {
            let fixed_pos = (
                fixed_pos.0 + box_model.content_box.x,
                fixed_pos.1 + box_model.content_box.y,
            );

            for (idx, child) in node.children.iter().enumerate() {
                let Some(child) = child.node() else {
                    continue;
                };

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
        },
        Vertex {
            position: [x + w, y, 0.0],
            color,
        },
        Vertex {
            position: [x + w, y + h, 0.0],
            color,
        },
        Vertex {
            position: [x, y + h, 0.0],
            color,
        },
    ];

    let idxs: Vec<u16> = vec![0, 3, 2, 0, 2, 1];

    (verts, idxs)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_layout_without_geometry() {
        let root = LayoutNode::new(Style::default());
        let (vertices, indices) = parse_layout(&root, 0.0);
        assert!(vertices.is_empty());
        assert!(indices.is_empty());
    }

    #[test]
    fn demo_catalog_contains_named_entries() {
        let demos = demo::catalog();
        assert!(demos.iter().any(|demo| demo.name == "demo_layout_0_6"));
        assert!(demos.iter().any(|demo| demo.name == "complex_dashboard"));
        assert!(demos.iter().any(|demo| demo.name == "simple_fixed_panel"));
    }

    #[test]
    fn demo_catalog_builds_renderable_layouts() {
        for demo in demo::catalog() {
            let mut root = (demo.build)();
            LayoutEngine::layout(&mut root, 800.0, 600.0);
            let (vertices, indices) = parse_layout(&root, 0.0);

            assert_eq!(
                vertices.len() % 4,
                0,
                "{} generated incomplete quads",
                demo.name
            );
            assert_eq!(
                indices.len() % 6,
                0,
                "{} generated incomplete triangle pairs",
                demo.name
            );
        }
    }
}
