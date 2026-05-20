import {create} from "zustand";

interface Type{
    spaces: ICodeSpace[]
    current: number;
    set_current: (i:number)=>void
    add_code_space: ()=>void
    remove_code_space: (id: number)=>void
    add_file_to_code_space: (id:number, file: OpenedFile)=>void
    remove_file_from_code_space: (id:number, file: OpenedFile)=>void
}

export const codeSpaceStore = create<Type>((set, get)=>({
    add_code_space(): void {
        let spaces = get().spaces
        if (spaces.length==0){
            set({
                spaces: [{
                    id:0,
                    opened_files: []
                }]
            })
        }else{
            let id = spaces[spaces.length-1].id+1;
            set({
                spaces: [...spaces, {id, opened_files:[]}]
            })
        }

    },
    add_file_to_code_space(id: number, file: OpenedFile): void {
        let spaces = get().spaces;
        if (id==-1) {
            set({
                spaces: [{
                    id: 0, opened_files: [{...file, id: 0}]
                }],
                current: 0
            })
        }
        let id_ = spaces.map(el=>el.id)

        if (!id_.includes(id)) {
            set({
                spaces: [...spaces, {
                    id, opened_files: [{...file, id:0}]
                }]
            })
        }else {
            let found = spaces.find(el=>el.id)!;
            let i = spaces.indexOf(found);
            if (spaces[i]===undefined){
                return;
            }
            if (spaces[i].opened_files.map(el=>el.path).includes(file.path)){
                return
            }
            let last = spaces[i].opened_files;
            let last_ = last[last.length-1];
            spaces[i].opened_files.push({...file, id: last_.id+1});
            set ({
                spaces: [...spaces]
            })
        }
    },
    remove_code_space(id: number): void {},
    remove_file_from_code_space(id: number, file: OpenedFile): void {},
    spaces: [],
    current: 0,
    set_current: (i)=> set({current: i})

}))

