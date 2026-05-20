import {create} from "zustand";


interface Type{
    directory_icon: string;
    file_icons: [string, string][]
    load:()=>void
}

export const fsAsideIconsStore=create<Type>((set, get)=>({
    directory_icon: "",
    file_icons: [],
    load(): void {

    }

}))