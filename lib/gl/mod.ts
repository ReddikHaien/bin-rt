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


export enum WindowEvents{
    None = 0,
    KeyEvent = 1,
}

export enum Action{
    Release = 0,
    Press = 1,
    Repeat = 2,
}

export enum GlEnums{
    COLOR_BUFFER_BIT = 16384,

    ARRAY_BUFFER = 34962,
    STATIC_DRAW = 35044,
    STATIC_READ = 35035,
}

export interface Modifiers{
    Shift: boolean,

}

export const Render ={
    
    initialize(){
        invokeMethod(ops.opRenderInitialize);
    },
    createWindow(){
        return Window.createWindow();
    },

    setClearColor(r: number, g: number, b: number, a: number){
        let buffer = new ArrayBuffer(16);
        let view = new DataView(buffer);
        view.setFloat32(0,r,false);
        view.setFloat32(4,g,false);
        view.setFloat32(8,b,false);
        view.setFloat32(12,a,false);
        invokeMethod(ops.opSetClearColor,new Uint8Array(buffer));
    },

    clear(mask: number){
        let buffer = intToBuf(mask);
        invokeMethod(ops.opClear,buffer);
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
        const buf: Uint8Array = invokeMethod(ops.opRenderCreateWindow);
        if (buf.length === 1){
            return new Window();
        }
        else{
            return new Window(buf);
        }
    }

    makeCurrent(){
        invokeMethod(ops.opRenderWindowMakeCurrent,this.#windowId);
    }
    
    static setClearColor(r: number, g: number, b: number, a: number){
        Render.setClearColor(r,g,b,a);
    }

    clear(mask: number){
        Render.clear(mask);
    }

    shouldClose(){
        return  invokeMethod(ops.opRenderWindowShouldClose,this.#windowId)[0] === 1;
    }
    swapBuffers(){
        invokeMethod(ops.opRenderSwapBuffers,this.#windowId);
    }

    addListener(listener: listenerTypes){
        let list = this.#listeners.get(listener.event) as cb[];
        if (list == null || list == undefined){
            list = [];
            this.#listeners.set(listener.event,list);
            invokeMethod(ops.opRenderWindowActivateKeyPolling,this.#windowId)
        }

        list.push(listener.listener);
    }

    poll(){
        const events: Uint8Array = invokeMethod(ops.opRenderPollEvents,this.#windowId);
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

export class GlBuffer{
    #bufferId: number;
    #bufferType: number;
    //a bufferid for the buffer on the other side
    #sharedBufferReference: number;
    //an instance of a buffer who is referenced on the other side
    #sharedBufferInstance: Uint8Array;

    private constructor(bufferId: number,bufferType:number){
        this.#bufferId = bufferId ?? 0;
        this.#bufferType = bufferType;
        this.#sharedBufferReference = 0;
        this.#sharedBufferInstance = new Uint8Array(0);
    }
    static createArrayBuffer(){
        let bufferId = invokeMethod(ops.opCreateBuffer);
        return new GlBuffer(bufToInt(bufferId),GlEnums.ARRAY_BUFFER);
    }

    setData(data: Float32Array,usage: GlEnums.STATIC_DRAW | GlEnums.STATIC_READ){
        //layout id target size usage buffer reference
        const buffer = new Uint8Array(4 + 4 + 4 + 4 + 4);
        const view = new DataView(buffer.buffer);
        
        view.setInt32(0,this.#bufferId);
        view.setInt32(4,this.#bufferType);
        view.setInt32(8,data.length*4);
        view.setInt32(12,usage);

        if (data.byteLength > this.#sharedBufferInstance.byteLength || data.byteLength < this.#sharedBufferInstance.length*0.7){
            //needs buffer resize
            
        }

        invokeMethod(ops.opSetBufferData,buffer);
    }
    
}