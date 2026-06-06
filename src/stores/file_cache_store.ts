import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";


interface Type {
    files: FileCache[],
    add_to_cache: (file: FileCacheLight) => void,
    check: (path: string) => [boolean, number];
    make_dirty: (path: string) => void;
    save: (path: string) => void;
    write_file: (path: string, content: string) => void;
    write_file_by_id: (id: number, content: string) => void;
    get_by_path: (path:string)=>FileCache|null;
    get_by_id: (id: number)=>FileCache|null;
    remove: (path: string)=>void;
    move: (from: string, to: string)=>void;
}


export const fileCacheStore =
    create<Type>((set, get) => ({
        add_to_cache(file: FileCacheLight): void {
            const files = get().files
            const exists = files.some(el => el.path === file.path);
            if (exists) {
                return;
            }
            const id =
                files.length > 0
                    ? files[files.length - 1].id + 1
                    : 0;

            const res: FileCache = {
                ...file,
                id
            };

            set({
                files: [...files, res]
            });
        },
        check(path: string): [boolean, number] {
            let files = get().files;
            for (let i of files) {
                if (i.path == path) {
                    return [true, i.id];
                }
            }
            return [false, -1];
        },
        files: [],
        make_dirty(path: string): void {
            const files = get().files.map(el=>{
                if (el.path!=path){
                    return el
                }
                el.is_dirty=true;
                return el
            })
            set({
                files: files
            })
        },
         save(path: string): void {
            const files =  get().files.map( (el)=>{
                if (el.path!=path){
                    return el;
                }
                invoke("write_file", {path: el.path, content: el.content}).then();
                return {
                    ...el,
                    is_dirty: false
                }
            })


            set({
                files:  files
            })
        },
        write_file(path: string, content: string): void {
            const files = get().files.map(file => {
                if (file.path !== path) {
                    return file;
                }

                return {
                    ...file,
                    content,
                    is_dirty: true
                };
            });

            set({
                files
            });
        },
        write_file_by_id(id: number, content: string): void {
            const files = get().files.map(file => {
                if (file.id !== id) {
                    return file;
                }
                return {
                    ...file,
                    content,
                    is_dirty: true
                };
            });
            set({
                files
            });
        },
        get_by_path(path: string): FileCache |null{
            let files = get().files;
            for (let i of files){
                if (i.path==path){
                    return i
                }
            }
            return null
        },
        get_by_id(id: number): FileCache |null{
            let files = get().files;
            for (let i of files){
                if (i.id==id){
                    return i
                }
            }
            return null
        },
        remove(path: string): void {
            const files = get().files.filter(el=>el.path!=path);
            set({
                files: files
            })
        }, move(from: string, to: string): void {
            const files = get().files.map(el=>{
                if (el.path==from){
                    const elem = {...el}
                    elem.path=to;
                    return elem
                }
                return el
            })
            set({
                files: files
            })
        }


    }))