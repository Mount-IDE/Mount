import "./styles/aside-button.css"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import arrow from "../../../assets/arrow.svg"

type Props = {
    bt: IAsideButton
    data_dir: string,
    selected: boolean
    cb: (el: IAsideButton, val: boolean) => void
}

export default function AsideButton(props: Props) {

    let [dir, setDir] = useState(arrow)
    useEffect(() => {
        async function load() {
            try {
                let res = await invoke<string>("make_path_from_icon", {
                    components: props.bt.icon,
                    path: "aside_icons",
                    code: true
                })
                setDir(res)
            } catch (e) {
                console.error(e)
            }
        }

        load().then();
    }, [props.data_dir, props.bt.icon]);

    return (
        <button onClick={() => props.cb(props.bt, props.selected)}
                className={props.selected ? "project-mini-aside-button selected" : "project-mini-aside-button"}>
            <img alt={props.bt.alt} src={dir}/>
        </button>
    )
}