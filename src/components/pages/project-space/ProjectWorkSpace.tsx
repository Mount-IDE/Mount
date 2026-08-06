import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";
import React from "react";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {asideStore} from "../../../stores/aside_store.ts";
import Bottom from "./Bottom.tsx";
import {themeStore} from "../../../stores/theme_store.ts";


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

    const theme = themeStore(state => state.current_theme?.elements?.project_space)



    return (
        <div id={"project-workspace"}>
            <MiniAside state={left} top_button={set_current_left_button} bot_button={set_current_bot_button}
                       is_left={true}
                       set_top={set_current_left} set_bot={set_current_bot}
                       top={left_top} bottom={left_bot} max_top={3} max_bot={null}/>
            <hr className={"project-hr"}
                style={{
                    border: theme?.mini_aside?.left?.hr?.border ?? "1px solid var(--border3)"
                }}
            />
            <div id={"project-center"}>
                <div id={"project-top"}>
                    <Aside state={left} left={true}/>
                    <Central/>
                    <Aside state={right} left={false}/>
                </div>
                <Bottom />
            </div>
            <hr
                style={{
                    border: theme?.mini_aside?.right?.hr?.border ?? "1px solid var(--border3)"
                }}
                className={"project-hr"}/>
            <MiniAside state={right} top_button={set_current_right_button} is_left={false} set_top={set_current_right}
                       set_bot={set_current_bot} top={right_top}
                       max_top={3} max_bot={null}
            />
        </div>
    )
}