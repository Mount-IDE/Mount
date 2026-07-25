import "./styles/notifications.css"
import {noteStore} from "../../stores/note_store.ts";
import image from "../../assets/title-close.svg"

export default function Notifications() {


    const bg = {
        0: `rgb(53, 142, 239)`,
        1: `rgb(243, 205, 53)`,
        2: `rgb(253, 99, 99)`,
        3: `rgb(0, 0, 0)`,
    }


    const notifications = noteStore(state => state.notification_bus)

    const notes = notifications.length > 3 ? notifications.slice(3) : notifications;

    const rem = noteStore(state => state.rem_note)
    return (
        <div id={"notifications"}>
            {
                notes.map((el, i) =>
                    <div
                        key={i}
                        style={{
                            background: `rgb(from ${bg[el.type]} r g b / 0.5)`,
                            border: `1px solid ${bg[el.type]}`
                        }}
                        className={"notification"}>
                        <div className={"notification-header"}>
                            <button
                                onClick={() => rem(el.id)}
                            >
                                <img src={image}/>
                            </button>
                        </div>
                        <p>{el.text}</p>
                    </div>
                )
            }
        </div>
    )
}