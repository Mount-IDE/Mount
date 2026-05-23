import "./styles/context-menu.css"
import {RefObject} from "react";


export interface IContextMenuButton {
    title: string;
    cb: () => void;
    icon?: string;
    hotkeys?: string
}


type Props = {
    buttons: IContextMenuButton[]
    x: number;
    y: number;
    show: boolean;
    ref?:RefObject<HTMLDivElement|null>
}

export default function ContextMenu(props: Props) {

    return (
        <div ref={props.ref}
            style={{
                opacity: props.show? 1:0,
                pointerEvents: props.show?"all": "none",
                left: `${props.x}px`,
                top: `${props.y}px`,
            }}
            className={"context-menu"}>
            {
                props.buttons.map((el,i)=>
                    <ContextMenuButton obj={el} key={i}/>
                )
            }
        </div>
    )
}

type ButtonProps = {
    obj: IContextMenuButton
}

function ContextMenuButton(props: ButtonProps) {
    const {obj} = props;
    return (
        <div

            className={"context-menu-button"}
             onClick={obj.cb}
        >
            {obj.icon !== undefined &&
                <div className={"context-menu-button-img"}>
                    <img src={`/builtin/context-icons/${obj.icon}`}/>
                </div>
            }
            <p className={"context-menu-button-p"}>{obj.title}</p>
            {
                obj.hotkeys !==undefined &&
                <p className={"context-menu-button-hotkey"}>{obj.hotkeys}</p>
            }
        </div>
    )
}