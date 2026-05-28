use anyhow::Result;

fn main() -> Result<()> {
    let name = std::env::args()
        .nth(1)
        .expect(format!("Available demos {:?}", ui_layout_demo::demo::catalog()).as_str());

    ui_layout_demo::run_demo(&name)
}
