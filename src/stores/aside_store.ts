import React from "react";
import {create} from "zustand";
import {projectStore} from "./project_store.ts";

interface Type {
    left_aside: boolean,
    right_aside: boolean,
    bottom: boolean,

    current_left: comp
    current_right: comp
    current_bottom: comp

    toggle_left: toggleCallback
    toggle_right: toggleCallback
    toggle_bottom: toggleCallback

    set_current_left: (elem: comp) => void
    set_current_right: (elem: comp) => void
    set_current_bottom: (elem: comp) => void

    clear: () => void

    widgets_results: Record<string, Record<string, IVal>>

    write_results: (widget: string, id: string, val: IVal) => void;

    rewrite_results: (widget: string, ids: string[], val: IVal) => void


    set_results: (rec: Record<string, Record<string, IVal>>) => void
}

// type comp = (props?: { active?: boolean }) => ReactElement | null
export type comp = React.ComponentType<{ active?: boolean }>
type prev_ = (prev: boolean) => boolean

export type toggleCallback = ((prev?: prev_) => void)

export const asideStore = create<Type>((set, get) => ({
    widgets_results: {},

    set_results(rec: Record<string, Record<string, IVal>>): void {

        set({
            widgets_results: rec
        })
    },


    rewrite_results: (widget, id, val) => {
        let res = {...get().widgets_results}
        for (let i of id) {
            if (widget in res) {
                res[widget][i] = val
            }
        }
        set({
            widgets_results: res,
        })

        let proj = {...projectStore.getState().current_project} as IProject | null;
        if (proj) {
            proj!.workspace = {
                ...proj.workspace,
                aside_results: {...res}
            }

            projectStore.getState().save_project(proj)

        }



    },
    write_results: (widget, id, val) => {
        let res = get().widgets_results
        let res2 = {
            ...res,
            [widget]: {
                ...res[widget],
                [id]: val
            }
        }
        set({
            widgets_results: res2
        })
        let proj = {...projectStore.getState().current_project} as IProject | null;
        if (proj) {
            proj!.workspace = {
                ...proj.workspace,
                aside_results: {...res2}
            }

            projectStore.getState().save_project(proj)

        }
    },
    bottom: false,
    left_aside: false,
    right_aside: false,
    clear: () => {
        set({
            bottom: false,
            left_aside: false,
            right_aside: false,
            widgets_results: {}
        })
    },
    current_bottom: () => null,
    current_left: () => null,
    current_right: () => null,
    set_current_bottom:(elem: comp)=>set({
            current_bottom: elem
        }),
    set_current_left:(elem: comp)=>set({
        current_left: elem
    }),
    set_current_right:(elem: comp)=>set({
        current_right: elem
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
    },
    toggle_left(prev: prev_ | undefined): void {
        if (prev!==undefined){
            set({
                left_aside: prev(get().left_aside)
            })
        }else {
            set({
                left_aside: !get().left_aside
            })
        }
    },
    toggle_right(prev: prev_ | undefined): void {
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
