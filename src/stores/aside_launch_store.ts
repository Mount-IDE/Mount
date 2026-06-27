import {create} from "zustand";

export type LaunchRunStatus = "queued" | "running" | "exited" | "failed";

export interface LaunchRunWindow {
    runId: string;
    launchReference: number;
    title: string;
    object: LaunchObject;
    status: LaunchRunStatus;
}

interface Type {
    windows: LaunchRunWindow[];
    activeRunId: string | null;
    start_launch: (object: LaunchObject, title: string) => void;
    close_launch: (runId: string) => void;
    set_active: (runId: string | null) => void;
    set_status: (runId: string, status: LaunchRunStatus) => void;
}

export const asideLaunchStore = create<Type>((set, get) => ({
    windows: [],
    activeRunId: null,
    start_launch(object: LaunchObject, title: string): void {
        const runId = `launch-${object.launch_reference}-${Date.now()}`;
        const nextWindow: LaunchRunWindow = {
            runId,
            launchReference: object.launch_reference,
            title,
            object,
            status: "queued",
        };

        set({
            windows: [
                ...get().windows.filter((window) => window.launchReference !== object.launch_reference),
                nextWindow,
            ],
            activeRunId: runId,
        });
    },
    close_launch(runId: string): void {
        const windows = get().windows.filter((window) => window.runId !== runId);
        const activeRunId = get().activeRunId === runId ? windows[windows.length - 1]?.runId ?? null : get().activeRunId;
        set({windows, activeRunId});
    },
    set_active(runId: string | null): void {
        set({activeRunId: runId});
    },
    set_status(runId: string, status: LaunchRunStatus): void {
        set({
            windows: get().windows.map((window) =>
                window.runId === runId ? {...window, status} : window
            ),
        });
    },
}));
