import {create} from "zustand";

interface Type{
    path_to_creation: string
    set_path: (val: string)=>void


}



export const contextStore=create<Type>((set,get)=>({
    path_to_creation: "",
    set_path(val: string): void {
        set({
            path_to_creation: val
        })
    }

}))