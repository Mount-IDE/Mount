import "./styles/title-bar.css"
import wrap from "../../assets/title-wrap.svg"
import resize from "../../assets/title-resize.svg"
import close from "../../assets/title-close.svg"
import {getCurrentWindow} from "@tauri-apps/api/window"
import pageStore, {Window} from "../../stores/page_store.ts";
import MenuBar from "./MenuBar.tsx";
import Launch from "./Launch.tsx";
import SettingsButton from "./SettingsButton.tsx";
import {computeBP, themeStore} from "../../stores/theme_store.ts";
import {useMemo} from "react";

export default function TitleBar() {

    const appWindow = getCurrentWindow();
    const buttons = [
        {
            icon: wrap,
            cb: async () => {
                await appWindow.minimize();
            }
        }, {
            icon: resize,
            cb: async () => {
                const res = await appWindow.isMaximized();
                if (res) {
                    await appWindow.unmaximize()
                } else {
                    await appWindow.maximize();
                }
            }
        }, {
            icon: close,
            cb: async () => {
                await appWindow.close()
            }
        },

    ]

    const current_page = pageStore(state => state.current)

    let page = pageStore(state => state.current)

    const theme = themeStore(state => state.get_titlebar(page));
    //console.log(theme)

    const buttonThemes = useMemo(() => {


        return {
            background: theme?.button?.background,
            ...computeBP(theme?.button?.border, "border")
        }
    }, [theme])

    return (
        <div id={"title-bar"}
             style={{
                 background: theme?.this?.background,
                 ...computeBP(theme?.this?.border, "border")
             }}>
            {
                current_page == Window.Project &&
                <>
                    <MenuBar/>
                    <Launch/>
                </>
            }
            <div id={"title-bar-buttons"}>
                <SettingsButton/>
                {buttons.map(el => {
                    return <TitleBarButton styles={buttonThemes} key={el.icon} {...el}/>
                })}
            </div>
        </div>
    )
}


type Props = {
    icon: string;
    cb: () => void;
    styles: any
}

function TitleBarButton(props: Props) {
    return (
        <button {...props.styles} className={"title-bar-button"} onClick={props.cb}>
            <img src={props.icon}/>
        </button>
    )
}