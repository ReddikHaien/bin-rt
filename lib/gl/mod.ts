import * as ops from "./pluginLoader.ts"; 

const encoder = new TextEncoder();

function invokeMethod(op: number, buffer?: Uint8Array): Uint8Array{
    return (Deno as any).core.opSync(op,null, buffer);
}


export enum GlEnums{
    COLOR_BUFFER_BIT = 16384,

    ARRAY_BUFFER = 34962,
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
    //=============================== BUFFER =====================//
    createBuffer(): Buffer{
        throw new Error("not implemented 'createBuffer'");
    },
    deleteBuffer(buffer: Buffer){
        throw new Error("not implemented deleteBuffer");
    },
    getBufferParameter(buffer: Buffer, pname: GlEnums): any{
        throw new Error("not implemented getBufferParameter");
    },
    isBuffer(buffer: any): boolean{
        throw new Error("not implemented isBuffer");
    },

    //=============================== SHADER =====================//
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

    
    //=============================== UNIFORMS AND ATTRIBS =======//
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


    // ===================== MISC =======
    clear(mask: number){
        throw new Error("not implemented clear");
    },
    setClearColor(r: number, g: number, b: number, a: number){
        throw new Error("not implemented setClearColor");
    }
}