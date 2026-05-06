import {create} from "zustand";

interface Type{
    spaces: ICodeSpace[]

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
        let id_ = spaces.map(el=>el.id)
        if (!id_.includes(id)) {
            set({
                spaces: [...spaces, {
                    id, opened_files: [file]
                }]
            })
        }else {
            let found = spaces.find(el=>el.id)!;
            let i = spaces.indexOf(found);
            spaces[i].opened_files.push(file);
            set ({
                spaces: [...spaces]
            })
        }
    },
    remove_code_space(id: number): void {},
    remove_file_from_code_space(id: number, file: OpenedFile): void {},
    spaces: []

}))

