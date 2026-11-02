import "./styles/launch-page.css"
import Button from "@common/Button.tsx";
import {launchStore} from "@stores/launch_store.ts";
import {projectStore} from "@stores/project_store.ts";

import plus from "@assets/plus.svg"
import minus from "@assets/title-wrap.svg"
import LaunchSection from "./LaunchSection.tsx";
import {useEffect, useMemo, useState} from "react";
import {noteStore, NotificationType} from "@stores/note_store.ts";
import pageStore from "@stores/page_store.ts";
import {motion} from "motion/react";
import Icon from "@common/Icon.tsx";
import Table from "@common/Table.tsx";
import launchService from "@/src/services/launch_service.ts";


export default function LaunchPage() {

    const current_launch_template = launchStore(state => state.current_template)

    const project = projectStore(state => state.current_project);
    const project_template = project?.template;


    // const [curRef, setCurRef] = useState<number | null>(null)
    const curRef = projectStore(state => state.current_project?.workspace.current_launch) ?? null

    const templates = project?.workspace.launch_templates ?? [];


    const changed_references = launchStore(state => state.changed_references)

    /**
     * copy
     */
    const references_copy = launchStore(state => state.copy_references)


    useEffect(() => {
        if (project?.workspace.launch_references) {
            launchStore.setState({
                copy_references: [...project.workspace.launch_references]
            })
        }
    }, [project?.workspace.launch_references]);


    useEffect(() => {
        if (current_launch_template == null && curRef != null) {
            let ref = projectStore.getState().current_project?.workspace.launch_references.find(el => el.id == curRef)
            if (ref) {
                let temp =
                    projectStore.getState().current_project!.workspace.launch_templates.find(el => el.id == ref.template[1])
                if (temp) {
                    launchStore.setState({
                        current_template: temp
                    })
                }
            }
        }
    }, []);


    /**
     * meta of current template
     */
    let meta = useMemo<LaunchSection>(() => {

        let val = references_copy[curRef ?? -1]?.results[-1]?.["name"] as string | undefined
        if (!val) {
            console.log("REFS", references_copy)
            let ref = references_copy.find(el => el.id == curRef)
            if (ref) {
                let needed_temp = templates.find(el => el.id == ref?.template[1])
                if (needed_temp) {
                    val = needed_temp.title
                } else {
                    val = "Not Found"
                }
            } else {
                val = "Not found"
            }
        }
        return ({
            id: -1,
            options: [
                {
                    def: val,
                    id: "name",
                    title: "Name",
                    typ: {
                        typ: "input"
                    }
                }
            ],

        })
    }, [curRef, references_copy, templates])

    const [showContext, setShowContext] = useState(false)

    const updateReferences = projectStore(state => state.update_launch_references);

    //const updateObjects = projectStore(state => state.update_launch_objects)


    /**
     *
     */
    async function apply(): Promise<boolean> {
        if (current_launch_template) {
            launchStore.setState({
                changed_references: new Set()
            })
            await updateReferences(references_copy);
            const found =
                references_copy.find(el => el.id == curRef)
            if (found) {
                launchService.set_current_launch(found.id)
            }
            let res = await launchService.update_launch_objects();
            if (!res) {
                return false
            }
            if (res[1] != null) {
                projectStore.getState().save_project(res[1])
            }
            return true;
        }
        return true; // no matter, selected template, or not
    }

    /**
     *
     * @param el
     * @param ref
     */
    function addReference(el: LaunchTemplate, ref: LaunchTemplateReference) {
        // launchStore.getState().add_reference(ref)
        launchService.add_ref(ref)
        launchService.set_current_launch(ref.id)
        launchStore.setState({
            current_template: el
        })
    }

    /**
     *
     */
    function removeReference() {
        if (curRef !== null) {
            launchService.remove_ref(curRef)
            console.log(launchService.set_current_launch(null))
        }
    }


    /**
     *
     * @param ref
     */
    function selectReference(ref: LaunchTemplateReference) {
        let needed = references_copy.find(el => el.id == ref.id)
        if (needed) {
            let temp = templates.find(el => el.id == needed?.template[1])
            if (temp) {
                launchStore.setState({
                    current_template: temp
                })
                launchService.set_current_launch(ref.id)
            }
        }
    }

    return (
        <motion.div
            initial={{opacity: 0}}
            animate={{opacity: 1}}
            exit={{opacity: 0}}
            transition={{duration: 0.2}}
            id={"launch-page"}>
            <div id={"launch-top"}>
                <div id={"launch-label"}>
                    <p>Launch Configurations</p>
                </div>
            </div>
            <div id={"launch-main"}>
                <div id={"launch-left"}>
                    <div id={"launch-left-head"} className={"items-center justify-center"}>
                        <Icon src={plus} size={"80%"} onClick={() => setShowContext(prev => !prev)}/>
                        <Icon src={minus} size={"80%"} onClick={() => removeReference()}/>
                        {
                            showContext &&
                            <div id={"launch-template-select"} className={"z-100"}>
                                {templates.map((el, i) =>
                                    <LaunchSelectElements references={references_copy}
                                                          add_ref={(r) => {
                                                              addReference(el, r)
                                                              setShowContext(false)
                                                          }}
                                                          obj={el} key={i} temp={project_template}
                                    />
                                )}
                            </div>
                        }
                    </div>
                    <div id={"launch-left-list"}>
                        {
                            references_copy.map((el, key) =>
                                <LaunchRefElement
                                    selected={curRef == el.id}
                                    key={key}
                                    obj={el}
                                    cb={selectReference}
                                    changed={changed_references.has(el.id)}
                                />
                            )
                        }
                    </div>
                </div>
                <div id={"launch-right"} className={"relative"}>
                    {
                        (curRef == null) &&
                        <p style={{margin: "auto"}}
                           className={"flex select-none flex-col items-center justify-center text-(--subtitle) absolute inset-0"}>Create
                            or select exist launch configuration</p>
                    }
                    {
                        current_launch_template !== null && curRef != null &&
                        <LaunchPageSections
                            project={project}
                            curRef={curRef}
                            current_launch_template={current_launch_template}
                            meta={meta}
                        />
                    }
                    <div id={"launch-before"}></div>
                </div>
            </div>
            <div id={"launch-bottom"}>
                <div id={"launch-buttons"}>
                    <Button width={100} title={"Ok"} cb={async () => {
                        let res = await apply()
                        if (res) {
                            //opened(false)
                            launchStore.getState().set_opened(false)
                            pageStore.getState().setFilter(false)
                        } else {
                            noteStore.getState().add_note({
                                text: "Cannot apply launch configuration",
                                type: NotificationType.WARN
                            })
                        }
                    }}/>
                    <Button width={100} title={"Cancel"} cb={() => {
                        // opened(false)
                        launchStore.getState().set_opened(false)
                        pageStore.getState().setFilter(false)
                    }}/>
                    <Button width={100} title={"Apply"} cb={async () => {
                        let res = await apply()
                        if (!res) {
                            noteStore.getState().add_note({
                                text: "Cannot apply launch configuration",
                                type: NotificationType.WARN
                            })
                        }
                    }}/>
                </div>
            </div>
        </motion.div>

    )
}


type SectionsProps = {
    current_launch_template: LaunchTemplate
    curRef: number
    project: IProject | null
    meta: LaunchSection
}

/**
 *
 * @param props
 * @constructor
 */
function LaunchPageSections(props: SectionsProps) {
    const {project, meta, curRef, current_launch_template} = props

    return (
        <div id={"launch-sections"}>
            <p className={"text-(--subtitle) text-[10pt] font-[400]"}>{current_launch_template.title}</p>
            <LaunchSection cur_ref={curRef}
                           obj={meta}
                           functions={current_launch_template?.functions ?? []}
                           project={project}
            />
            {
                current_launch_template?.sections.map((el, i) =>
                    <LaunchSection cur_ref={curRef}
                                   key={i}
                                   obj={el}
                                   functions={current_launch_template?.functions ?? []}
                                   project={project}
                    />
                )
            }
            <div className={""}>
                <p className={"text-(--subtitle)"}>Before Launch</p>
                <Table rows={[
                    {
                        data: ["3", "4", "5"]
                    }
                ]} add_row={() => {
                }} rem_row={(_i) => {

                }} swap_rows={(_first, _second) => {

                }} style={"select-none"}/>
            </div>
        </div>
    )
}

type RefProps = {
    obj: LaunchTemplateReference
    cb: (obj: LaunchTemplateReference) => void
    selected: boolean
    changed: boolean
}


/**
 *
 * @param props
 * @constructor
 */
function LaunchRefElement(props: RefProps) {
    const path = props.obj.icon ? `/builtin/fs-icons/${props.obj.icon}` : "/builtin/fs-icons/any.svg"
    return (
        <div onClick={() => props.cb(props.obj)}
             className={`select-none launch-left-ref ${props.selected ? "selected" : ""}`}
        >
            <Icon src={path} size={"15px"}/>
            {props.obj.name}
            {props.changed && <span style={{
                color: props.selected ? "var(--title)" : "var(--subtitle)"
            }} className={"text-(--subtitle)"}>*</span>}
        </div>
    )
}


type SelectProps = {
    references: LaunchTemplateReference[],
    add_ref: (ref: LaunchTemplateReference) => void
    obj: LaunchTemplate
    temp: ITemplate | undefined
}

/**
 *
 * @param props
 * @constructor
 */
function LaunchSelectElements(props: SelectProps) {
    const path = props.obj.icon ? `/builtin/fs-icons/${props.obj.icon}` : "/builtin/fs-icons/any.svg"
    return (
        <div className={"launch-template-select"}
             onClick={() => {
                 let last = 0;
                 if (props.references.length > 0) {
                     last = props.references[props.references.length - 1].id + 1;
                 }
                 props.add_ref({
                     id: last,
                     name: props.obj.title,
                     results: {},
                     scheme: 0,
                     template: [props.temp?.id ?? "", props.obj.id]
                 })
             }}
        >
            <div className={"launch-template-select-img"}>
                <img src={path}/>
            </div>
            {props.obj.title}
        </div>
    )
}