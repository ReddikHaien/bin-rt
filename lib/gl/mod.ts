import * as ops from "./pluginLoader.ts"; 

const encoder = new TextEncoder();

function invokeMethod(op: number, buffer?: Uint8Array): Uint8Array{
    return (Deno as any).core.opSync(op,null, buffer);
}


export enum GlEnums{
    COLOR_BUFFER_BIT = 16384,

    ARRAY_BUFFER         = 0x8892,
    ELEMENT_ARRAY_BUFFER = 0x8893,

    STATIC_DRAW = 35044,
    STATIC_READ = 35035,
}

interface WindowOptions{
    title: string,
    width: number, 
    heigth: number 
}

function pushBuffer(buffer: Uint8Array){
    invokeMethod(ops.opPushBuffer,buffer);
}

function popBuffer(){
    invokeMethod(ops.opPopBuffer)
}

function createView(bytesize: number){
    let buf = new Uint8Array(bytesize);
    let view = new DataView(buf.buffer);
    return {buf, view};
}
const argumentBuffer = createView(16);

function bufferToU32(buffer: Uint8Array, offset: number){
    return buffer[0+offset] << 24 | buffer[1+offset] << 16 | buffer[2+offset] << 8 | buffer[3+offset];
}

export function initializeRender(options: WindowOptions){
    let nameBuf = encoder.encode(options.title);
    pushBuffer(nameBuf);

    let {buf, view} = createView(8);

    view.setInt32(0,options.width);
    view.setInt32(4,options.heigth);
    
    invokeMethod(ops.opRenderInitialize,buf);
    return gl;
}

export class Program{
    #programId: number;
    constructor(programId: number){
        this.#programId = programId;
    }
    get programId(){
        return this.#programId;
    }
}

export class Shader{
    #shaderId: number;
    constructor(shaderId: number){
        this.#shaderId = shaderId;
    }
    get shaderId(){
        return this.#shaderId;
    }
}

export class UniformLocation{
    #uniformLocation: number;
    constructor(uniformLocation: number){
        this.#uniformLocation = uniformLocation;
    }
    get uniformLocation(){
        return this.#uniformLocation;
    }
}

export class Buffer{
    #bufferId: number;
    constructor(bufferId: number){
        this.#bufferId = bufferId;
    }
    get bufferId(){
        return this.#bufferId;
    }
}

const gl = {

//#region Buffer
    createBuffer(): Buffer{
        return new Buffer(bufferToU32(invokeMethod(ops.opCreateBuffer),0));
    },
    deleteBuffer(buffer: Buffer){
        argumentBuffer.view.setUint32(0,buffer.bufferId);
        invokeMethod(ops.opDeleteBuffer,argumentBuffer.buf);
    },
    getBufferParameter(buffer: Buffer, pname: GlEnums): any{
        throw new Error("not implemented getBufferParameter");
    },
    isBuffer(buffer: any): boolean{
        return buffer instanceof Buffer;
    },
    bindBuffer(target: GlEnums.ARRAY_BUFFER | GlEnums.ELEMENT_ARRAY_BUFFER, buffer: Buffer|null){
        argumentBuffer.view.setUint32(0,target);
        argumentBuffer.view.setUint32(0,buffer?.bufferId ?? 0);
        invokeMethod(ops.opBindBuffer,argumentBuffer.buf);
    },
    bufferData(target: GlEnums.ARRAY_BUFFER | GlEnums.ELEMENT_ARRAY_BUFFER, size: number|ArrayBufferView, usage: GlEnums.STATIC_DRAW|GlEnums.STATIC_READ){
        switch(typeof size){
            case "number": {
                argumentBuffer.view.setUint32(0,target);
                argumentBuffer.view.setUint32(4,size);
                argumentBuffer.view.setUint32(8,usage);
                invokeMethod(ops.opSetBufferDataSize,argumentBuffer.buf);
            } break;
            case "object": {
                if (!ArrayBuffer.isView(size)){
                    throw new Error("expected buffer recieved " + size);
                }
                argumentBuffer.view.setUint32(0,target);
                argumentBuffer.view.setUint32(4,usage);
                pushBuffer(new Uint8Array(size.buffer));
                invokeMethod(ops.opSetBufferDataArr,argumentBuffer.buf);
            } break;
            default: {
                throw new Error("expected number or object recieved " + typeof(size));
            }  
        }
    },
    bufferSubData(target: GlEnums.ARRAY_BUFFER | GlEnums.ELEMENT_ARRAY_BUFFER, offset: number, source: ArrayBufferView, sourceOffset=0){
        argumentBuffer.view.setUint32(0,target);
        argumentBuffer.view.setUint32(4,offset);
        argumentBuffer.view.setUint32(8,sourceOffset);
        pushBuffer(new Uint8Array(source.buffer));
        invokeMethod(ops.opSetBufferSubData,argumentBuffer.buf);
    },
//#endregion

//#region Shader
    attatchShader(program: Program, shader: Shader){
        throw new Error("not implemented attachShader");
    },
    bindAttribLocation(program: Program, index: number, name: string){
        throw new Error("not implemented bindAttribLocation");
    },
    compileShader(shader: Shader){
        throw new Error("not implemented compileShader");
    },
    createShader(): Shader{
        throw new Error("not implemented createShader");
    },
    deleteProgram(program: Program){
        throw new Error("not implemented deleteProgram");
    },
    deleteShader(shader: Shader){
        throw new Error("not implemented deleteShader");
    },
    detachShader(program: Program, shader: Shader){
        throw new Error("not implemented detachShader");
    },
    getAttachedShaders(program: Program): Shader[]{
        throw new Error("not implemented getAattachedShaders");
    },
    getProgramParameter(program: Program, pname: GlEnums): GlEnums|number{
        throw new Error("not implemented getProgramParameter");
    },
    getProgramInfoLog(program: Program): string{
        throw new Error("not implemented getProgramInfoLog");
    },
    getShaderParameter(shader: Shader, pname: GlEnums): GlEnums|number{
        throw new Error("not implemented getShaderParameter");
    },
    getShaderInfoLog(shader: Shader): string{
        throw new Error("not implemented getShaderInfoLog");
    },
    isProgram(program: any): boolean{
        throw new Error("not implemented isProgram");
    },
    isShader(shader: any): boolean{
        throw new Error("not implemented isShader");
    },
    linkProgram(program: Program){
        throw new Error("not implemented linkProgram");
    },
    useProgram(program: Program){
        throw new Error("not implemented useProgram");
    },
    validateProgram(program: Program){
        throw new Error("not implemented validatedProgram");
    },
//#endregion
    
//#region Uniforms and Attributes
    disableVertexAttribArray(index: number){
        throw new Error("not implemented disableVertexAttribArray");
    },
    enableVertexAttribArray(index: number){
        throw new Error("not implemented enableVertexAttribArray");
    },
    getActiveAttrib(program: Program, index: number){
        throw new Error("not implemented getActiveAttrib");
    },
    getActiveUniform(program: Program, index: number){
        throw new Error("not implemented getActiveUniform");
    },
    getAttribLocation(program: Program, name: string){
        throw new Error("not implemented getAttribLocation");
    },
    getUniform(program: Program, location: number){
        throw new Error("not implemented getUniform");
    },
    getUniformLocation(program: Program, name: string): UniformLocation{
        throw new Error("not implemented getUniformLocation");
    },
//#endregion

//#region Drawbuffer operations
    clear(mask: number){
        argumentBuffer.view.setUint32(0,mask);
        invokeMethod(ops.opClear,argumentBuffer.buf);
    },
    clearColor(r: number, g: number, b: number, a: number){
        argumentBuffer.view.setFloat32(0,r);
        argumentBuffer.view.setFloat32(4,b);
        argumentBuffer.view.setFloat32(8,g);
        argumentBuffer.view.setFloat32(12,a);
        invokeMethod(ops.opSetClearColor,argumentBuffer.buf);
    },
//#endregion

//#region GLFW
    //==================== GLFW ==================//
    shouldWindowClose(){
        return invokeMethod(ops.opShouldWindowClose)[0] == 1;
    },
    pollEvents(){
        invokeMethod(ops.opPollEvents);
    },
    swapWindowBuffers(){
        invokeMethod(ops.opSwapWindowBuffers);
    }
//#endregion
}