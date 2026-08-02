import "./styles/aside-launch.css";
import "@xterm/xterm/css/xterm.css";
import {projectStore} from "../../../stores/project_store.ts";
import {useEffect, useRef, useState} from "react";
import cross from "../../../assets/title-close.svg"
import {Terminal} from "@xterm/xterm";
import {FitAddon} from "@xterm/addon-fit";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {launchStore} from "../../../stores/launch_store.ts";

type Props = { active?: boolean };

type FlatLaunchTask = {
    command: string;
    env?: [string, string][];
    cwd?: string;
};

/*
type TerminalOutput = { id: string; data: string };
type TerminalExit = { id: string };*/


type LaunchProcess = {
    id: number,
    obj: LaunchObject
}

export default function AsideLaunch(_: Props) {

    let project = projectStore(state => state.current_project);
    //   let objects = project?.workspace.launch_objects ?? [];
    let references = project?.workspace.launch_references ?? [];

    let objects = launchStore(state => state.active_objects);

    let processes: LaunchProcess[] = [...objects].map((el, i) => ({id: i, obj: el}))


    let [current, setCurrent] = useState(0)


    return (
        <div id={"aside-launch"}>
            <div id={"aside-launch-header"}>
                {
                    //  objects.length == references.length &&
                    processes.map((el, i) =>
                        <LaunchPos current={i == current} key={i} proc={el} ref={references[i]}
                                   onClose={(proc) => {
                                       launchStore.getState().remove_active_object(proc.obj)
                                   }}
                                   onSelect={(proc) => setCurrent(proc.id)}
                                   selected={current == el.id}
                        />
                    )
                }
            </div>
            <div id={"aside-launch-terminal"}>
                {
                    //objects.length == references.length &&
                    processes.map((el, i) =>
                        <LaunchTerminal proc={el} key={i} selected={current == el.id}/>
                    )}

                {
                    processes.length == 0 &&
                    <div>
                        No Launches
                    </div>
                }
            </div>
        </div>
    )
}

type PosProps = {
    current: boolean
    proc: LaunchProcess
    ref: LaunchTemplateReference
    onClose: (proc: LaunchProcess) => void
    onSelect: (proc: LaunchProcess) => void
    selected: boolean
}


function LaunchPos(props: PosProps) {

    return (
        <div className={`launch-pos`}
             style={{
                 background: props.current ? "var(--border3)" : ""
             }}
             onClick={() => {
                 props.onSelect(props.proc)
             }}
        >
            <p>
                {props.ref.name}
            </p>
            <button
                onClick={(e) => {
                    e.stopPropagation()
                    props.onClose(props.proc)
                }}
            >
                <img src={cross}/>
            </button>
        </div>
    )
}


type TermProps = {
    proc: LaunchProcess
    selected: boolean
}


function LaunchTerminal(props: TermProps) {

    const ref = useRef<HTMLDivElement>(null)

    const fitAddonRef = useRef<FitAddon>(null)

    const termRef = useRef<Terminal>(null)

    //  const allowRef = useRef(false)

    const backendId = useRef<string>(null)

    const project = projectStore(state => state.current_project);

    /*  const lastSizeRef = useRef({
          rows: 0,
          cols: 0
      })*/

    const currenTask = useRef(0)

    useEffect(() => {
        if (!project) {
            return;
        }
        const elem = ref.current
        if (!elem) return
        const term = new Terminal({
            allowTransparency: true,
            cursorBlink: true,
            convertEol: true,
            fontFamily: "Consolas, 'Cascadia Mono', 'Courier New', monospace",
            fontSize: 13,
            lineHeight: 1.2,
            theme: {
                background: "rgba(0,0,0,0)",
                foreground: "#d7d7d7",
                cursor: "#ffffff",
                selectionBackground: "#334155",
            },
        })

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(elem);

        termRef.current = term
        fitAddonRef.current = fitAddon

        const fitAndResize = (() => {
            let raf = 0;

            return () => {
                cancelAnimationFrame(raf);

                raf = requestAnimationFrame(() => {
                    if (elem.offsetWidth === 0 || elem.offsetHeight === 0) return;

                    const oldCols = term.cols;
                    const oldRows = term.rows;

                    fitAddon.fit();

                    if (oldCols === term.cols && oldRows === term.rows) return;
                });
            };
        })();

        let outputUnlisten: UnlistenFn | null = null;
        let exitUnlisten: UnlistenFn | null = null;
        const resizeObserver = new ResizeObserver(() => {
            if (!backendId.current) return;
            fitAndResize();
        });
        resizeObserver.observe(elem);


        term.onData((data) => {
            let back = backendId.current;
            if (back) {
                invoke("write_launch", {id: back, text: data}).then();
            }

        })

        const queueOutput = (() => {
            let buffer = "";
            let frame: number | null = null;
            const flush = () => {
                frame = null;
                term.write(buffer);
                buffer = "";
            };
            return (data: string) => {
                buffer += data;
                if (frame === null) frame = requestAnimationFrame(flush);
            };
        })();


        let tasks = flatTasks(props.proc.obj.tasks);


        async function runTask(index: number) {
            if (index >= tasks.length) {
                return;
            }
            let task = tasks[index];
            try {
                let id = await invoke<string>("launch_task", {
                    task,
                    project
                });
                //  fitAndResize();
                backendId.current = id;
            } catch (e) {
                console.error(e)
            }
        }


        listen<{ id: string, data: string }>("launch-read", (val) => {
            if (val.payload.id == backendId.current) {
                queueOutput(val.payload.data)
            }
        }).then((unlisten) => outputUnlisten = unlisten)

        listen<number>("launch-exit", (code) => {
            currenTask.current += 1;
            queueOutput(`LAUNCH EXITED WITH CODE ${code}`)
            backendId.current = null;
            runTask(currenTask.current).then();
        }).then((unlisten) => exitUnlisten = unlisten)

        runTask(0).then();
        return () => {
            term.dispose()
            resizeObserver.disconnect()
            outputUnlisten?.()
            exitUnlisten?.()
            if (backendId.current) {
                invoke("close_launch", {
                    id: backendId.current
                }).then()
            }
        }

    }, []);

    return (
        <div
            style={{
                display: props.selected ? "block" : "none"
            }}
            className={"launch-terminal"}>
            <div ref={ref} className={"launch-term"}>

            </div>
        </div>
    )
}


function flatTasks(tasks: LaunchTask[]) {
    let res: FlatLaunchTask[] = []
    for (let task of tasks) {
        if ("SINGLE" in task) {
            res.push(task["SINGLE"] as FlatLaunchTask)
        } else {
            let val = task["GRAPH"]!;
            let next = flatTasks([val!.next]);
            res = [...res, ...next];
            res.push({
                command: val.command, cwd: val.cwd, env: val.env
            })
        }
    }

    return res;
}