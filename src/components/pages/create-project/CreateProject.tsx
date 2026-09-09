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
import {motion} from "motion/react";

/**
 *
 * @constructor
 */
export default function CreateProject() {

    const create_project = createProjectStore(state => state.create_project)
    const current_template = cacheStore(state => state.currentTemplate)
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
            console.log(res)
            if (res[0] == 0) {
                createProjectStore.getState().close();
                noteStore.getState().add_note({
                    type: NotificationType.NOTE,
                    text: "Project was created"
                }, 2000)
                await projectStore.getState().open_project(res[2]!)
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
        <motion.div
            initial={{opacity: 0}}
            animate={{opacity: 1}}
            exit={{opacity: 0}}
            transition={{duration: 0.2}}
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
                {
                    current_template != null &&
                    <>
                        <ProjectMeta/>
                        <ProjectPackages/>
                    </>

                }
                {
                    current_template == null &&
                    <p style={{
                        marginLeft: "5%",
                        position: "absolute",
                        left: "45%",
                        top: "45%",
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                        color: "var(--subtitle)",
                        fontSize: "14pt"
                    }}>
                        Select the project template
                    </p>
                }
            </div>
            <div id={"create-project-bottom"}>
                <div id={"create-project-buttons"}>
                    <Button title={"Close"} cb={() => close_project()}/>
                    <Button title={"Create Project"} cb={() => create_project_()}/>
                </div>
            </div>
        </motion.div>
    )
}
