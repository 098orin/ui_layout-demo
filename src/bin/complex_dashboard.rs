use anyhow::Result;

fn main() -> Result<()> {
    gui_test::run_layout(gui_test::demo::complex_dashboard())
}
