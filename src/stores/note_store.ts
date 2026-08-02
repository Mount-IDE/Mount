import {create} from "zustand";

export const enum NotificationType {
    NOTE,
    WARN,
    ERR,
    FATAL
}

let id = 0;

interface Notification {
    text: string;
    type: NotificationType;

}

interface Note extends Notification {
    id: number
}

interface Type {
    notification_bus: Note[]
    add_note: (note: Notification, time?: number) => void;
    clear: () => void;
    rem_note: (i: number) => void;
}


export const noteStore = create<Type>((set, get) => ({
    notification_bus: [],
    add_note: (note: Notification, time?: number) => {
        let notes = [...get().notification_bus]
        let res = {id: id, ...note}
        notes.push(res);
        if (time) {
            let id_ = id;
            setTimeout(() => {
                let notes = get().notification_bus;
                notes = notes.filter(el => el.id != id_)
                set({notification_bus: notes})
            }, time)
        }
        id++;
        set({
            notification_bus: notes
        })
    },
    clear: () => {
        set({notification_bus: []})
    },
    rem_note: (i) => {
        let res = get().notification_bus
        res = res.filter(el => el.id != i)
        set({notification_bus: res})
    }
}))