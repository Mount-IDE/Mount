type PendingEntry = {
    resolve: (v: any) => void
    reject: (e: any) => void
    timer: ReturnType<typeof setTimeout>
}

export class PluginRunner {
    private worker: Worker
    private pending = new Map<string, PendingEntry>();

    constructor() {
        this.worker = this.spawnWorker();
    }


    spawnWorker(): Worker {
        const worker = new Worker(new URL("../workers/lsp.worker.js", import.meta.url), {
            type: "module"
        })
        worker.onmessage = (e) => {
            const {id, ok, result, error} = e.data;
            const entry = this.pending.get(id);
            if (!entry) return;
            clearTimeout(entry.timer);
            this.pending.delete(id);
            ok ? entry.resolve(result) : entry.reject(new Error(error));
        };
        worker.onerror = (e) => {
            console.error('plugin worker crashed:', e.message);
        };


        return worker
    }

    async call(code: string, functionName: string, args: any, timeoutMs = 5000): Promise<any> {
        const id = crypto.randomUUID();
        return new Promise((resolve, reject) => {
            const timer = setTimeout(() => {
                this.pending.delete(id);
                this.worker.terminate();
                this.worker = this.spawnWorker();
                reject(new Error(`plugin "${functionName}" timed out after ${timeoutMs}ms`));
            }, timeoutMs);

            this.pending.set(id, {resolve, reject, timer});
            this.worker.postMessage({id, code, functionName, args});
        });
    }

    dispose() {
        this.worker.terminate();
    }

}