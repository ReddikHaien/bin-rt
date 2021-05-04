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

export const op_render_initialize = ops.op_render_initialize;
export const op_render_create_window = ops.op_render_create_window;
export const op_render_window_make_current = ops.op_render_window_make_current;
export const op_render_poll_events = ops.op_render_window_poll_events;
export const op_render_window_should_close = ops.op_render_window_should_close;
export const op_render_swap_buffers = ops.op_render_swap_buffers;
export const op_render_window_activate_key_polling = ops.op_render_window_activate_key_polling;

export const op_set_clear_color = ops.op_set_clear_color;
export const op_clear = ops.op_clear;

export function closePlugin(){
    Deno.close(plugin);
}