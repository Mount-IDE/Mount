import {create} from "zustand"


interface Type{
    files: FileCache[],
    add_to_cache: (file: FileCacheLight)=>void,
    check: (path: string)=>[boolean, number];
}




export const fileCacheStore=
    create<Type>((set, get)=>({
        add_to_cache(file: FileCacheLight): void {
            const files_ = get().files
            const files = new Set(files_);
            const id = files_.length>0?files_[files_.length-1].id+1:0
            const res: FileCache = {...file, id}
            files.add(res);
            set({
                files: [...files]
            })
        },
        check(path: string): [boolean, number] {
            let files = get().files;
            for (let i of files){
                if (i.path==path){
                    return [true, i.id];
                }
            }
            return [false, -1];
        },
        files: [],
        

    }))