use anyhow::Result;

fn main() -> Result<()> {
    let name = std::env::args()
        .nth(1)
        .expect(format!("Available demos {:?}", gui_test::demo::catalog()).as_str());

    gui_test::run_demo(&name)
}
