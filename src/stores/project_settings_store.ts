import {create} from "zustand";
import {launchStore} from "./launch_store.ts";

interface Type {
    new_project_data: IProject | null
    set_project: (proj: IProject | null) => void
    opened: boolean
    set_opened: (val: boolean) => void

    variables: IVar[],
    set_variables: (vars: IVar[]) => void,
    write_var: (var_: IVar) => void,
    write_var_name: (i: number, val: string) => void,
    write_var_val: (i: number, val: IVal) => void,
    main_results: Record<number, string>,
    write_main: (option: number, val: string) => void,
    find_main: (option: number) => string | undefined

    add_variable: (type: "string" | "number" | "boolean") => void;
    rem_variable: (i: number) => void;
}


export const projectSettingsStore = create<Type>((set, get) => ({
    new_project_data: null,
    main_results: {},
    variables: [],
    add_variable(type: "string" | "number" | "boolean"): void {
        set({
            variables: [...get().variables, {
                name: "variable",
                value: type == "number" ? 0 : type == "string" ? "" : false
            }]
        })
    },
    rem_variable: (i) => set({
        variables: get().variables.filter((_, i_) => i != i_)
    }),
    write_var_name(i: number, val: string): void {
        set({
            variables: get().variables.map((el, i_) => i == i_ ? {name: val, value: el.value} : el)
        })
    },
    write_var_val(i: number, val: IVal): void {
        set({
            variables: get().variables.map((el, i_) => i == i_ ? {name: el.name, value: val} : el)
        })
    },

    set_variables(vars: IVar[]): void {
        set({variables: vars})
    },
    write_var(var_: IVar): void {
        set({variables: get().variables.map(el => el.name == var_.name ? var_ : el)})
    },

    find_main: (option: number):
        string | undefined =>
        get().main_results[option],

    write_main(option: number, val: string): void {
        let res = get().main_results
        set({
            main_results: {
                ...res,
                [option]: val
            }
        })
    },


    set_project: (proj) => {
        set({new_project_data: proj})
        if (proj) {
            let cur_launch = proj.workspace.current_launch;
            if (cur_launch != null) {
                let found =
                    proj.workspace.launch_references.find(el => el.id == cur_launch)
                if (found !== undefined) {
                    launchStore.getState().set_current_launch(found!);
                }
            }
        }
    },
    opened: false,
    set_opened: (val: boolean) => set({opened: val})


}))