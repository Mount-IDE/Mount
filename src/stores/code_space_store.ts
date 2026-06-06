import {create} from "zustand";

function pushToStack(stack: number[], id: number): number[] {
    return [...stack.filter(el => el !== id), id];
}

function normalizeStack(space: ICodeSpace): number[] {
    const openedIds = new Set(space.opened_files.map(el => el.id));
    const stack = space.opened_files_stack ?? [];
    const normalized = stack.filter(id => openedIds.has(id));

    if (space.current_file !== null && openedIds.has(space.current_file)) {
        return pushToStack(normalized, space.current_file);
    }

    return normalized;
}

interface Type {
    spaces: ICodeSpace[]
    current: number;
    set_current: (i: number) => void
    add_code_space: () => number
    remove_code_space: (id: number) => void
    add_file_to_code_space: (id: number, cache_id: number, file: OpenedFile) => number
    remove_file_from_code_space: (id: number, file: Opened) => void
    select_current_file: (id: number, id2: number | null) => void // id of space and id of file
    get_space: (id: number) => ICodeSpace | null
}

export const codeSpaceStore = create<Type>((set, get) => ({
    add_code_space(): number {
        let spaces = get().spaces
        if (spaces.length == 0) {
            set({
                spaces: [{
                    id: 0,
                    opened_files: [],
                    current_file: null,
                    opened_files_stack: []
                }]
            })
            return 0
        } else {
            let id = spaces[spaces.length - 1].id + 1;
            set({
                spaces: [...spaces, {
                    id, opened_files: [], current_file: null, opened_files_stack: []
                }]
            })
            return id
        }

    },
    add_file_to_code_space(id: number, cache_id: number, file: OpenedFile): number {
        let spaces = get().spaces;
        if (id == -1) {
            set({
                spaces: [{
                    id: 0, opened_files: [{
                        ...file, id: 0, cache_id
                    }], current_file: 0, opened_files_stack: [0]
                }],
                current: 0
            })
            return 0;
        }
        let id_ = spaces.map(el => el.id)

        if (!id_.includes(id)) {
            set({
                spaces: [...spaces, {
                    id, opened_files: [{...file, id: 0, cache_id}], current_file: 0, opened_files_stack: [0]
                }]
            })
            return 0;
        } else {
            let found = spaces.find(el => el.id == id)!;
            let i = spaces.indexOf(found);
            if (spaces[i] === undefined) {
                return -1;
            }
            const alreadyOpened = spaces[i].opened_files.find(el => el.path === file.path);
            if (alreadyOpened) {
                set({
                    spaces: spaces.map(space => {
                        if (space.id !== id) {
                            return space;
                        }

                        return {
                            ...space,
                            current_file: alreadyOpened.id,
                            opened_files_stack: pushToStack(normalizeStack(space), alreadyOpened.id)
                        };
                    })
                });
                return alreadyOpened.id;
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
                    current_file: nextId,
                    opened_files_stack: pushToStack(normalizeStack(space), nextId)
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
        const spaces = get().spaces.map((space) => {
            if (space.id !== id) {
                return space;
            }
            const closedFile = space.opened_files.find(
                el => el.id === file.id
            );
            if (!closedFile) {
                return space;
            }
            const opened = space.opened_files.filter(
                el => el.id !== file.id
            );
            const openedIds = new Set(opened.map(el => el.id));
            const stack = normalizeStack(space).filter(
                stackId => stackId !== file.id && openedIds.has(stackId)
            );
            const currentFile = stack.length > 0 ? stack[stack.length - 1] : null;

            return {
                ...space,
                opened_files: opened,
                current_file: currentFile,
                opened_files_stack: stack,
            };
        });
        const res = spaces.filter(el => el.opened_files.length > 0)
        set({spaces: res});
    },
    spaces: [],
    current: 0,
    set_current: (i) => set({current: i}),
    select_current_file(id: number, id2: number | null): void {
        const spaces = get().spaces.map(el => {
            if (el.id != id) {
                return el;
            }
            if (id2 === null) {
                return {
                    ...el,
                    current_file: null,
                    opened_files_stack: normalizeStack(el)
                };
            }
            if (!el.opened_files.some(file => file.id === id2)) {
                return el;
            }
            return {
                ...el,
                current_file: id2,
                opened_files_stack: pushToStack(normalizeStack(el), id2)
            };
        })
        set({
            spaces: spaces
        })
    },
    get_space(id: number): ICodeSpace | null {
        const got = get().spaces.find(el => el.id == id);
        if (!got) {
            return null
        }
        return got!
    }


}))

