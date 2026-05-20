import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";
import React, {useEffect} from "react";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {asideStore} from "../../../stores/aside_store.ts";
import FsAside from "../../aside-widgets/FsAside.tsx";


export default function ProjectWorkSpace() {

    const left_top = asideButtonsStore(state => state.left_buttons);
    const left_bot = asideButtonsStore(state => state.bottom_buttons);
    const right_top = asideButtonsStore(state => state.right_buttons);




    const set_current_left = asideStore(state => state.set_current_left);
    const set_current_right = asideStore(state => state.set_current_right);
    const set_current_bot = asideStore(state => state.set_current_bottom);


    let left =
        asideStore(state => state.left_aside);
    let right = asideStore(state => state.right_aside);

    const set_current_left_button = asideButtonsStore(state => state.set_current_left_button);
    const set_current_right_button = asideButtonsStore(state => state.set_current_right_button)

    const set_current_bot_button =
        asideButtonsStore(state => state.set_current_bottom_button)


    return (
        <div id={"project-workspace"}>
            <MiniAside top_button={set_current_left_button} bot_button={set_current_bot_button} is_left={true}
                       set_top={set_current_left} set_bot={set_current_bot}
                       top={left_top} bottom={left_bot} max_top={3} max_bot={null}/>
            <hr className={"project-hr"}/>
            <Aside state={left} left={true}/>
            <Central/>
            <Aside state={right} left={false}/>
            <hr className={"project-hr"}/>
            <MiniAside top_button={set_current_right_button} is_left={false} set_top={set_current_right}
                       set_bot={set_current_bot} top={right_top}
                       max_top={3} max_bot={null}/>
        </div>
    )
}