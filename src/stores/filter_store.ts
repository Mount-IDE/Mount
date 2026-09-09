import {create} from "zustand";


interface Type {
    opened: boolean
    set_opened: (op?: boolean) => void

    activated: boolean
    set_activated: (b: boolean) => void
}


export const filterStore = create<Type>((set, get) => ({
    activated: false,
    set_activated(b: boolean): void {
        set({activated: b})
    },


    set_opened(op?: boolean): void {
        set(prev => {
            if (op != undefined) {
                return {
                    opened: op
                }
            }
            return {
                opened: !prev.opened
            }
        })
    },
    opened: false

}))