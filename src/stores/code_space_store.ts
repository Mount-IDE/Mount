import {create} from "zustand";

interface Type {
    spaces: ICodeSpace[]
    current: number;
    set_current: (i: number) => void
    add_code_space: () => void
    remove_code_space: (id: number) => void
    add_file_to_code_space: (id: number, cache_id: number, file: OpenedFile) => number
    remove_file_from_code_space: (id: number, file: Opened) => void
    select_current_file: (id: number, id2: number|null)=>void // id of space and id of file
    get_space: (id: number)=>ICodeSpace|null
}

export const codeSpaceStore = create<Type>((set, get) => ({
    add_code_space(): void {
        let spaces = get().spaces
        if (spaces.length == 0) {
            set({
                spaces: [{
                    id: 0,
                    opened_files: [],
                    current_file: null
                }]
            })
        } else {
            let id = spaces[spaces.length - 1].id + 1;
            set({
                spaces: [...spaces, {
                    id, opened_files: [], current_file: null
                }]
            })
        }

    },
    add_file_to_code_space(id: number, cache_id: number, file: OpenedFile): number {
        let spaces = get().spaces;
        if (id == -1) {
            set({
                spaces: [{
                    id: 0, opened_files: [{
                        ...file, id: 0, cache_id
                    }], current_file: 0
                }],
                current: 0
            })
            return 0;
        }
        let id_ = spaces.map(el => el.id)

        if (!id_.includes(id)) {
            set({
                spaces: [...spaces, {
                    id, opened_files: [{...file, id: 0, cache_id}], current_file: 0
                }]
            })
            return 0;
        } else {
            let found = spaces.find(el => el.id == id)!;
            let i = spaces.indexOf(found);
            if (spaces[i] === undefined) {
                return -1;
            }
            if (spaces[i].opened_files.map(el => el.path).includes(file.path)) {
                return -1;
            }
            let last = spaces[i].opened_files;
            const nextId =
                last.length > 0
                    ? last[last.length - 1].id + 1
                    : 0;
            const updated = spaces.map(space => {
                if (space.id !== id) {
                    return space;
                }

                return {
                    ...space,
                    opened_files: [
                        ...space.opened_files,
                        {
                            ...file,
                            id: nextId,
                            cache_id
                        }
                    ],
                    current_file: nextId
                };
            });

            set({
                spaces: updated
            });
            return nextId
        }
    },
    remove_code_space(id: number): void {
        const spaces = get().spaces;
        const res = spaces.filter(el => el.id != id);
        set({
            spaces: res
        })
    },
    remove_file_from_code_space(id: number, file: Opened): void {
        const spaces = get().spaces;
        const space = spaces.find(el => el.id == id);
        console.log(space)
        if (space == undefined) {
            return;
        }
        console.log(spaces, space)
        const file_index = space.opened_files.find(el => el.id == file.id)!;
        const opened = space.opened_files.filter(el => el.id != file.id);
        const i = spaces.indexOf(space);
        space.opened_files = opened;
        if (opened.length == 0) {
            const res = spaces.filter(el => el.id != id)
            set({
                spaces: res
            })
        } else {
            const index = space.opened_files.indexOf(file_index);
            if (index > 0 && index < space.opened_files.length) {
                space.current_file = index + 1
            } else if (index == 0 && space.opened_files.length > 1) {
                space.current_file = 1
            } else if (index == space.opened_files.length && index > 0) {
                space.current_file = index - 1;
            }
            spaces[i] = space;
            set({
                spaces: spaces
            })
        }
    },
    spaces: [],
    current: 0,
    set_current: (i) => set({current: i}),
    select_current_file(id: number, id2: number|null): void {
        const spaces = get().spaces.map(el=>{
            if (el.id!=id){
                return el;
            }
            el.current_file = id2;
            return el;
        })
        set({
            spaces: spaces
        })
    },
    get_space(id: number): ICodeSpace | null {
        const got = get().spaces.find(el=>el.id==id);
        if (!got) {
            return null
        }
        return got!
    }


}))

