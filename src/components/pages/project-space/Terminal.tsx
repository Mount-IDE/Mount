import "./styles/terminal.css";
import "@xterm/xterm/css/xterm.css";
import {FitAddon} from "@xterm/addon-fit";
import {Terminal as XTerm} from "@xterm/xterm";
import {invoke} from "@tauri-apps/api/core";
import {listen, type UnlistenFn} from "@tauri-apps/api/event";
import {useCallback, useEffect, useRef, useState} from "react";
import {projectStore} from "../../../stores/project_store.ts";
import plus from "../../../assets/plus.svg";
import cross from "../../../assets/title-close.svg";
import {cacheStore} from "../../../stores/cache_store.ts";

type TerminalOutput = { id: string; data: string };
type TerminalExit = { id: string };
type TerminalTab = { id: string; title: string; exited: boolean; shell: string };

type Props = { active?: boolean };

function makeTerminalId() {
    return `terminal-${crypto.randomUUID?.() ?? `${Date.now()}-${Math.random()}`}`;
}


/**
 *
 * @param props
 * @constructor
 */
export default function Terminal(props: Props) {
    const cwd = projectStore((state) => state.path_to_current_project);
    const [tabs, setTabs] = useState<TerminalTab[]>([]);
    const [activeId, setActiveId] = useState<string | null>(null);
    const nextIndex = useRef(1);

    const shells = cacheStore((state) => state.shells);
    const [currentShell, setCurrentShell] = useState<string | null>(shells[0] ?? null);

    const createTerminal = useCallback(() => {
        const index = nextIndex.current++;
        const id = makeTerminalId();
        const shell = currentShell ?? shells[0] ?? "cmd";
        const tab: TerminalTab = {id, title: `terminal ${index}`, exited: false, shell};
        setTabs((prev) => [...prev, tab]);
        setActiveId(id);
    }, [currentShell, shells]);

    useEffect(() => {
        if (props.active && tabs.length === 0) createTerminal();
    }, [props.active, tabs.length, createTerminal]);

    const closeTerminal = useCallback((id: string) => {
        const next = tabs.filter((tab) => tab.id !== id);
        setTabs(next);
        if (activeId === id) setActiveId(next.length > 0 ? next[next.length - 1].id : null);
    }, [activeId, tabs]);

    const markExited = useCallback((id: string) => {
        setTabs((prev) =>
            prev.map((tab) =>
                tab.id === id ? {...tab, exited: true, title: `${tab.title} exited`} : tab
            )
        );
    }, []);

    return (
        <div id="terminal-page">
            <div id="terminal-tabs">
                <div id="terminal-label">Terminal</div>
                <div id="terminal-other">
                    {tabs.map((tab) => (
                        <button
                            key={tab.id}
                            className={tab.id === activeId ? "terminal-tab terminal-tab-active" : "terminal-tab"}
                            onClick={() => setActiveId(tab.id)}
                        >
                            <span>{tab.title}</span>
                            <span
                                className="terminal-tab-close"
                                onClick={(e) => {
                                    e.stopPropagation();
                                    closeTerminal(tab.id);
                                }}
                            >
                <img src={cross}/>
              </span>
                        </button>
                    ))}
                </div>
                <div id="terminal-add">
                    <button id="terminal-new" onClick={createTerminal}>
                        <img src={plus}/>
                    </button>
                    <select
                        id="terminal-add-list"
                        onChange={(e) => setCurrentShell(e.currentTarget.value)}
                        value={currentShell ?? ""}
                    >
                        {shells.map((s, i) => (
                            <option key={i} value={s}>
                                {s}
                            </option>
                        ))}
                    </select>
                </div>
            </div>
            <div id="terminal-body">
                {tabs.map((tab) => (
                    <TerminalProc
                        key={tab.id}
                        id={tab.id}
                        shell={tab.shell}
                        cwd={cwd || "."}
                        active={Boolean(props.active) && tab.id === activeId}
                        onExit={markExited}
                    />
                ))}
            </div>
        </div>
    );
}

type TermProps = {
    id: string;
    cwd: string;
    active: boolean;
    shell: string;
    onExit: (id: string) => void;
};


/**
 *
 * @param id
 * @param cwd
 * @param active
 * @param shell
 * @param onExit
 * @constructor
 */
function TerminalProc({id, cwd, active, shell, onExit}: TermProps) {
    const ref = useRef<HTMLDivElement>(null);
    const termRef = useRef<XTerm | null>(null);
    const readyRef = useRef(false);
    const disposedRef = useRef(false);
    const backendIdRef = useRef<string | null>(null);
    const lastSizeRef = useRef({cols: 80, rows: 24});
    const fitAddonRef = useRef<FitAddon | null>(null);

    useEffect(() => {
        const element = ref.current;
        if (!element) return;

        disposedRef.current = false;
        readyRef.current = false;
        const backendId = `${id}-${crypto.randomUUID?.() ?? `${Date.now()}-${Math.random()}`}`;
        backendIdRef.current = backendId;

        const term = new XTerm({
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
        });
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(element);
        fitAddonRef.current = fitAddon;
        termRef.current = term;

        const fitAndResize = () => {
            if (disposedRef.current || element.offsetWidth === 0 || element.offsetHeight === 0) return;
            try {
                fitAddon.fit();
            } catch {
                return;
            }
            lastSizeRef.current = {cols: term.cols, rows: term.rows};
            if (readyRef.current) {
                void invoke("resize_terminal", {id: backendId, rows: term.rows, cols: term.cols});
            }
        };

        let outputUnlisten: UnlistenFn | null = null;
        let exitUnlisten: UnlistenFn | null = null;
        let resizeObserver = new ResizeObserver(fitAndResize);
        resizeObserver.observe(element);

        term.onData((data) => {
            if (!readyRef.current) return;
            void invoke("write_terminal", {id: backendId, data});
        });

        const queueOutput = (() => {
            let buffer = "";
            let frame: number | null = null;
            const flush = () => {
                frame = null;
                if (disposedRef.current) return;
                term.write(buffer);
                buffer = "";
            };
            return (data: string) => {
                buffer += data;
                if (frame === null) frame = requestAnimationFrame(flush);
            };
        })();

        listen<TerminalOutput>("terminal-output", (e) => {
            if (e.payload.id === backendIdRef.current) queueOutput(e.payload.data);
        }).then((unlisten) => {
            outputUnlisten = unlisten;
        });

        listen<TerminalExit>("terminal-exit", (e) => {
            if (e.payload.id === backendIdRef.current) {
                readyRef.current = false;
                onExit(id);
                term.writeln("\r\n[process exited]");
                void invoke("close_terminal", {id: backendId});
            }
        }).then((unlisten) => {
            exitUnlisten = unlisten;
        });

        invoke<string>("open_terminal", {
            id: backendId,
            shell,
            cwd,
            rows: lastSizeRef.current.rows,
            cols: lastSizeRef.current.cols,
        })
            .then(() => {
                if (disposedRef.current) {
                    void invoke("close_terminal", {id: backendId});
                    return;
                }
                readyRef.current = true;
                fitAndResize();
                term.focus();
            })
            .catch((err) => term.writeln(`failed to open terminal: ${String(err)}`));

        return () => {
            disposedRef.current = true;
            readyRef.current = false;
            term.dispose();
            resizeObserver.disconnect();
            outputUnlisten?.();
            exitUnlisten?.();
            void invoke("close_terminal", {id: backendId});
            termRef.current = null;
            fitAddonRef.current = null;
        };
    }, [id, cwd, shell, onExit]);

    useEffect(() => {
        if (!active) return;
        requestAnimationFrame(() => {
            const term = termRef.current;
            const backendId = backendIdRef.current;
            if (!term) return;
            try {
                fitAddonRef.current?.fit();
            } catch {
                return;
            }
            lastSizeRef.current = {cols: term.cols, rows: term.rows};
            if (backendId && readyRef.current) {
                void invoke("resize_terminal", {id: backendId, rows: term.rows, cols: term.cols});
            }
            term.focus();
        });
    }, [active]);

    return (
        <div
            className={active ? "terminal-proc" : "terminal-proc terminal-proc-hidden"}
        >
            <div ref={ref} className="terminal-proc-fit"/>
        </div>
    );
}
