import {create} from "zustand";

interface Type {
    opened: boolean
    copy_references: LaunchTemplateReference[]
    active_objects: Set<LaunchObject>
    current_template: LaunchTemplate | null
    //current_launch_ref: LaunchTemplateReference | null

    set_opened: (val: boolean) => void

    changed_references: Set<number>


    current_obj: LaunchObject | null
    clear: () => void
}


export const launchStore = create<Type>((set, _get) => ({
    clear: () => {
        set({
            //    current_launch_ref: null,
            opened: false,
            current_template: null,
            active_objects: new Set(),
            copy_references: [],
            changed_references: new Set()
        })
    },

    current_obj: null,
    current_template: null,
    opened: false,
    // current_launch_ref: null,
    active_objects: new Set(),
    copy_references: [],

    changed_references: new Set(),
    set_opened(val: boolean): void {
        set({opened: val})
    },

}))
