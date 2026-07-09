import "./styles/title-bar.css"
import wrap from "../../assets/title-wrap.svg"
import resize from "../../assets/title-resize.svg"
import close from "../../assets/title-close.svg"
import {getCurrentWindow} from "@tauri-apps/api/window"
import pageStore, {Window} from "../../stores/page_store.ts";
import MenuBar from "./MenuBar.tsx";
import Launch from "./Launch.tsx";
import SettingsButton from "./SettingsButton.tsx";

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

    return (
        <div id={"title-bar"}>
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
                    return <TitleBarButton key={el.icon} {...el}/>
                })}
            </div>
        </div>
    )
}


type Props = {
    icon: string;
    cb: () => void;
}

function TitleBarButton(props: Props) {
    return (
        <button className={"title-bar-button"} onClick={props.cb}>
            <img src={props.icon}/>
        </button>
    )
}