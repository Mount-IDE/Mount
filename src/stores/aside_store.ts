import React, {ReactElement} from "react";
import {create} from "zustand";

interface Type {
    left_aside: boolean,
    right_aside: boolean,
    bottom: boolean,

    current_left: () => ReactElement | null
    current_right: () => ReactElement | null
    current_bottom: () => ReactElement | null

    toggle_left: toggleCallback
    toggle_right: toggleCallback
    toggle_bottom: toggleCallback

    set_current_left: (elem: ReactElement | null) => void
    set_current_right: (elem: ReactElement | null) => void
    set_current_bottom: (elem: ReactElement | null) => void
}

type prev_ = (prev: boolean) => boolean

export type toggleCallback = ((prev?: prev_) => void)

export const asideStore = create<Type>((set, get) => ({
    bottom: false,
    left_aside: false,
    right_aside: false,
    current_bottom: () => null,
    current_left: () => null,
    current_right: () => null,
    set_current_bottom:(elem: React.ReactElement | null)=>set({
            current_bottom: ()=>elem
        }),
    set_current_left:(elem: React.ReactElement | null)=>set({
        current_left: ()=>elem
    }),
    set_current_right:(elem: React.ReactElement | null)=>set({
        current_left: ()=>elem
    }),
    toggle_bottom(prev: prev_ | undefined): void {
        if (prev!==undefined){
            set({
                bottom: prev(get().bottom)
            })
        }else {
            set({
                bottom: !get().bottom
            })
        }
    }, toggle_left(prev: prev_ | undefined): void {
        if (prev!==undefined){
            set({
                left_aside: prev(get().left_aside)
            })
        }else {
            set({
                left_aside: !get().left_aside
            })
        }
    }, toggle_right(prev: prev_ | undefined): void {
        if (prev!==undefined){
            set({
                right_aside: prev(get().right_aside)
            })
        }else {
            set({
                right_aside: !get().right_aside
            })
        }
    }


}))