import {create} from "zustand";

interface Type{
    current_group: number;
    groups: Group[]
    set_current_group: (group: number)=>void;
    set_groups: (groups: Group[])=>void;
}


export interface Group{
    id: number,
    name: string
}


export const mainPageStore=create<Type>((set, get)=>({
    current_group: 0,
    groups: [],
    set_current_group(group: number): void {
        if (group < get().groups.length){
            set({
                current_group: group
            })
        }
    }, set_groups(groups: Group[]): void {
        set({
            groups
        })
    }

}))