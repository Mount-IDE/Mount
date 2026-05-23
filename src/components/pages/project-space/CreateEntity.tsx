import "./styles/create-entity.css"
import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {cacheStore} from "../../../stores/cache_store.ts";
import {contextStore} from "../../../stores/context_store.ts";
import {menuStore} from "../../../stores/menu_store.ts";


type Props = {}

type FieldValue = {
    [key: string]: string
}

export default function CreateEntity(_: Props) {
    const templates = cacheStore(state => state.file_templates)
    const [current_template, setCurrentTemplate] = useState<number | null>(null)

    const close_window = menuStore(state => state.close_file_create_menu)
    const path_to = contextStore(state => state.path_to_creation)
    const [fields, setFields] = useState<FieldValue>({})
    const current = current_template != null ? templates[current_template] : null

    useEffect(() => {
        setFields(prev => {
            const copy = {...prev}
            if (current?.base_name !== undefined) {
                copy["name"] = current.base_name
            }
            if (current?.ext !== undefined) {
                copy["ext"] = current.ext
            }
            if (current?.default_content !== undefined) {
                copy["content"] = current.default_content
            }

            return copy
        })
    }, [current]);

    async function create_fs() {
        if (current == null) {
            console.warn("0")
            return;
        }
        if ("name" in Object.keys(fields)) {
            console.warn("1")
            return;
        }
        const name = fields["name"];
        const ext = fields["ext"];

        try {
            const os = await invoke<string>("get_os");
            const sep = os == "windows" ? "\\" : "/";
            let extension = ext.length > 0 ? ext : ""
            extension = extension.length > 0 && extension[0] == "." ? extension.slice(1) : extension;
            const path_ = `${path_to}${sep}${name}.${extension}`;
            if (current!.typ == "file") {
                if (current.default_content !== undefined) {
                    await invoke(
                        "create_file", {
                            path: path_,
                            content: current.default_content!
                        })
                } else {
                    await invoke("create_file", {path: path_})
                }

            } else {
                await invoke("create_dir", {path: path_})
            }
        } catch
            (e) {
            console.error(e)
        } finally {
            close_window()
        }
    }


    useEffect(() => {
        console.log(current)
    }, [current]);

    useEffect(() => {
        console.log("fields", fields, current)
    }, [fields]);


    return (
        <>
            <div id={"create-entity"}>
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
                        {current != null &&
                            <>
                                <Field id={"name"} val={fields["name"] ?? ""}
                                       cb={(val: string) => setFields(prev => {
                                           const copy = {...prev};
                                           copy["name"] = val;
                                           return copy
                                       })} placeholder={"Name"}/>

                                <Field id={"ext"} val={fields["ext"] ?? ""} cb={(val: string) => setFields(prev => {
                                    const copy = {...prev};
                                    copy["ext"] = val;
                                    return copy
                                })} placeholder={"Extension"}/>
                                {
                                    current.default_content !== undefined &&
                                    <Field area id={"content"} val={fields["content"]}
                                           cb={(val: string) => setFields(prev => {
                                               const copy = {...prev}
                                               copy["content"] = val;
                                               return copy
                                           })}/>
                                }
                            </>
                        }
                        {
                            current == null &&

                            <p style={{
                                color: "var(--subtitle)",
                                width: "50%",
                                height: "100%",
                                display: "flex",
                                flexDirection: "column",
                                alignItems: "center",
                                justifyContent: "center"
                            }}>Nothing to show</p>
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
            </div>
            <div style={{
                position: "fixed",
                top: 0,
                left: 0,
                width: "100vw",
                height: "100vh",
                backdropFilter: "blur(5px)",
                background: "rgba(0,0,0,0.5)",
                zIndex: 5,
            }}></div>
        </>
    )
}


type TempProps = {
    obj: configFsTemplate
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


type FieldProps = {
    id: string
    val: string,
    cb: (val: string) => void
    placeholder?: string;
    area?: boolean
}

function Field(props: FieldProps) {


    const ref = useRef<HTMLTextAreaElement>(null)

    useEffect(() => {
        const cur = ref.current;
        if (!cur) return;
        cur.style.height=`${cur.scrollHeight}px`
    }, [props.val, props.area]);

    // useEffect(() => {
    //     props.cb(props.val)
    // }, []);
    return (
        <div className={"create-entity-field"}>
            {props.area &&
                <textarea ref={ref} placeholder={props.placeholder} value={props.val}
                          onInput={(e) => props.cb(e.currentTarget.value)}/>
            }
            {!props.area &&
                <input placeholder={props.placeholder} value={props.val}
                       onInput={(e) => props.cb(e.currentTarget.value)}/>
            }

        </div>
    )
}