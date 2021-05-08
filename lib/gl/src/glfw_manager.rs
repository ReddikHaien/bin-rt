
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use glfw::{Action, Context, Key};
use gl;

struct Data{
    instance: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64,glfw::WindowEvent)>
}

static mut DATA: Option<Data> = None;

pub fn initialize_plugin(interface: &mut dyn Interface){
    interface.register_op("op_should_window_close", window_should_close);
    interface.register_op("op_activate_key_polling", activate_key_polling);
    interface.register_op("op_poll_events",poll_events);
    interface.register_op("op_swap_buffers", swap_buffers);
}

pub fn initialize_glfw(title: String, width: u32, height: u32) {
    let instance = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw");
    let mut window = instance.create_window(width, height, &title, glfw::WindowMode::Windowed).expect("Failed to create window");
    window.0.make_current();                                
    gl::load_with(|s| window.0.get_proc_address(s) as *const _);

    unsafe{
        DATA = Some(Data{
            instance,
            window: window.0,
            events: window.1
        });
    }
}

fn window_should_close(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match DATA{
            Some(ref d) => {
                Op::Sync(OpResponse::Buffer(Box::new([match d.window.should_close() {true => 1, _ => 0}])))
            },
            None => panic!("Render not initialized")
        }
    }
}

fn poll_events(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match DATA{
            Some(ref mut d) =>{

                d.instance.poll_events();

                let mut result: Vec<u8> = Vec::new();

                for (_,event) in glfw::flush_messages(&d.events){
                    match event{
                        glfw::WindowEvent::Key(key,scancode,action,_) => {
                            result.push(1u8);        
                            let bytes = scancode.to_be_bytes();
                            result.push(bytes[0]);
                            result.push(bytes[1]);
                            result.push(bytes[2]);
                            result.push(bytes[3]);
                            
                            match action {
                                Action::Release => result .push(0u8),
                                Action::Press => result.push(1u8),
                                Action::Repeat => result.push(2u8),
                            }
                        }
                        _ => panic!("unhandled event {:#?}",event)
                    }
                }
                if result.len() == 0{
                    Op::Sync(OpResponse::Buffer(Box::new([0])))
                }
                else{
                    Op::Sync(OpResponse::Buffer(result.into_boxed_slice()))
                }
                

            },
            _ => panic!("Render not initialized")
        }
    }
}

fn activate_key_polling(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match DATA{
            Some(ref mut d) => {
                d.window.set_key_polling(true);
            },
            None => panic!("Render not initialized") 
        }
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn swap_buffers(_interface: &mut dyn Interface, _zero_copy: Option<ZeroCopyBuf>) -> Op{
    
    unsafe{
        match DATA{
            Some(ref mut d) =>{
                d.window.swap_buffers();
            },
            None => panic!("Render not initialized")
        }
    }
    
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}