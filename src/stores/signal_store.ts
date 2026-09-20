import {create} from "zustand";


type SignalPath = Record<string, Record<string, number[]>>; // aside id -> signal name -> random salt

interface Type {
    signals: SignalPath

    append: (aside: string, signal: string) => void;

    remove_first: (aside: string, signal: string) => void;

    remove_by_aside: (prefix: string) => void;

    remove_by_signal_name: (aside: string, signal: string) => void;


    count: (aside: string, signal: string) => number;

    append_save: (aside: string, signal: string) => void

}


export const signalStore = create<Type>((set, get) => ({


    append_save: (aside, signal) => {
        let signals = get().signals;
        let found = signals[aside]?.[signal];
        if (!found) {
            set({
                ...signals,
                [aside]: {
                    ...signals[aside],
                    [signal]: [0]
                }
            })
        }
    },

    signals: {},

    append: (aside, signal) => {
        let signals = get().signals;
        let found = signals[aside]?.[signal] as number[] | undefined
        if (!found) {
            set({
                signals: {
                    ...signals,
                    [aside]: {
                        ...signals[aside],
                        [signal]: [0]
                    }
                }
            })
        } else {
            if (found.length > 32) {
                return
            }
            if (found.length == 0) {
                found.push(0)
            } else {
                let last = found[found.length - 1];
                found.push(last + 1);
            }
            set({
                signals: {
                    ...signals,
                    [aside]: {
                        ...signals[aside],
                        [signal]: [...found]
                    }
                }
            })
        }
    },
    remove_first: (aside, signal) => {
        let signals = get().signals;
        let found = signals[aside]?.[signal] as number[] | undefined
        if (found) {
            found.splice(0, 1);
            set({
                signals: {
                    [aside]: {
                        ...signals[aside],
                        [signal]: [...found]
                    }
                }
            })
        }
    },
    remove_by_aside: (prefix) => {
        let signals = get().signals
        delete signals[prefix]
        set({
            signals: {...signals}
        })
    },
    remove_by_signal_name: (aside, signal) => {
        let signals = get().signals;
        let found = signals[aside]
        delete found[signal]
        set({
            signals: {
                ...signals,
                [aside]: {
                    ...found
                }
            }
        })
    },
    count: (aside, signal): number => {
        let signals = get().signals[aside]?.[signal]?.length ?? 0
        return signals
    }
}))