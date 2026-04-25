import {create} from "zustand"


interface Type {
    packages: IPackage[]
    templates: ITemplate[]
    currentTemplate: ITemplate | null,

    add_template_to_cache: (t: ITemplate) => void
    add_package_to_cache: (t: IPackage) => void
    clear_templates: () => void
    clear_packages: () => void
    clear_current_template: () => void
}


export const cacheStore = create<Type>((set, get) => ({
    currentTemplate: null,
    packages: [],
    templates: [],
    add_package_to_cache: (t: IPackage) => set(prev => {
        const pack = prev.packages;
        pack.push(t);
        return {
            packages: pack
        }
    })
    ,
    add_template_to_cache: (t: ITemplate) => set(prev => {
        const temp = prev.templates;
        temp.push(t);
        return {
            templates: temp
        }
    }),
    clear_current_template: () => set({
        currentTemplate: null
    }),
    clear_packages: () => set({
        packages: []
    }),
    clear_templates: () => set({
        templates: []
    })


}))
