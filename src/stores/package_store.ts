import {create} from "zustand";


interface Type {
    packages: IPackage[]


    set_package: (pack: IPackage[]) => void
}


export const packageStore = create<Type>((set, get) => ({
    set_package(pack: IPackage[]): void {
        set({packages: pack})
    },
    packages: []

}))