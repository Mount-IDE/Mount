import "./styles/settings-page.css"
import Button from "../../common/Button.tsx";
import {settingsStore} from "../../../stores/settings_store.ts";
import pageStore from "../../../stores/page_store.ts";


export default function SettingsPage() {


    const close_settings = settingsStore(state => state.set_show_settings)
    const close_blur = pageStore(state => state.setFilter)


    function close() {
        close_settings(false)
        close_blur(false)
    }

    return (
        <div id={"settings-page"}>
            <div id={"settings-head"}>
                <p id={"settings-label"}>Settings</p>
            </div>
            <div id={"settings-main"}></div>
            <div id={"settings-footer"}>
                <div id={"settings-buttons"}>
                    <Button title={"Close"} cb={close}/>
                    <Button title={"Ok"} cb={() => {
                    }}/>
                    <Button title={"Apply"} cb={() => {
                    }}/>
                </div>
            </div>
        </div>
    )
}