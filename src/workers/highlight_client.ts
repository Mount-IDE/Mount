import type {Token} from "./highlight_shared";

type Listener = (fileId: string, reqId: number, lines: Map<number, Token[]> | null) => void;

const worker = new Worker(new URL("./highlight.worker.ts", import.meta.url), {type: "module"});
const listeners = new Set<Listener>();

worker.onmessage = (e) => {
    const {fileId, reqId, lines} = e.data;
    listeners.forEach(l => l(fileId, reqId, lines));
};

const registered = new Set<string>();

export const highlightClient = {
    register(key: string, wasmUrl: Uint8Array, querySource: string) {
        if (registered.has(key)) return;
        registered.add(key);
        worker.postMessage({type: "register", key, wasmUrl, querySource});
    },
    highlight(reqId: number, fileId: string, key: string, text: string, nodes: Record<string, string>) {
        worker.postMessage({type: "highlight", reqId, fileId, key, text, nodes});
    },
    close(fileId: string) {
        worker.postMessage({type: "close", fileId});
    },
    subscribe(l: Listener) {
        listeners.add(l);
        return () => listeners.delete(l);
    },
};