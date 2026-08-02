import "./styles/mini-aside.css"
import more2 from "../../../assets/more2.svg"
import React, {useEffect, useState} from "react";
import AsideButton from "./AsideButton.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
// import aside from "./Aside.tsx";
import {asideStore, comp} from "../../../stores/aside_store.ts";


type Props = {
    top: IAsideButton[],
    bottom?: IAsideButton[],
    max_top: number | null
    max_bot: number | null
    set_top?: (elem: comp) => void
    set_bot?: (elem: comp) => void
    is_left: boolean
    top_button: (el: IAsideButton | null) => void
    bot_button?: (el: IAsideButton | null) => void
    state: boolean
}


function MiniAside(props: Props) {


    const top = props.max_top == null
        ? props.top
        : props.top.slice(0, props.max_top);

    const bottom = props.max_bot == null
        ? props.bottom
        : props.bottom?.slice(0, props.max_bot);

    let data_dir = cacheStore(state => state.data_dir)

    const [currentTop, setCurrentTop] = useState<number | null>(null)
    const [currentBot, setCurrentBot] = useState<number | null>(null)


    useEffect(() => {
        if (!props.state) {
            setCurrentTop(null)
            setCurrentBot(null)
        }
    }, [props.state])


    const toggle_top_visibility = props.is_left ?
        asideStore(state => state.toggle_left) :
        asideStore(state => state.toggle_right)

    const toggle_bot_visibility = asideStore(state => state.toggle_bottom);


    async function toggleTop(el: IAsideButton, val: boolean) {
        if (val) {
            props.top_button(null);
            setCurrentTop(null);
            toggle_top_visibility!(_ => false)
            props.set_top!(() => null);

        } else {
            props.top_button(el)
            setCurrentTop(el.id)
            toggle_top_visibility!(_ => true)
            props.set_top!(el.component)
        }
    }

    async function toggleBottom(el: IAsideButton, val: boolean) {
        if (val) {
            if (el.component_type === "Light") {
                props.bot_button!(null);
                props.set_bot!(() => null)
            }
            setCurrentBot(null)
            toggle_bot_visibility!(_ => false)
        } else {
            props.bot_button!(el)
            setCurrentBot(el.id)
            toggle_bot_visibility!(_ => true)
            if (el.component_type === "Light") props.set_bot!(el.component)
        }
    }


    return (
        <div className={"project-mini-aside"}>
            <div className={"project-mini-aside-top"}>
                <div className={"project-mini-aside-buttons"}>
                    {top.map((el) =>
                        <AsideButton cb={toggleTop} selected={el.id == currentTop} bt={el} key={el.id}
                                     data_dir={data_dir}/>
                    )}
                </div>
                {
                    props.max_top !== null &&
                    <button className={"project-mini-aside-other-buttons"}>
                        <img src={more2}/>
                    </button>
                }
            </div>
            {
                props.bottom &&
                <div className={"project-mini-aside-bottom"}>
                    {
                        props.max_bot !== null &&
                        <button className={"project-mini-aside-other-buttons"}>
                            <img src={more2}/>
                        </button>}
                    <div className={"project-mini-aside-buttons"}>
                        {bottom!.map((el) =>
                            <AsideButton cb={toggleBottom} selected={el.id == currentBot} bt={el} key={el.id}
                                         data_dir={data_dir}/>
                        )}
                    </div>
                </div>
            }
        </div>


    )
}

export default React.memo(MiniAside)
