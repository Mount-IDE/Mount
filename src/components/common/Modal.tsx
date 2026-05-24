import "./styles/modal.css"
import {useEffect, useState} from "react";
import {menuStore} from "../../stores/menu_store.ts";


type ModalType = "alert" |"confirm" | "prompt"

export interface ModalButton {
    typ: "cancel" | "input"
    title: string,
    cb: (val?: string) => void;
}

export type ModalProps = {
    title: string
    typ: ModalType,
    val?: string
    buttons: ModalButton[]
}


export default function Modal(props: ModalProps) {

    const [input, setInput] = useState(props.val!==undefined?props.val:"")

    useEffect(() => {
        setInput(props.val!==undefined?props.val:"")
    }, [props.val]);
    function cb(bt: ModalButton) {
        bt.cb(bt.typ == "input" ? input : undefined)
    }

    const close = menuStore(state=>state.close_modal);

    return (
        <div className={"modal"}>
            <p className={"modal-p"}>{props.title}</p>
            <div className={"modal-content"}>
                {props.typ === 'prompt' &&
                    <input value={input} onInput={(e) => setInput(e.currentTarget.value)}/>
                }
            </div>
            <div className={"modal-buttons"}>
                {
                    props.typ=="prompt" &&
                    <ModalButton obj={{
                        cb: (_)=> close(),
                        title: "Cancel",
                        typ: "cancel"

                    }} _callback={(obj)=>{obj.cb()}}/>
                }
                {
                    props.typ=="confirm" &&
                    <ModalButton obj={{
                        cb:(_)=> close(),
                        title: "Cancel",
                        typ: "cancel"
                    }} _callback={(obj)=>{obj.cb()}}/>
                }
                {props.buttons.map((el, i) =>
                    <ModalButton obj={el} _callback={cb} key={i}/>
                )}
            </div>
        </div>
    )
}

type BtProps = {
    _callback: (bt: ModalButton) => void
    obj: ModalButton
}

function ModalButton(props: BtProps) {

    return (
        <div onClick={() => props._callback(props.obj)} className={"modal-button"}>
            {props.obj.title}
        </div>
    )
}