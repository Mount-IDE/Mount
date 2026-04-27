import {create} from "zustand";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void;
    results: Result,
    tags: {id: number, name: string}[]
    packages: Map<string, IPackage>
    add_result:(tid: string, sid:number, pid: string, value: string | boolean)=>void,
    // get_result:(tid: string, sid:number, pid: string)=>string | boolean | undefined,
    // has_result: (from:string)=>boolean,
    add_package: (pack: IPackage)=>void
    add_packages: (pack: IPackage[])=>void
    remove_package: (pack: IPackage)=>void
    add_tag: (name: string)=>void
    remove_tag: (id: number)=>void
    change_tag: (id: number, to: string)=>void

}

export interface Result {
    [template: string]: {
        [section: number]: {
            [parameter: string]: string | boolean
        }
    }
}




export const createProjectStore = create<Type>((set, get) => ({
    page_opened: false,
    results: {},
    packages: new Map(),
    tags: [],
    close: () => set({page_opened: false}),
    open: () => set({page_opened: true}),
    add_result: (tid: string, sid:number, pid: string, value)=> set(prev=>{
        const results = prev.results;
        return {
            results: {
                ...results,
                [tid]: {
                    ...results[tid],
                    [sid]: {
                        ...results[tid]?.[sid],
                        [pid]: value
                    }
                }
            }
        }
    }),
    add_package:(pack: IPackage)=> set(prev=>{
        const newMap = new Map(prev.packages);
        newMap.set(pack.id, pack);
        return { packages: newMap };

    }),
    add_packages:(pack: IPackage[])=> set(prev=>{
        const newMap = new Map(prev.packages);
        for (let i of pack){
            newMap.set(i.id, i);
        }
        return {
            packages: newMap
        }
    }),
    remove_package:(pack: IPackage)=> set(prev=>{
        const newMap = new Map(prev.packages);
        newMap.delete(pack.id);
        return { packages: newMap };
    }),
    add_tag: (name: string)=> set(prev=>{
        const id = prev.tags.length>0? prev.tags[prev.tags.length-1].id:0
        return {
            tags: [...prev.tags, {id:id+1, name}]
        }
    }),
    remove_tag: (id: number)=> set(prev=>{
        return {
            tags: prev.tags.filter(el=>el.id!=id)
        }
    }), change_tag:(id: number, to: string)=> set(prev=> {
        return {
            tags: prev.tags.map(el=>el.id==id? {id, name:to}: el)
        }
    })


}))