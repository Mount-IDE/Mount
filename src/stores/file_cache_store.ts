import {create} from "zustand"


interface Type{
    files: FileCache[],
    add_to_cache: (file: FileCache)=>void,
    check: (path: string)=>boolean;
}




export const fileCacheStore=
    create<Type>((set, get)=>({
        add_to_cache(file: FileCache): void {
            const files = new Set(get().files);
            files.add(file);
            set({
                files: [...files]
            })
        },
        check(path: string): boolean {
            let files = get().files;
            for (let i of files){
                if (i.path==path){
                    return true;
                }
            }
            return false;
        },
        files: [],
        

    }))