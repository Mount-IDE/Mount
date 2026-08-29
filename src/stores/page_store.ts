import {create} from "zustand"

export const enum Window{
    Main,
    Project
}

interface Type {
    current: Window
    need_filter: boolean
    openMain():void
    openProject():void,
    setFilter(val:boolean):void
}


const pageStore =
    create<Type>((set, _) => ({
        current: Window.Main,
        need_filter:false,
        openMain:() => set({current: Window.Main}),
        openProject:() => set({current: Window.Project}),
        setFilter: (val)=>set({need_filter:val})

    }))


export default pageStore;