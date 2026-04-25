import {create} from "zustand";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void;
    results: Map<string, string | boolean>
    add_result:(from: string, value: string | boolean)=>void,
    get_result:(from:string)=>string | boolean | undefined,
    has_result: (from:string)=>boolean
}




export const createProjectStore = create<Type>((set, get) => ({
    page_opened: false,
    results: new Map(),
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
    has_result:(from: string)=>get().results.has(from)


}))