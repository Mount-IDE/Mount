import "./styles/project-settings-variables.css"
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import cross from "../../../assets/title-close.svg"
import plus from "../../../assets/plus.svg"
import {useEffect, useRef, useState} from "react";

export default function ProjectSettingsVariables() {


    const variables = projectSettingsStore(state => state.variables)
    const changeName = projectSettingsStore(state => state.write_var_name);
    const changeVal = projectSettingsStore(state => state.write_var_val);
    const [show, setShow] = useState(false)
    const ref = useRef<HTMLDivElement>(null)
    const ref2 = useRef<HTMLDivElement>(null)

    const addVar_ = projectSettingsStore(state => state.add_variable)
    const remVar = projectSettingsStore(state => state.rem_variable)
    useEffect(() => {
        let cur = ref.current;

        let bt = ref2.current


        function handler(e: MouseEvent) {
            if (!e.target) {
                return
            }
            if (!cur) {
                return
            }
            if (!bt) {
                return
            }
            let tg = e.target as Element
            console.log(tg, cur, cur.contains(tg))
            if (!(cur.contains(tg) || tg == cur) && !bt.contains(tg)) {
                setShow(false)
            }
        }

        window.addEventListener("click", handler)


        return () => {
            window.removeEventListener("click", handler)
        }
    }, [show])


    function addVar(type: "string" | "number" | "boolean") {
        setShow(false)
        addVar_(type)
    }

    return (
        <div id={"project-settings-var"}>
            <div id={"var-header"}>
                <div id={"var-header-label"}>
                    <p>№</p>
                    <p style={{
                        width: "38%"
                    }}>Name</p>
                    <p>Value</p>
                </div>
                <div ref={ref2} id={"var-bt"} onClick={() => setShow(prev => !prev)}>
                    <img src={plus}/>
                </div>
                {
                    show &&
                    <div ref={ref} id={"var-bt-variants"}>
                        <div onClick={() => addVar("number")}>Number</div>
                        <div onClick={() => addVar("string")}>String</div>
                        <div onClick={() => addVar("boolean")}>Bool</div>
                    </div>
                }
            </div>
            <div id={"var-body"}>
                {
                    variables.map((el, i) => (
                        <div className={"variable"} key={i}>
                            <span>{i + 1}</span>
                            <input className={"var-name"} value={el.name}
                                   onInput={
                                       e => changeName(i, e.currentTarget.value)}/>
                            {
                                typeof el.value == "string" &&
                                <input className={"var-val"}
                                       value={el.value}
                                       onInput={e => changeVal(i, e.currentTarget.value)}
                                />
                            }
                            {
                                typeof el.value == "boolean" &&
                                <input
                                    className={"var-val"}
                                    type={"checkbox"}
                                    checked={el.value}
                                    onChange={e =>
                                        changeVal(i, e.currentTarget.checked)}
                                />
                            }
                            {
                                typeof el.value == "number" &&
                                <input
                                    className={"var-val"}
                                    type={"number"}
                                    value={el.value}
                                    onInput={e => changeVal(i, e.currentTarget.value)}
                                />
                            }

                            <div className={"var-delete"}
                                 onClick={() => remVar(i)}
                            >
                                <img src={cross}/>
                            </div>
                        </div>
                    ))
                }
            </div>
        </div>
    )
}