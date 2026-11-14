import "./styles/central.css"
import {codeSpaceStore} from "@stores/code_space_store.ts";
import CodeSpace from "./CodeSpace.tsx";
import logo from "@assets/logo_black_white.svg"
import {computeBP, themeStore} from "@stores/theme_store.ts";
import Icon from "@common/Icon.tsx";
import {Fragment} from "react";


export default function Central() {

    const code_spaces = codeSpaceStore(state => state.spaces)

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.center);

    const current = theme?.this;


    return (
        <div id={"project-central"}
             style={{
                 background: theme?.this?.background,
                 borderRadius: current?.rounded,
                 ...computeBP(current?.border, "border"),
                 ...computeBP(current?.padding, "padding"),
                 minWidth: "10%"
             }}>
            {code_spaces.map((el, i) =>
                <CodeSpace key={el.id} obj={el} amount={i}/>
            )}
            {
                code_spaces.length === 0 &&
                <Logo/>
            }
        </div>
    )
}


type CommandProps = {
    text: string
    hotkeys: string[]
}

function Command(props: CommandProps) {


    return (
        <div className={"text-(--subtitle) font-[250] flex gap-[5px] justify-between w-1/4 text-[9pt] h-[20pt]"}>
            {props.text}
            <div className={"flex gap-[10px] "}>
                {
                    props.hotkeys.map((el, i) =>
                        <Fragment key={i}>
                            {i > 0 &&
                                <span>+</span>
                            }
                            <div
                                style={{
                                    // padding: "5px"
                                }}
                                className={"w-[33px] flex flex-col items-center justify-center bg-(--border3) rounded-[5px] text-(--subtitle) text-[6pt]"}
                            >
                                {el}
                            </div>
                        </Fragment>
                    )
                }
            </div>
        </div>
    )
}

function Logo() {
    return (
        <div
            style={{
                userSelect: "none",
                width: "100%",
                height: "100%",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center"
            }}
        >
            <Icon src={logo} size={"300px"} in_size={"200px"}/>
            <div className={"w-full flex flex-col items-center gap-4"}
                 style={{
                     marginTop: "35px"
                 }}
            >
                <Command text={"Show Command Pallete"} hotkeys={["Shift", "Shift"]}/>
                <Command text={"Open Global Settings"} hotkeys={["Ctrl", "Shift", "P"]}/>
                <Command text={"Open Project Settings"} hotkeys={["Ctrl", "Shift", "O"]}/>
            </div>
        </div>
    )
}