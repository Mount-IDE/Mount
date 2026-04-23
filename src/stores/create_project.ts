import {create} from "zustand";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void
}


export const createProjectStore = create<Type>((set, get) => ({
    page_opened: false,
    close: () => set({page_opened: false}),
    open: () => set({page_opened: true})


}))