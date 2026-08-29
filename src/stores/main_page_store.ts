import {create} from "zustand";

interface Type {
    current_group: number;
    groups: Group[]
    set_current_group: (group: number) => void;
    set_groups: (groups_: Group[]) => void;


    filter_string: string

    set_filter_string: (val: string) => void
}


export interface Group {
    id: number,
    name: string
}


export const mainPageStore = create<Type>((set, _) => ({
    filter_string: "",
    set_filter_string(val: string): void {
        set({
            filter_string: val
        })
    },


    current_group: 0,
    groups: [],
    set_current_group(group: number): void {
        // if (group < get().groups.length) {
            set({
                current_group: group
            })
        // }
    }, set_groups(groups_: Group[]): void {
        let groups = [...groups_];
        if (groups.find(el => el.name == "general") == null) {
            if (groups.length === 0) {
                groups = [{id: 0, name: "general"}]
            } else {
                let first_id = groups[0].id;
                groups = [{id: first_id - 1, name: "general"}, ...groups]
            }
        }
        set({
            groups
        })
    }

}))