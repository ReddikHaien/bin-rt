import * as ops from "./pluginLoader.ts"; 

function invokeMethod(op: number, buffer?: Uint8Array): Uint8Array{
    return (Deno as any).core.opSync(op,null, buffer);
}

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

export const Render ={
    get COLOR_BUFFER_BIT() {return 16384; },

    initialize(){
        invokeMethod(ops.op_render_initialize);
    },
    createWindow(){
        return Window.createWindow();
    },
    makeWindowCurrent(window: number){
        (Deno as any).core.opSync("op_window_make_current",null,intToBuf(window));
    },


    setClearColor(r: number, g: number, b: number, a: number){
        let buffer = new ArrayBuffer(16);
        let view = new DataView(buffer);
        view.setFloat32(0,r,true);
        view.setFloat32(4,g,true);
        view.setFloat32(8,b,true);
        view.setFloat32(12,a,true);
        (Deno as any).core.opSync("op_set_clear_color",null,new Uint8Array(buffer));
    },

    clear(mask: number){
        let buffer = intToBuf(mask);
        (Deno as any).core.opSync("op_clear",null,buffer);
    },

    swapWindowBuffer(window: number){
        let buffer = intToBuf(window);
        (Deno as any).core.opSync("op_swap_buffers",null,buffer);
    },


    cleanUp(){
        ops.closePlugin();
    },
}  

type cb = (...args: any[]) => void;
export type KeyListener = (key: string, scancode: number, action: Action, modifiers: number) => void;


type listenerTypes = {
    event: WindowEvents.KeyEvent,
    listener: KeyListener
} | {
    event: WindowEvents.None,
    listener: cb
}


export class Window{
    #windowId: Uint8Array;
    #listeners: Map<WindowEvents, cb[]>
    private constructor(windowId?: number|Uint8Array){
        this.#windowId = (windowId instanceof Uint8Array) ? windowId : intToBuf(windowId ?? 0);
        this.#listeners = new Map();
    }

    static createWindow(): Window{
        const buf: Uint8Array = invokeMethod(ops.op_render_create_window);
        if (buf.length === 1){
            return new Window();
        }
        else{
            return new Window(buf);
        }
    }

    shouldClose(){
        return  invokeMethod(ops.op_render_window_should_close,this.#windowId)[0] === 1;
    }
    swapBuffers(){
        invokeMethod(ops.op_render_swap_buffers,this.#windowId);
    }

    addListener(listener: listenerTypes){
        let list = this.#listeners.get(listener.event) as cb[];
        if (list == null || list == undefined){
            list = [];
            this.#listeners.set(listener.event,list);
            invokeMethod(ops.op_render_window_activate_key_polling,this.#windowId)
        }

        list.push(listener.listener);
    }

    poll(){
        const events: Uint8Array = invokeMethod(ops.op_render_poll_events,this.#windowId);
        let i = 0;
        while(i < events.length){
            switch (events[i] as WindowEvents){
                case WindowEvents.KeyEvent: {
                    i++;
                    let scancode = events[i] << 24 | events[i+1] << 16 | events[i+2] << 8 | events[i+3];
                    let action = events[i+4];
                    i+= 5;
                    let list = this.#listeners.get(WindowEvents.KeyEvent) as KeyListener[];
                    list?.forEach(l => l(String.fromCharCode(scancode),scancode,action,0));
                }
                break;
                case WindowEvents.None: i++; break;
                default: 
                throw new Error("unknown event id " + events[i]);
            }
        }
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