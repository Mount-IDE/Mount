import "./styles/title-bar.css"
import wrap from "../../assets/title-wrap.svg"
import resize from "../../assets/title-resize.svg"
import close from "../../assets/title-close.svg"
import {getCurrentWindow} from "@tauri-apps/api/window"
export default function TitleBar() {

    const appWindow = getCurrentWindow();
    const buttons = [
        {
            icon: wrap,
            cb: async ()=>{
                appWindow.minimize();
            }
        },{
            icon: resize,
            cb: async ()=>{
                const res=await appWindow.isMaximized();
                if (res) {
                    appWindow.unmaximize()
                }else{
                    appWindow.maximize();
                }
            }
        },{
            icon: close,
            cb: async()=>{
                appWindow.close()
            }
        },

    ]

    return (
        <div id={"title-bar"}>
            <div id={"title-bar-buttons"}>
                {buttons.map(el=>{
                    return <TitleBarButton key={el.icon} {...el}/>
                })}
            </div>
        </div>
    )
}


type Props = {
    icon: string;
    cb: ()=>void;
}

function TitleBarButton(props: Props) {
    return (
        <button className={"title-bar-button"} onClick={props.cb}>
            <img src={props.icon}/>
        </button>
    )
}