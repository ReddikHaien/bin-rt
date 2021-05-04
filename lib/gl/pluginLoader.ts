const filename = "gl";
let suffix = ".so";
let prefix = "lib";

if (Deno.build.os === "windows") {
    suffix = ".dll";
    prefix = "";
}
else if (Deno.build.os === "darwin") {
    suffix = ".dylib";
}
const filepath = `./lib/gl/target/debug/${prefix}${filename}${suffix}`;

const plugin = Deno.openPlugin(filepath); 
(Deno as any).core.syncOpsCache();

let ops:{[x: string]: number}  = (Deno as any).core.ops();

export const opRenderInitialize = ops.op_render_initialize;
export const opRenderCreateWindow = ops.op_render_create_window;
export const opRenderWindowMakeCurrent = ops.op_render_window_make_current;
export const opRenderPollEvents = ops.op_render_window_poll_events;
export const opRenderWindowShouldClose = ops.op_render_window_should_close;
export const opRenderSwapBuffers = ops.op_render_swap_buffers;
export const opRenderWindowActivateKeyPolling = ops.op_render_window_activate_key_polling;

export const opCreateBuffer = ops.op_create_buffer;
export const opSetBufferData = ops.op_set_buffer_data;

export const opSetClearColor = ops.op_set_clear_color;
export const opClear = ops.op_clear;


export function closePlugin(){
    Deno.close(plugin);
}