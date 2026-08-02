import "./styles/notifications.css"
import {noteStore, NotificationType} from "../../stores/note_store.ts";
import image from "../../assets/title-close.svg"
import {AnimatePresence, motion} from "motion/react";
import {useEffect} from "react";
import {listen} from "@tauri-apps/api/event";

export default function Notifications() {


    const bg = {
        0: `rgb(53, 142, 239)`,
        1: `rgb(243, 205, 53)`,
        2: `rgb(253, 99, 99)`,
        3: `rgb(0, 0, 0)`,
    }


    const notifications = noteStore(state => state.notification_bus)

    const notes = notifications.length > 3 ? notifications.slice(notifications.length - 3) : notifications;

    const rem = noteStore(state => state.rem_note)

    useEffect(() => {

        let note = listen<string>("NOTE", (d) => {
            let parsed = JSON.parse(d.payload).data;
            noteStore.getState().add_note({
                text: parsed,
                type: NotificationType.NOTE
            })
        })
        let warn = listen<string>("WARN", (d) => {
            let parsed = JSON.parse(d.payload).data;
            noteStore.getState().add_note({
                text: parsed,
                type: NotificationType.WARN
            })
        })
        let err = listen<string>("ERROR", (d) => {
            let parsed = JSON.parse(d.payload).data;
            noteStore.getState().add_note({
                text: parsed,
                type: NotificationType.ERR
            })
        })
        let fatal = listen<string>("FATAL", (d) => {
            let parsed = JSON.parse(d.payload).data;
            noteStore.getState().add_note({
                text: parsed,
                type: NotificationType.FATAL
            })
        })


        return () => {
            note.then(fn => fn())
            warn.then(fn => fn())
            err.then(fn => fn())
            fatal.then(fn => fn())
        }


    }, []);


    return (
        <div id={"notifications"}>
            <AnimatePresence>
                {
                    notes.map((el, i) =>
                        <motion.div
                            key={el.id}
                            layout
                            initial={{
                                opacity: 0,
                                y: 50,
                            }}
                            animate={{
                                opacity: 1,
                                y: 0
                            }}
                            exit={{
                                opacity: 0,
                                y: 50
                            }}
                            transition={{
                                duration: 0.25,

                            }}
                            className={"notification"}
                        >
                            <div
                                key={i}
                                style={{
                                    background: `rgb(from ${bg[el.type]} r g b / 0.65)`,
                                    border: `1px solid ${bg[el.type]}`
                                }}
                                className={"notification child"}>
                                <div className={"notification-header"}>
                                    <button
                                        onClick={() => rem(el.id)}
                                    >
                                        <img src={image}/>
                                    </button>
                                </div>
                                <p>{el.text}</p>
                            </div>
                        </motion.div>
                    )
                }
            </AnimatePresence>

        </div>
    )
}