use anyhow::Result;

fn main() -> Result<()> {
    ui_layout_demo::run_layout(ui_layout_demo::demo::simple_inline_flow())
}
