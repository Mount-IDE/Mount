import "./styles/context-menu.css"
import {RefObject, useEffect, useRef, useState} from "react";


export interface IContextMenuButton {
    title: string;
    cb: () => void;
    icon?: string;
    hotkeys?: string
}


type Props = {
    buttons: IContextMenuButton[] // array of buttons
    x: number; // x cord of mouse click
    y: number; // y cord of mouse click
    show?: boolean; // if true that component can be visible and hidden otherwise
    ref?: RefObject<HTMLDivElement | null> // reference to manage the component
    auto?: boolean // if true - component close automatically
}


/**
 * Component of Context menu
 *
 * receives an array of buttons and render it
 * @param props
 * @constructor
 */
export default function ContextMenu(props: Props) {


    const ref = useRef<HTMLDivElement>(null)

    const [context, setShowContext] = useState(props.show == undefined ? false : props.show);


    useEffect(() => {

        if (!props.auto) return

        function handler(e: MouseEvent) {
            const tg = ref.current;


            if (!tg) return;
            const elem = e.target as Element
            if (!(tg.contains(elem) || tg == elem)) {
                setShowContext(false)

            }

        }

        window.addEventListener("click", handler);
        return () => {
            window.removeEventListener("click", handler);
        };

    }, [props.auto]);

    useEffect(() => {
        setShowContext(props.show != undefined ? props.show : false)
    }, [props.show]);


    return (
        <div ref={props.auto ? ref : props.ref}
             style={{
                 opacity: context ? 1 : 0,
                 pointerEvents: context ? "all" : "none",
                 left: `${props.x}px`,
                 top: `${props.y}px`,
             }}
             className={"context-menu"}>
            {
                props.buttons.map((el, i) =>
                    <ContextMenuButton obj={el} key={i}/>
                )
            }
        </div>
    )
}

type ButtonProps = {
    obj: IContextMenuButton // object of button
}


/**
 * Button of context menu
 * @param props
 * @constructor
 */
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
                obj.hotkeys !== undefined &&
                <p className={"context-menu-button-hotkey"}>{obj.hotkeys}</p>
            }
        </div>
    )
}