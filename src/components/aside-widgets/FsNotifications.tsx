import "./styles/notifications.css"
import {noteStore} from "../../stores/note_store.ts"

export default function FsNotifications() {

    const notes = noteStore(state => state.notification_bus)

    return (
        <>
            <div className={"aside-in"}>
                {
                    notes.map((el, i) => (
                        <div>{el.text}</div>
                    ))
                }
            </div>
        </>
    )
}