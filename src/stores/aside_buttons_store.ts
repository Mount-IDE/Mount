import {create} from "zustand"


interface Type{
    left_buttons: IAsideButton[],
    bottom_buttons: IAsideButton[],
    right_buttons: IAsideButton[],

    current_left: IAsideButton |null
    current_right: IAsideButton | null
    current_bottom: IAsideButton | null

    load_left: (bt: IAsideButton[])=>void
    load_bottom: (bt: IAsideButton[])=>void
    load_right: (bt: IAsideButton[])=>void

    add_to_left: (bt: IAsideButton)=>void
    add_to_bottom: (bt: IAsideButton)=>void
    add_to_right: (bt: IAsideButton)=>void

    remove_from_left: (id: number)=>void
    remove_from_bottom: (id: number)=>void
    remove_from_right: (id: number)=>void

    set_current_left_button: (bt: IAsideButton |null)=>void;
    set_current_bottom_button: (bt: IAsideButton| null)=>void;
    set_current_right_button: (bt: IAsideButton| null)=>void;
}


export const asideButtonsStore= create<Type>((set, get)=>({
    left_buttons: [],
    bottom_buttons: [],
    right_buttons: [],
    add_to_bottom(bt: IAsideButton): void {
        const buttons = get().bottom_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({bottom_buttons: [...buttons]})
    },
    add_to_left(bt: IAsideButton): void {
        const buttons = get().left_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({left_buttons: [...buttons]})
    },
    add_to_right(bt: IAsideButton): void {
        const buttons = get().right_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({right_buttons: [...buttons]})
    },
    load_bottom(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.bottom_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                bottom_buttons: [...buttons]
            }
        })
    },
    load_left(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.left_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                left_buttons: [...buttons]
            }
        })
    },
    load_right(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.right_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                right_buttons: [...buttons]
            }
        })
    },
    remove_from_bottom(id: number): void {
        const res = get().bottom_buttons.filter(el=>el.id!=id)
        set({
            bottom_buttons: [...res]
        })
    },
    remove_from_left(id: number): void {
        const res = get().left_buttons.filter(el=>el.id!=id)
        set({
            left_buttons: [...res]
        })
    },
    remove_from_right(id: number): void {
        const res = get().right_buttons.filter(el=>el.id!=id)
        set({
            right_buttons: [...res]
        })
    },
    current_bottom: null,
    current_left: null,
    current_right: null,
    set_current_bottom_button(bt: IAsideButton| null): void {
        set({
            current_bottom: bt
        })
    },
    set_current_left_button(bt: IAsideButton|null): void {
        set({
            current_left: bt
        })
    },
    set_current_right_button(bt: IAsideButton|null): void {
        set({
            current_right: bt
        })
    }


}))