const encoder = new TextEncoder();

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

let plugin = Deno.openPlugin(filepath); 
(Deno as any).core.syncOpsCache();

export const Render ={
    initialize(){
        (Deno as any).core.opSync("op_initialize");
    }
}  

/*
argTest(...args: any){
        let input = args.map((v: any) => encoder.encode(JSON.stringify(v)));
        (Deno as any).core.opSync("op_arg_test",args,new Float32Array([1,2,3]));
    },
    returnTest(){
        return (Deno as any).core.opSync("op_return_test");
    }

*/