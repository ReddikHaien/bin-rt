#[path = "./data.rs"] mod data;

use glfw::{Action, Context, Key};

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::OpResponse;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;
use std::convert::TryInto;

static mut DATA: Option<data::RenderData> = None;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    println!("registrerer metoder");
    interface.register_op("op_initialize", initialize_render);
    interface.register_op("op_create_window", create_new_window);
    interface.register_op("op_window_make_current", window_make_current);
    interface.register_op("op_poll_events",window_poll);
    interface.register_op("op_window_should_close", window_should_close);

    interface.register_op("op_set_clear_color", set_clear_color);
}

pub fn initialize_render(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let ginstance = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw");

    
    unsafe {
        DATA = Some(data::RenderData {
            glfw: ginstance,
            windows: std::collections::BTreeMap::new(),
        });
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

pub fn create_new_window(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    unsafe {
        match DATA {
            Some(ref mut render_data) => {
                let g: &glfw::Glfw = &render_data.glfw;
                let window_tuple = g.create_window(800, 600, "test", glfw::WindowMode::Windowed);

                match window_tuple {
                    Some(window_tuple) => {
                        let index = create_identifier();

                        let w = &mut render_data.windows;
                        w.insert(index, window_tuple);
                        Op::Sync(OpResponse::Buffer(Box::new(index.to_be_bytes())))
                    }
                    _ => panic!("Failed to create window")
                }
            }
            _ => panic!("Render not initialized")
        }
    }
}

pub fn window_make_current(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    match zero_copy {
        Some(b) => {
            if b.len() == 4 {
                let index = u32::from_be_bytes(
                    b[..]
                        .try_into()
                        .expect("Failed to convert slice to array"),
                );

                unsafe{
                     match DATA{
                         Some(ref mut d)=>{
                            let w = &mut d.windows;
                            let tuple = w.get_mut(&index);
                            match tuple{
                                Some((window,_) ) =>{
                                    window.set_key_polling(true);
                                    window.make_current();
                                    gl::load_with(|s| window.get_proc_address(s) as *const _);
                                },
                                None => panic!("No window with the given id {}",index)
                            }    
                         },
                         None => panic!("Render not initialized")
                     }
                }
            }
            Op::Sync(OpResponse::Buffer(Box::new([0])))
        }, 
        _ => panic!("expected int arg, recieved none")
    }
}

pub fn window_should_close(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    let mut should_close: u8 = 1;
    let index = zero_copy_to_int(&zero_copy).expect("Failed to convert buffer to int");
    unsafe{
        match DATA{
            Some(ref d)=>{
                let w = &d.windows;
                let tuple = w.get(&index);
                match tuple{
                    Some((window,_) ) =>{
                        should_close = match window.should_close() {true => 1u8, _ => 0};
                    },
                    None => panic!("No window with the given id {}",index)
                }
            },
            None => panic!("Render not initialized")
        }
    }
    Op::Sync(OpResponse::Buffer(Box::new([should_close])))
}

pub fn window_poll(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    unsafe{
        match DATA{
            Some(ref mut d) =>{

                let index = zero_copy_to_int(&zero_copy).expect("Failed to convert buffer to int");

                d.glfw.poll_events();

                let mut result: Vec<u8> = Vec::new();

                let w = d.windows.get(&index);
                match w{
                    Some(w) =>{
                        for (_,event) in glfw::flush_messages(&w.1){
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
                    None => panic!("No window with id {}", index)
                }

            },
            _ => panic!("Render not initialized")
        }
    }


}


pub fn set_clear_color(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let r = zero_copy_to_float(&zero_copy, 0);
    let g = zero_copy_to_float(&zero_copy, 4);
    let b = zero_copy_to_float(&zero_copy, 8);
    let a = zero_copy_to_float(&zero_copy, 12);
    unsafe{
        gl::ClearColor(r,g,b,a);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))    
}

pub fn clear_window(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op{
    let mask = zero_copy_to_int(&zero_copy).or::<u32>(Ok(0u32)).unwrap();
    unsafe{
        gl::Clear(mask);
    }
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}


pub fn terminate_render(interface: &mut dyn Interface, zero_copy: Option<ZeroCopyBuf>) -> Op {
    Op::Sync(OpResponse::Buffer(Box::new([0])))
}

fn zero_copy_to_float(zero_copy: &Option<ZeroCopyBuf>,index: usize) -> f32{
    match zero_copy{
        Some(d)=>{
            if d.len() - index >= 4{

                 f32::from_le_bytes(
                    d[index..index+4]
                    .try_into().
                    expect("Failed to convert buffer to float")
                 )
            }
            else{
                0f32
            }
        },
        None =>{
            0f32
        } 
    }
}

fn zero_copy_to_int(zero_copy: &Option<ZeroCopyBuf>) -> Result<u32, String>{
    match zero_copy{
        Some(d) =>{
            if d.len() == 4 {
                let index = u32::from_be_bytes(
                    d[..]
                        .try_into()
                        .expect("Failed to convert slice to array"),
                );
                Ok(index)
            }
            else{
                Err(String::from("Expected 4 bytes"))
            }
        },
        _ => Err(String::from("Expected buffer, recieved none"))
    }
}

static mut ID_COUNTER: u32 = 1;
fn create_identifier() -> u32 {
    unsafe {
        let i = ID_COUNTER;
        ID_COUNTER += 1;
        i
    }
}
