import {create}from "zustand"
import {invoke} from "@tauri-apps/api/core"
interface Type{
    config: FsConfigIcons[]|null,
    load: ()=>Promise<void>;
    get_dir_by_type:(typ?: string)=>[boolean, string, string];
    get_file_by_ext:(ext:string)=>[boolean, string, string];
}


export const fsExtStore=create<Type>((set,get)=>({
    config: null,
    load: async ()=>{
        try{
            const conf = await invoke<FsConfigIcons[]>("get_fs_ext_icons");
            set({config: conf});
            console.log(conf)
        } catch(e) {
            console.error(e)
        }
    },
    get_dir_by_type(typ: string | undefined): [boolean, string, string] {
        const config = get().config;
        if (config==null){
            return [false, "", ""]
        }
        if (!typ){
            const icons = config[0].icons.filter(el=>el.typ=="directory");
            if (icons.length==0){
                return [false, "", ""]
            }
            return [true,icons[0].icon, config[0].theme]
        }
        const icons = config[0].icons.filter(el=>el.typ=="directory" && el.ext.includes(typ))
        if (icons.length==0){
            return [false, "", ""]
        }
        return [true,icons[0].icon, config[0].theme]
    },
    get_file_by_ext(ext: string): [boolean, string, string] {
        const config = get().config;
        if (config==null) {
            return [false, "", ""]
        }
        const icons = config[0].icons.filter(el=>el.typ=="file"&& el.ext.includes(ext));
        if (icons.length==0){
            return [false, "", ""]
        }
        return [true,icons[0].icon, config[0].theme]
    }

}))