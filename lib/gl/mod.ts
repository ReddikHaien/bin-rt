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

const plugin = Deno.openPlugin(filepath); 
(Deno as any).core.syncOpsCache();

function intToBuf(int: number): Uint8Array{
    return new Uint8Array([int >> 24 & 255, int >> 16 & 255, int >> 8 & 255, int & 255])
}

function bufToInt(buf: Uint8Array): number{
    return buf[0] << 24 | buf[1] << 16 | buf[2] << 8 | buf[3];
}

interface Listener{
    methods: Function[]
}

export enum WindowEvents{
    None = 0,
    KeyEvent = 1,
}

export enum Action{
    Release = 0,
    Press = 1,
    Repeat = 2,
}

export interface Modifiers{
    Shift: boolean,

}


export type KeyListener = (key: string, scancode: number, action: Action, modifiers: number) => void;


const listeners: Map<WindowEvents,Listener> = new Map();


export const Render ={
    initialize(){
        (Deno as any).core.opSync("op_initialize");
    },
    createWindow(){
        const buf: Uint8Array = (Deno as any).core.opSync("op_create_window");
        if (buf.length === 1){
            return 0;
        }
        return bufToInt(buf);
    },
    shouldClose(window: number): boolean{
        const buf: Uint8Array = (Deno as any).core.opSync("op_window_should_close",null, intToBuf(window));
        return buf[0] === 1;
    },
    pollWindow(window: number){
        const events: Uint8Array = (Deno as any).core.opSync("op_poll_events",null,intToBuf(window));
        let i = 0;
        while(i < events.length){
            switch (events[i] as WindowEvents){
                case WindowEvents.KeyEvent: {
                    i++;
                    let scancode = events[i] << 24 | events[i+1] << 16 | events[i+2] << 8 | events[i+3];
                    let action = events[i+4];
                    i+= 5;
                    let list = listeners.get(WindowEvents.KeyEvent)?.methods as KeyListener[];
                    list?.forEach(l => l(String.fromCharCode(scancode),scancode,action,0));
                }
                break;
                case WindowEvents.None: i++; break;
                default: 
                throw new Error("unknown event id " + events[i]);
            }
        }
    },
    makeWindowCurrent(window: number){
        (Deno as any).core.opSync("op_window_make_current",null,intToBuf(window));
    },

    addListener(event: WindowEvents, listener: KeyListener){
        if (!listeners.has(event)){
            listeners.set(event,{methods: []});
        }
        listeners.get(event)?.methods.push(listener);
    },

    cleanUp(){
        Deno.close(plugin);
    },
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