import "./styles/mini-aside.css"
import more2 from "../../../assets/more2.svg"
import {ReactElement, useState} from "react";
import AsideButton from "./AsideButton.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
import {asideStore} from "../../../stores/aside_store.ts";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";

type Props = {
    top: IAsideButton[],
    bottom?: IAsideButton[],
    max_top: number|null
    max_bot: number|null
    set_top?: (elem: (elem:ReactElement | null)=>void) => void
    set_bot?: (elem: (elem:ReactElement | null)=>void) => void
    is_left: boolean
}


export default function MiniAside(props: Props) {


    const top = props.max_top == null
        ? props.top
        : props.top.slice(0, props.max_top);

    const bottom = props.max_bot == null
        ? props.bottom
        : props.bottom?.slice(0, props.max_bot);

    let data_dir = cacheStore(state => state.data_dir)

    const [currentTop, setCurrentTop]=useState<number| null>(null)
    const [currentBot, setCurrentBot]=useState<number| null>(null)


    const left = asideStore(state=>state.left_aside)
    const right = asideStore(state=>state.right_aside)

    const set_current_top_button = props.is_left ?
        asideButtonsStore(state => state.set_current_left_button) :
        asideButtonsStore(state => state.set_current_right_button)

    const set_current_bot_button =
        asideButtonsStore(state => state.set_current_bottom_button)


    const toggle_top_visibility = props.is_left ?
        asideStore(state => state.toggle_left):
        asideStore(state => state.toggle_right)

    const toggle_bot_visibility = asideStore(state=>state.toggle_bottom);


    async function toggleTop(el:IAsideButton, val: boolean) {
        if (val) {
            set_current_top_button(null);
            setCurrentTop(null);
            toggle_top_visibility!(_ => false)
            props.set_top!(()=>null);

        } else {
            set_current_top_button(el)
            setCurrentTop(el.id)
            toggle_top_visibility!(_ => true)
            props.set_top!(el.component)
        }
    }
    async function toggleBottom(el:IAsideButton, val: boolean) {
        if (val) {
            set_current_bot_button(null);
            setCurrentBot(null)
            toggle_bot_visibility!(_ => false)
            props.set_bot!(()=>null)
        } else {
            set_current_bot_button(el)
            setCurrentBot(el.id)
            toggle_bot_visibility!(_ => true)
            props.set_bot!(el.component)
        }
    }


    return (

            <div
                style={{
                    borderLeft: right && !props.is_left? "1px solid var(--border2)":"1px solid transparent",
                    borderRight: left && props.is_left? "1px solid var(--border2)":"1px solid transparent",
                }}
                className={"project-mini-aside"}>
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