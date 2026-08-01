import "./styles/launch-page.css"
import Button from "../../common/Button.tsx";
import {launchStore} from "../../../stores/launch_store.ts";
import {projectStore} from "../../../stores/project_store.ts";

import plus from "../../../assets/plus.svg"
import minus from "../../../assets/title-wrap.svg"
import LaunchSection from "./LaunchSection.tsx";
import {useEffect, useMemo, useState} from "react";
import {LOG} from "../../../utils/utils.ts";


export default function LaunchPage() {
    const current_launch_template = launchStore(state => state.current_template)
    const project = projectStore(state => state.current_project);
    const template = project?.template;
    const set_current_template = launchStore(state => state.set_current_template)
    const set_current_template_by_ref = launchStore(state => state.set_current_temp_by_ref)

    const opened = launchStore(state => state.set_opened)
    const setCurrentLaunchReference = launchStore(state => state.set_current_launch)
    const setCurrentLaunchProject = projectStore(state => state.set_current_launch);

    const [curRef, setCurRef] = useState(-1)
    const find = launchStore(state => state.find_temp)

    const templates = project?.workspace.launch_templates ?? [];
    const references = project?.workspace.launch_references ?? [];

    let meta = useMemo<LaunchSection>(() => ({
        id: -1,
        options: [
            {
                def:
                    find(
                        -1,
                        "name",
                        curRef,
                        project)
                    ?? templates.find(e => e.id == (references[curRef]?.template?.[1] ?? -1))?.title ?? "",
                id: "name",
                title: "Name",
                typ: {
                    typ: "input"
                }
            }
        ],

    }), [curRef, project])
    useEffect(() => {
        LOG(`META ${JSON.stringify(meta)}`)
    }, [meta]);


    const [showContext, setShowContext] = useState(false)

    const updateReferences = projectStore(state => state.update_launch_references);
    const updateObjects = projectStore(state => state.update_launch_objects)

    // const setReferences = launchStore(state => state.set_references);

    useEffect(() => {
        updateReferences(project?.workspace.launch_references ?? []).then()
    }, [project?.workspace.launch_references]);

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
                        <button className={"launch-head-bt"} onClick={() => {
                            let new_ref = references.filter(el => el.id != curRef);
                            updateReferences(new_ref).then();
                        }}>
                            <img src={minus}/>
                        </button>
                        {
                            showContext &&
                            <div id={"launch-template-select"}>
                                {templates.map((el, i) =>
                                    <LaunchSelect references={references} add_ref={(ref) => {
                                        let copy = [...references]
                                        copy.push(ref);
                                        updateReferences(copy).then()
                                        setCurRef(ref.id)
                                        set_current_template(el)

                                    }} obj={el} key={i} temp={template}/>
                                )}
                            </div>
                        }
                    </div>
                    <div id={"launch-left-list"}>
                        {
                            references.map((el, key) =>
                                <LaunchRef current={curRef == el.id} key={key} obj={el}
                                           cb={(obj) => {
                                               set_current_template_by_ref(obj, templates)
                                               setCurRef(el.id)
                                           }}/>
                            )
                        }
                    </div>
                </div>
                <div id={"launch-right"}>
                    <div id={"launch-sections"}>
                        {
                            current_launch_template !== null &&
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
                    {/*<hr/>*/}
                    <div id={"launch-before"}></div>
                </div>
            </div>
            <div id={"launch-bottom"}>
                <div id={"launch-buttons"}>
                    <Button width={100} title={"Ok"} cb={async () => {
                        if (current_launch_template) {
                            updateReferences(references).then();
                            let res = await updateObjects(current_launch_template);
                            if (!res) {
                                return
                            }
                            const found = references.find(el => el.id == curRef)
                            if (found) {
                                setCurrentLaunchReference(found!)
                                setCurrentLaunchProject(found!.id);
                            }
                            opened(false)
                        }
                    }}/>
                    <Button width={100} title={"Cancel"} cb={() => {
                        opened(false)
                    }}/>
                    <Button width={100} title={"Apply"} cb={async () => {
                        if (current_launch_template) {
                            updateReferences(references).then();
                            const found = references.find(el => el.id == curRef)
                            if (found) {
                                setCurrentLaunchReference(found!)
                                setCurrentLaunchProject(found!.id);
                            }
                            await updateObjects(current_launch_template);


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

function LaunchRef(props: RefProps) {
    LOG(props.obj)
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