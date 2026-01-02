mod app;

use anyhow::Result;
use app::App;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
