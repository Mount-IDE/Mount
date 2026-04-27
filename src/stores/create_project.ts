import {create} from "zustand";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void;
    results: Map<string, string | boolean>,
    packages: Map<string, IPackage>
    add_result:(from: string, value: string | boolean)=>void,
    get_result:(from:string)=>string | boolean | undefined,
    has_result: (from:string)=>boolean,
    add_package: (pack: IPackage)=>void
    add_packages: (pack: IPackage[])=>void
    remove_package: (pack: IPackage)=>void
}




export const createProjectStore = create<Type>((set, get) => ({
    page_opened: false,
    results: new Map(),
    packages: new Map(),
    close: () => set({page_opened: false}),
    open: () => set({page_opened: true}),
    add_result: (from, value)=> set(prev=>{
        const results = prev.results;
        results.set(from, value);
        return {
            results: results
        }
    }),
    get_result:(from: string)=> get().results.get(from),
    has_result:(from: string)=>get().results.has(from),
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
    })


}))