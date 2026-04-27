import {create} from "zustand";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void;
    results: Result,
    packages: Map<string, IPackage>
    add_result:(tid: string, sid:number, pid: string, value: string | boolean)=>void,
    // get_result:(tid: string, sid:number, pid: string)=>string | boolean | undefined,
    // has_result: (from:string)=>boolean,
    add_package: (pack: IPackage)=>void
    add_packages: (pack: IPackage[])=>void
    remove_package: (pack: IPackage)=>void
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
    // get_result:(from: string)=> get().results.get(from),
    // has_result:(from: string)=>get().results.has(from),
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