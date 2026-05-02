import {create} from "zustand"
import React from "react";


interface Type{
    left_top_buttons: IAsideButton[],
    left_bottom_buttons: IAsideButton[],
    right_top_buttons: IAsideButton[],

    load_left_top: (bt: IAsideButton[])=>void
    load_left_bot: (bt: IAsideButton[])=>void
    load_right_top: (bt: IAsideButton[])=>void

    add_to_left_top: (bt: IAsideButton)=>void
    add_to_left_bot: (bt: IAsideButton)=>void
    add_to_right_top: (bt: IAsideButton)=>void

    remove_from_left_top: (id: number)=>void
    remove_from_left_bot: (id: number)=>void
    remove_from_right_top: (id: number)=>void
}


export const asideButtonsStore= create<Type>((set, get)=>({
    left_top_buttons: [],
    left_bottom_buttons: [],
    right_top_buttons: [],
    add_to_left_bot(bt: IAsideButton): void {
        const buttons = get().left_top_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({left_top_buttons: [...buttons]})
    },
    add_to_left_top(bt: IAsideButton): void {
        const buttons = get().left_bottom_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({left_bottom_buttons: [...buttons]})
    },
    add_to_right_top(bt: IAsideButton): void {
        const buttons = get().right_top_buttons;
        const id = buttons.map(el=>el.id)
        if (id.includes(bt.id)){
            while (id.includes(bt.id)){
                bt.id+=1
            }
        }
        buttons.push(bt)
        set({right_top_buttons: [...buttons]})
    },

    load_left_bot(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.left_bottom_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                left_bottom_buttons: [...buttons]
            }
        })
    },
    load_left_top(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.left_top_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                left_top_buttons: [...buttons]
            }
        })
    },
    load_right_top(bt: IAsideButton[]): void {
        set(prev=>{
            let buttons = prev.right_top_buttons
            bt.forEach(el=>buttons.push(el));
            buttons.sort((a,b)=>a.id-b.id)
            return {
                right_top_buttons: [...buttons]
            }
        })
    },

    remove_from_left_bot(id: number): void {
        const res = get().left_bottom_buttons.filter(el=>el.id!=id)
        set({
            left_bottom_buttons: [...res]
        })
    },
    remove_from_left_top(id: number): void {
        const res = get().left_top_buttons.filter(el=>el.id!=id)
        set({
            left_top_buttons: [...res]
        })
    },
    remove_from_right_top(id: number): void {
        const res = get().right_top_buttons.filter(el=>el.id!=id)
        set({
            right_top_buttons: [...res]
        })
    }

}))