import {create} from "zustand";
import {ModalProps} from "../components/common/Modal.tsx";


interface Type {
    file_create_menu: boolean;
    open_file_create_menu: () => void;
    close_file_create_menu: () => void;
    set_file_create_menu: (val: boolean) => void;
    modal: boolean,
    modal_settings: ModalProps | null

    open_modal: (settings: ModalProps)=>void;
    close_modal: ()=>void;
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
    },
    close_modal(): void {
        set({
            modal: false,
        })
    },
    modal: false,
    modal_settings: null,
    open_modal(settings: ModalProps): void {
        set({
            modal: true,
            modal_settings: settings
        })
    }


}))