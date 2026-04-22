import {create} from "zustand"

export const enum Window{
    Main,
    Project
}

interface Type {
    current: Window
    openMain():void
    openProject():void
}


const pageStore =
    create<Type>((set, get)=>({
        current: Window.Main,
        openMain:() => set({current: Window.Main}),
        openProject:() => set({current: Window.Project}),
    }))


export default pageStore;