import {create} from "zustand";


interface Type {
    file_create_menu: boolean;
    open_file_create_menu: () => void;
    close_file_create_menu: () => void;
    set_file_create_menu: (val: boolean) => void;
}


export const menuStore = create<Type>((set, get) => ({
    file_create_menu: false,
    close_file_create_menu: () => {
        set({
            file_create_menu: false
        })
    },
    open_file_create_menu: () => {
        set({
            file_create_menu: true
        })
    }, set_file_create_menu: (val: boolean) => {
        set({
            file_create_menu: val
        })
    }


}))