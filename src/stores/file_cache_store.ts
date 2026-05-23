import {create} from "zustand"


interface Type {
    files: FileCache[],
    add_to_cache: (file: FileCacheLight) => void,
    check: (path: string) => [boolean, number];
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


    }))