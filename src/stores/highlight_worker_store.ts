import {create} from "zustand";

//import work from "../../public/highlight.worker.ts?url"

interface Type {
    worker: Worker | null,
    init: () => void
}


export const highlightWorkerStore = create<Type>((_set, _get) => ({
    init(): void {
        /*   let worker = new Worker(work /!*, {type: "module"}*!/)
           set({
               worker: worker
           })*/
    },
    worker: null

}))