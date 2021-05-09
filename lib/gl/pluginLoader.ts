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
export const opPushBuffer = ops.op_push_buffer;
export const opPopBuffer = ops.op_pop_buffer;

export function closePlugin(){
    Deno.close(plugin);
}