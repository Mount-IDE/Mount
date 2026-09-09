import {create} from "zustand";


interface Type {
    opened: boolean
    set_opened: (op?: boolean) => void
}


export const filterStore = create<Type>((set, get) => ({
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