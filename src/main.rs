//imports
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

// Constants
//title
const WINDOW_TITLE: &'static str = "00.Base Code";
//width and height parameters
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

//appstructure
struct VulkanApp;

impl VulkanApp {

    //initializing a normal window
    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        //winit window builder function 
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    //runs every frame IG
    pub fn main_loop(event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                //inputs are handled here ig
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        //escape = exits the program
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                _ => (),
            }

        })
    }
}

fn main() {

    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);

    VulkanApp::main_loop(event_loop);
}
