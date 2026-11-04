import "./styles/create-entity.css"
import {useEffect, useMemo, useState} from "react";
import {cacheStore} from "@stores/cache_store.ts";
import {contextStore} from "@stores/context_store.ts";
import {menuStore} from "@stores/menu_store.ts";
import Input from "@common/Input.tsx";
import {motion} from "motion/react";
import Check from "@common/Check.tsx";
import FSContext from "@common/FSContext.tsx";
import List from "@common/List.tsx";
import Gen from "@common/Gen.tsx";
import {invoke} from "@tauri-apps/api/core";
import {projectStore} from "@stores/project_store.ts";


/**
 *
 * @constructor
 */
export default function CreateEntity() {
    const templates = cacheStore(state => state.file_templates)
    const [current_template, setCurrentTemplate] = useState<number | null>(null)

    const close_window = menuStore(state => state.close_file_create_menu)
    const path_to = contextStore(state => state.path_to_creation)
    const current = current_template != null ? templates[current_template] : null

    const [results, setResults] = useState<Record<string, Record<string, IVal>>>({})

    function write(val: IVal, id: string) {
        if (current?.id) {
            setResults(prev => {
                return {
                    ...prev,
                    [current.id]: {
                        ...prev[current.id],
                        [id]: val
                    }
                }
            })
        }
    }

    const meta = useMemo(() => {
        if (!current) {
            return []
        }
        let res: (false | ParProps)[] = [
            typeof current?.parameters?.default_name != "string" && {
                obj: {
                    id: "meta.name",
                    title: "Name",
                    typ: {
                        typ: "input",
                    }
                }, val: results[current!.id]?.["meta.name"] ?? "",
                write: (val: IVal) => write(val, "meta.name")
            },
            current?.parameters?.const_path != false && {
                obj: {
                    id: "meta.path",
                    title: "Path",
                    typ: {
                        typ: "file",
                        fs_type: "fs"
                    }
                },
                write: val => write(val, "meta.path"),
                val: results[current!.id]?.["meta.path"] ?? path_to
            }
        ]

        return res.filter(el => el != false)
    }, [current, path_to, results])


    useEffect(() => {
        if (current?.parameters?.default_name) {
            setResults(prev => {
                return {
                    ...prev,
                    [current.id]: {
                        ...prev[current.id],
                        "meta.name": current.parameters?.default_name ?? ""
                    }
                }
            })
        }
        if (current?.parameters?.const_path == true) {
            setResults(prev => {
                return {
                    ...prev,
                    [current.id]: {
                        ...prev[current.id],
                        "meta.path": path_to
                    }
                }
            })
        }
    }, [current?.parameters, path_to]);

    useEffect(() => {
        if (current?.id) {
            setResults(prev => {
                return {
                    ...prev,
                    [current.id]: {
                        ...prev[current.id],
                        "meta.path": path_to
                    }
                }
            })
        }
    }, [path_to, current]);



    async function create_fs() {
        if (current == null) {
            return;
        }
        try {
            await invoke("create_fs_entity", {
                results: results,
                template: current,
                vars: projectStore.getState().current_project!.vars
            })
            menuStore.getState().close_file_create_menu();
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <motion.div
            initial={{opacity: 0}}
            animate={{opacity: 1}}
            exit={{opacity: 0}}
            transition={{duration: 0.2}}

            id={"create-entity"}>
            <div id={"create-entity-header"}>
                <p id={"create-entity-path"}>{path_to}</p>
                <p id={"create-entity-label"}>Create file</p>
            </div>
            <div id={"create-entity-body"}>
                <div id={"create-entity-templates"}>
                    {templates.map((el, i) =>
                        <FileTemplate obj={el}
                                      key={el.id}
                                      selected={current_template != null && i == current_template}
                                      cb={() => setCurrentTemplate(prev => {
                                          if (prev == i) {
                                              return null
                                          }
                                          return i
                                      })}
                        />
                    )}
                </div>
                <div id={"create-entity-fields"}>
                    {
                        meta.map((el, i) =>
                            <EntityParameter {...el} key={i}/>
                        )
                    }
                    {
                        current?.options?.map((el, i) =>
                            <EntityParameter obj={el} write={(val) => write(val, el.id)} key={i}
                                             val={results[current?.id ?? "__none__"]?.[el.id] ?? el.def ?? ""}/>
                        )
                    }

                </div>
            </div>
            <div id={"create-entity-footer"}>
                <div>
                    <button className={"create-entity-button"}
                            onClick={close_window}
                    >Cancel
                    </button>
                    <button onClick={create_fs} className={"create-entity-button"}>Create</button>

                </div>
            </div>
        </motion.div>

    )
}


type TempProps = {
    obj: ConfigFsTemplate
    selected: boolean
    cb: () => void,
}

function FileTemplate({obj, selected, cb}: TempProps) {

    return (
        <div
            style={{
                borderBottom: selected ? "1px solid var(--border)" : "1px solid transparent"
            }}
            onClick={cb} className={"file-template"}>
            {obj.icon !== undefined &&
                <div className={"file-template-icon"}>
                    <img src={`/builtin/fs-icons/${obj.icon}`}/>
                </div>
            }
            <p>{obj.title}</p>
        </div>
    )
}



type ParProps = {
    obj: IPackageParameter
    write: (val: IVal) => void
    val: IVal
}

/**
 *
 * @param props
 * @constructor
 */
function EntityParameter(props: ParProps) {

    const {obj} = props
    const typ = obj.typ.typ
    return (
        <>
            {
                (typ == "input" || typ == "area") &&
                <Input typ={typ} value={props.val.toString()} title={obj.title} write={props.write}
                       placeholder={obj.typ.placeholder}/>
            }
            {
                typ == "check" &&
                <Check value={!!props.val} title={obj.title} write={props.write}/>
            }
            {
                typ == "file" &&
                <FSContext typ={obj.typ.fs_type == "file" ? "file" : "dir"} title={obj.title}
                           value={props.val.toString()} write={props.write} placeholder={obj.typ.placeholder}/>
            }
            {
                typ == "list" &&
                <List variants={obj.typ.list_type ?? []} title={obj.title} value={props.val.toString()}
                      write={props.write}/>
            }
            {
                typ == "gen" &&
                <Gen value={Array.isArray(props.val) ? props.val : []} title={obj.title} write={props.write}
                     required={obj.typ.required}/>
            }
        </>
    )
}

