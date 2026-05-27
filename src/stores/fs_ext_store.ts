import {create} from "zustand"

interface Type {
    config: FsConfigIcons[] | null,
    set_icons: (icons: FsConfigIcons[]) => void;
    get_dir_by_type: (typ?: string) => [boolean, string, string];
    get_file_by_ext: (ext: string) => [boolean, string, string];
    get_file_by_name: (name: string) => [boolean, string, string];
}


export const fsExtStore = create<Type>((set, get) => ({
    config: null,
    set_icons: (icons) => {
        set({
            config: icons
        })
    },
    get_dir_by_type(typ: string | undefined): [boolean, string, string] {
        const config = get().config;
        if (config == null) {
            return [false, "", ""]
        }
        if (!typ) {
            const icons = config[0].icons.filter(el => el.typ == "directory");
            if (icons.length == 0) {
                return [false, "", config[0].theme]
            }
            return [true, icons[0].icon, config[0].theme]
        }
        const icons = config[0].icons.filter(el => el.typ == "directory" && el.ext.includes(typ))
        if (icons.length == 0) {
            return [false, "", config[0].theme]
        }
        return [true, icons[0].icon, config[0].theme]
    },
    get_file_by_ext(ext: string): [boolean, string, string] {
        const config = get().config;
        if (config == null) {
            return [false, "", ""]
        }
        const icons = config[0].icons.filter(el => el.typ == "file" && el.ext.includes(ext));
        if (icons.length == 0) {
            return [false, "", config[0].theme]
        }
        return [true, icons[0].icon, config[0].theme]
    },
    get_file_by_name(name: string): [boolean, string, string] {
        const config = get().config;
        if (config === null) {
            return [false, "", ""]
        }

        const cfg = config[0];
        for (let j in cfg.icons) {
            let icons = cfg.icons[j];
            for (let ext of icons.ext){
                if (name.endsWith(ext)){
                    return [true, icons.icon, "_"]
                }
            }
        }
        return [false, "", ""];
    }


}))