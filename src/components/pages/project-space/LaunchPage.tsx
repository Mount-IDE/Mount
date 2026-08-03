import "./styles/launch-page.css"
import Button from "../../common/Button.tsx";
import {launchStore} from "../../../stores/launch_store.ts";
import {projectStore} from "../../../stores/project_store.ts";

import plus from "../../../assets/plus.svg"
import minus from "../../../assets/title-wrap.svg"
import LaunchSection from "./LaunchSection.tsx";
import {useEffect, useMemo, useState} from "react";
import {noteStore, NotificationType} from "../../../stores/note_store.ts";
import pageStore from "../../../stores/page_store.ts";


export default function LaunchPage() {

    const current_launch_template = launchStore(state => state.current_template)

    const project = projectStore(state => state.current_project);
    const template = project?.template;

    const [curRef, setCurRef] = useState<number | null>(null)

    // fn
    const set_current_template = launchStore(state => state.set_current_template)
    const set_current_template_by_ref = launchStore(state => state.set_current_temp_by_ref)
    const opened = launchStore(state => state.set_opened)
    const setCurrentLaunchReference = launchStore(state => state.set_current_launch)
    const setCurrentLaunchProject = projectStore(state => state.set_current_launch);
    const find = launchStore(state => state.find_temp)

    const templates = project?.workspace.launch_templates ?? [];
    const references = launchStore(state => state.references)

    useEffect(() => {
        if (references.length == 0) {
            launchStore.getState().add_references(project?.workspace.launch_references ?? [])
        }
    }, [project]);

    console.log("BASE", curRef, references)

    let meta = useMemo<LaunchSection>(() => {

        let val = find(-1, "name", curRef ?? -1)
        if (!val) {
            let ref = references.find(el => el.id == curRef)
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
    }, [curRef, references, templates])

    const [showContext, setShowContext] = useState(false)

    const updateReferences = projectStore(state => state.update_launch_references);
    const updateObjects = projectStore(state => state.update_launch_objects)


    /**
     *
     */
    async function apply(): Promise<boolean> {
        if (current_launch_template) {
            await updateReferences(references);
            const found =
                references.find(el => el.id == curRef)
            if (found) {
                setCurrentLaunchReference(found!)
                setCurrentLaunchProject(found!.id);
            }
            let res = await updateObjects(current_launch_template);
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
        launchStore.getState().add_reference(ref)
        setCurRef(ref.id)
        set_current_template(el)
    }

    /**
     *
     */
    function removeReference() {
        launchStore.getState().rem_reference(curRef ?? -1)
    }


    function selectReference(ref: LaunchTemplateReference) {
        set_current_template_by_ref(ref, templates)
        setCurRef(ref.id)
    }

    useEffect(() => {
        console.log("TEMP", current_launch_template)
    }, [current_launch_template]);

    useEffect(() => {
        console.log(references[curRef ?? -1])
    }, [curRef, references]);


    return (
        <div id={"launch-page"}>
            <div id={"launch-top"}>
                <div id={"launch-label"}>
                    <p>Launch Configurations</p>
                </div>
            </div>
            <div id={"launch-main"}>
                <div id={"launch-left"}>
                    <div id={"launch-left-head"}>
                        <button className={"launch-head-bt"} onClick={() => {
                            setShowContext(prev => !prev)
                        }}>
                            <img src={plus}/>
                        </button>
                        <button className={"launch-head-bt"} onClick={removeReference}>
                            <img src={minus}/>
                        </button>
                        {
                            showContext &&
                            <div id={"launch-template-select"}>
                                {templates.map((el, i) =>
                                    <LaunchSelect references={references}
                                                  add_ref={(r) => addReference(el, r)}
                                                  obj={el} key={i} temp={template}
                                    />
                                )}
                            </div>
                        }
                    </div>
                    <div id={"launch-left-list"}>
                        {
                            references.map((el, key) =>
                                <LaunchRef current={curRef == el.id} key={key} obj={el}
                                           cb={selectReference}/>
                            )
                        }
                    </div>
                </div>
                <div id={"launch-right"}>
                    <div id={"launch-sections"}>
                        {
                            current_launch_template !== null && curRef != null &&
                            <>
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
                            </>
                        }
                    </div>
                    <div id={"launch-before"}></div>
                </div>
            </div>
            <div id={"launch-bottom"}>
                <div id={"launch-buttons"}>
                    <Button width={100} title={"Ok"} cb={async () => {
                        let res = await apply()
                        if (res) {
                            opened(false)
                            pageStore.getState().setFilter(false)
                        } else {
                            noteStore.getState().add_note({
                                text: "Cannot apply launch configuration",
                                type: NotificationType.WARN
                            })
                        }
                    }}/>
                    <Button width={100} title={"Cancel"} cb={() => {
                        opened(false)
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
        </div>

    )
}


type RefProps = {
    obj: LaunchTemplateReference
    cb: (obj: LaunchTemplateReference) => void
    current: boolean
}


/**
 *
 * @param props
 * @constructor
 */
function LaunchRef(props: RefProps) {
    const path = props.obj.icon ? `/builtin/fs-icons/${props.obj.icon}` : "/builtin/fs-icons/any.svg"
    return (
        <div onClick={() => props.cb(props.obj)}
             className={`launch-left-ref ${props.current ? "selected" : ""}`}
        >
            <div className={"launch-left-ref-img"}>
                <img src={path}/>
            </div>
            {props.obj.name}
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
function LaunchSelect(props: SelectProps) {
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