import "./styles/context-menu.css"
import {RefObject, useEffect, useMemo, useRef} from "react";


export interface IContextMenuButton {
    title: string;
    cb: () => void;
    icon?: string;
    hotkeys?: string
}


type Props = _Props & {
    auto: true,
    setShow: (val: boolean) => void
    show: boolean
} | _Props & {
    auto?: false,
    setShow?: never
}


type _Props = {
    buttons: IContextMenuButton[] // array of buttons
    x?: number; // x cord of mouse click
    y?: number; // y cord of mouse click
    show?: boolean; // if true that component can be visible and hidden otherwise
    ref?: RefObject<HTMLDivElement | null> // reference to manage the component
    near?: RefObject<HTMLElement | null> //
    absolute?: boolean
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

    useEffect(() => {
        console.log(props.show)
        if (!props.auto) return

        function handler(e: MouseEvent) {
            const tg = ref.current;

            if (!tg) return;

            console.log("RECT!", tg.getBoundingClientRect())
            const elem = e.target as Element
            if (!(tg.contains(elem) || tg == elem)) {
                //setShowContext(false)
                // props.setShow?.(false)
            }

        }

        window.addEventListener("click", handler);
        return () => {
            window.removeEventListener("click", handler);
        };

    }, [props.auto, props.show]);


    const [x, y] = useMemo<[number, number]>(() => {
        const rect = props.near?.current?.getBoundingClientRect()
        console.log("RECT", rect)
        let sx = (rect?.left ?? 0)/* - (rect?.width??0)*/
        let sy = (rect?.top ?? 0) /*- (rect?.height??0)*/
        return [(props.x ?? 0) + sx, (props.y ?? 0) + sy]

    }, [props.near, props.near?.current, props.x, props.y])

    useEffect(() => {
        console.log("XY", x, y)
        console.log("REF", ref.current?.style.left, ref.current?.style.top)
    }, [x, y, props.auto, props.show, props.near?.current])

    /*
        useEffect(() => {
            const rect = props.near?.current?.getBoundingClientRect()
            let x = (rect?.left??0) + (props.x??0)
            let y = (rect?.top??0) + (props.y??0)
            if (props.ref?.current) {
                props.ref.current.style.left = `${x}px`
                props.ref.current.style.top=`${y}px`
            }else if (ref.current) {
                ref.current.
            }
        }, [props.x,props.y, props.auto, props.show, props.near?.current, props.]);*/

    return (
        <>
            {
                props.show &&
                <div ref={props.auto ? ref : props.ref}
                     style={{
                         position: props.absolute ? "absolute" : "fixed",
                         left: `${x}px`,
                         top: `${y}px`,
                     }}
                     className={"context-menu"}>
                    {
                        props.buttons.map((el, i) =>
                            <ContextMenuButton obj={el} key={i}/>
                        )
                    }
                </div>
            }
        </>

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
export function ContextMenuButton(props: ButtonProps) {
    const {obj} = props;
    return (
        <div

            className={"context-menu-button select-none"}
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