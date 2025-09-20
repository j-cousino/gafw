use std::error::Error;

use winit::event_loop::EventLoop;
use winit::event::WindowEvent;
use winit::application::ApplicationHandler;
use winit::window::Window;

fn main() -> Result<(), Box<dyn Error>>{
    let event_loop = EventLoop::new()?;
    
     event_loop.run_app(&mut App::default())?;
    
    Ok(())
}
#[derive(Default)]
struct App {
    window: Option<Window>,  
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop.create_window(Window::default_attributes()).unwrap()
        );
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },

                _ => (),                
            }        
    }
}