import "./styles/header.css"
import minus from "../../assets/title-wrap.svg"
import {asideStore} from "../../stores/aside_store.ts";
import {useEffect, useRef} from "react";

type Props={
    label: string
    is_left: boolean;
}



export default function Header(props: Props){

    const minus_ref = useRef<HTMLButtonElement>(null)

    const main_ref = useRef<HTMLDivElement>(null)

    useEffect(() => {
        const main = main_ref.current;
        const minus_ = minus_ref.current;
        if (!main || !minus_) return

        const obs = new ResizeObserver(e => {
            for (let entry of e) {
                if (entry.target == main) {
                    if (entry.contentRect.width > 130) {
                        minus_.style.display = "flex"
                    } else {
                        minus_.style.display = "none"
                    }
                }
            }
        });
        obs.observe(main!);

        return () => {
            obs.disconnect()
        }

    }, [props.is_left, props.label]);


    function click() {
        props.is_left ? asideStore.getState().toggle_left(_ => false) : asideStore.getState().toggle_right(_ => false)
    }

    return (
        <div ref={main_ref} className={"aside--header"}>
            <p className={"aside-header-p"}>{props.label}</p>
            <button ref={minus_ref} onClick={click} className={"aside-header-bt"}>
                <img src={minus}/>
            </button>
        </div>
    )
}