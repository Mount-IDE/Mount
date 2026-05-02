import "./styles/aside-button.css"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";


type Props={
    bt: IAsideButton
    data_dir: string,
    selected: boolean
}

export default function AsideButton(props: Props){

    let [dir, setDir]= useState("")
    useEffect(() => {
        async function load(){
            try {
                let res = await invoke<string>("make_path_from_icon", {
                    components: props.bt.icon,
                    path: "aside_icons",
                    code: true
                })
                console.log(res)
                setDir(res)
            } catch (e){
                console.error(e)
            }
        }
        load();
    }, [props.data_dir, props.bt.icon]);
    console.log(dir)
    return (
        <button className={ props.selected?"project-mini-aside-button selected":"project-mini-aside-button"}>
            <img alt={props.bt.alt} src={dir}/>
        </button>
    )
}