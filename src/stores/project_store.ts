import {create} from "zustand"


interface Type{
    path_to_current_project: string
    set_path_to_current_project(path: string):void,
    current_project: IProject| null,
    set_current_project(proj: IProject|null):void
}


export const projectStore=create<Type>((set, get)=>({
    path_to_current_project: "",
    current_project: null,
    set_path_to_current_project:(path: string)=>set({
        path_to_current_project:path
    }),
    set_current_project(proj: IProject | null): void {
        set({
            current_project:proj
        })
    }

}))