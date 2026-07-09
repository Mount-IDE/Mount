import settings from "../../assets/settings.svg"
import {menuBarStore} from "../../stores/menubar_store.ts";
import "./styles/settings-button.css"

export default function SettingsButton() {


    return (
        <button
            id={"settings-button"}
            onClick={menuBarStore.getState().global_settings}>
            <img src={settings}/>
        </button>
    )
}