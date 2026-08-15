import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";
import ProjectTemplates from "./ProjectTemplates.tsx";
import ProjectMeta from "./ProjectMeta.tsx";
import ProjectPackages from "./ProjectPackages.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {useEffect, useState} from "react";
import Load from "../../common/Load.tsx";
import {listen} from "@tauri-apps/api/event";
import {noteStore, NotificationType} from "../../../stores/note_store.ts";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {mapProjectButton} from "../../../utils/project-buttons.ts";
import {invoke} from "@tauri-apps/api/core";


/**
 *
 * @constructor
 */
export default function CreateProject() {

    const create_project = createProjectStore(state => state.create_project)
    const current_template = cacheStore(state => state.currentTemplate)
    const set_current_path = projectStore(state=>state.set_path_to_current_project);

    const [isCreating, setIsCreating] = useState(false)
    const [startEvent, setStartEvent] = useState<string | null>(null)
    const [endEvent, setEndEvent] = useState<string | null>(null)
    /**
     *
     */
    async function create_project_() {
        if (current_template) {
            setIsCreating(true)
            let res = await create_project(current_template!);

            if (res[0] == 0) {
                set_current_path(res[1]);
                createProjectStore.getState().close();
                const buttons = res[2]!.workspace.buttons;
                let left_top = buttons.filter(el => el.pos == "LeftTop")
                let left_bot = buttons.filter(el => el.pos == "LeftBottom")
                let right_top = buttons.filter(el => el.pos == "RightTop")

                let left_top_2 =
                    left_top.map<IAsideButton>(mapProjectButton);

                let left_bot_2 =
                    left_bot.map<IAsideButton>(mapProjectButton);

                let right_top_2 =
                    right_top.map<IAsideButton>(mapProjectButton);
                asideButtonsStore.getState().load_left(left_top_2);
                asideButtonsStore.getState().load_bottom(left_bot_2);
                asideButtonsStore.getState().load_right(right_top_2);
                try {
                    await invoke("unwatch_project");
                    await invoke("close_window_terminals");
                } catch (e) {
                    console.error(e)
                }
                noteStore.getState().add_note({
                    type: NotificationType.NOTE,
                    text: "Project was created"
                }, 2000)
            } else {
                noteStore.getState().add_note({
                    type: NotificationType.ERR,
                    text: "Error while creating project"
                })
            }
            setIsCreating(false)
        }
    }

    const [pointCount, setPointCount] = useState(0)

    useEffect(() => {

        let clear: number;
        console.log("RERENDER")
        const unlisten = listen<string>("task-start", (d) => {
            let val = d.payload;
            setStartEvent(val)
            setEndEvent(null)
            clearInterval(clear)
            clear = setInterval(() => {
                setPointCount(prev => {
                    if (prev >= 3) {
                        return 0;
                    }
                    return prev + 1;
                })
            }, 500)
        })

        const unlisten2 = listen<string>("task-end", (d) => {
            let val = d.payload;
            setEndEvent(val)
            setStartEvent(null)
        })

        const unlisten3 = listen<string>("task-error", (d) => {
            setEndEvent(null);
            setStartEvent(null);

            noteStore.getState().add_note({
                type: NotificationType.ERR,
                text: `Error while running task ${d.payload}`
            })
        })


        const unlisten4 = listen<string>("project", (d) => {
            //console.log("ZWRITE")
            let parsed = JSON.parse(d.payload);
            // console.log(parsed);
            let obj = JSON.parse(parsed.data) as Dependency[];
            let conflicts = obj.filter(el => el.level == "CONFLICTS");
            let optional = obj.filter(el => el.level == "OPTIONAL");
            let critical = obj.filter(el => el.level == "CRITICAL");
            let text = `Failed to check dependencies.\n
conflicts:\n
\toptional - ${optional.length}\n
\tcritical - ${critical.length}\n
\tconflicts - ${conflicts.length}
            `

            let type = critical.length > 0 || conflicts.length > 0 ? NotificationType.ERR : NotificationType.WARN

            noteStore.getState().add_note({
                text,
                type
            })

            setStartEvent(null)
            setEndEvent(null)
        })


        return () => {
            unlisten.then(fn => fn())
            unlisten2.then(fn => fn())
            unlisten3.then(fn => fn())
            unlisten4.then(fn => fn())
            clearInterval(clear)
        }
    }, [])

    return (
        <div
            id={"create-project"}>
            {
                startEvent != null &&
                <p id={"current-event"}>
                    Running task {startEvent}{".".repeat(pointCount)}
                </p>
            }
            {
                endEvent != null &&
                <p id={"current-event"}>
                    Ended task {endEvent}
                </p>
            }
            {
                isCreating &&
                <Load/>
            }
            <div id={"create-project-top"}>
                <div id={"create-project-top-label"}>Create Project</div>
            </div>
            <div id={"create-project-main"}>
                <ProjectTemplates/>
                <hr id={"create-project-hr"}/>
                <ProjectMeta/>
                <ProjectPackages/>
            </div>
            <div id={"create-project-bottom"}>
                <div id={"create-project-buttons"}>
                    <Button title={"Close"} cb={() => close_project()}/>
                    <Button title={"Create Project"} cb={()=>create_project_()}/>
                </div>
            </div>
        </div>
    )
}
