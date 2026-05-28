pub mod complex;
pub mod legacy;
pub mod simple;

use ui_layout::LayoutNode;

pub use complex::{complex_dashboard, complex_gallery, complex_nested_stress};
pub use legacy::{block, demo_layout_0_6, inline};
pub use simple::{simple_fixed_panel, simple_inline_flow, simple_nested_blocks};

#[derive(Clone, Copy, Debug)]
pub struct Demo {
    pub name: &'static str,
    pub genre: &'static str,
    pub build: fn() -> LayoutNode,
}

pub fn catalog() -> &'static [Demo] {
    &[
        Demo {
            name: "demo_layout_0_6",
            genre: "legacy",
            build: demo_layout_0_6,
        },
        Demo {
            name: "simple_fixed_panel",
            genre: "simple",
            build: simple_fixed_panel,
        },
        Demo {
            name: "simple_inline_flow",
            genre: "simple",
            build: simple_inline_flow,
        },
        Demo {
            name: "simple_nested_blocks",
            genre: "simple",
            build: simple_nested_blocks,
        },
        Demo {
            name: "complex_dashboard",
            genre: "complex",
            build: complex_dashboard,
        },
        Demo {
            name: "complex_gallery",
            genre: "complex",
            build: complex_gallery,
        },
        Demo {
            name: "complex_nested_stress",
            genre: "complex",
            build: complex_nested_stress,
        },
    ]
}

pub fn find(name: &str) -> Option<Demo> {
    catalog().iter().copied().find(|demo| demo.name == name)
}
