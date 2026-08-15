import "./styles/common-parameters.css"


import {open} from "@tauri-apps/plugin-dialog";
import dir from "../../assets/dir.svg";


type Props = {
    typ: "file" | "dir"
    title: string
    value: string
    write: (val: string) => void
    show?: boolean
    incorrect?: string
    required?: boolean
    placeholder: string
}

export default function FSContext(props: Props) {
    const typ = props.typ;
    const label = props.title;

    async function openDialog() {
        if (typ == "file") {
            const res = await open({
                directory: false,
                title: "Choose the file"
            })
            if (res !== null) {
                props.write(res)
            }
        } else {
            const res = await open({
                directory: true,
                title: "Choose the directory"
            })
            if (res !== null) {
                props.write(res)
            }
        }
    }


    /*

        let classes = props.def ? "project-parameter-value project-parameter-file" :
            "project-parameter-value project-parameter-file project-parameter-value-disabled"

    */

    return (
        <div
            className={"fs-context"}
            style={
                props.show == false ? {
                    opacity: 0.5,
                    pointerEvents: "none"
                } : {}
            }
        >
            <p className={"p"}>{label}</p>
            <div className={"in"}>
                <input placeholder={props.placeholder}
                       value={props.value}
                       onInput={(e) => props.write(e.currentTarget.value)}/>
                <button className={"bt"} onClick={openDialog}>
                    <img src={dir}/>
                </button>
            </div>

        </div>
    )
}