import "./styles/parameter.css"
import dir from "../../../assets/dir.svg"
import {open} from "@tauri-apps/plugin-dialog"
import {ChangeEvent, InputEvent, useEffect} from "react";
import {cacheStore} from "../../../stores/cache_store.ts";
import {createProjectStore} from "../../../stores/create_project.ts";

type Props = {
    param: IParameter
    set: (val: string | boolean) => void,
    section: number,
    allParams: IParameter[]
    is_main:boolean
}

export default function Parameter(props: Props) {
    const typ = props.param.typ.length > 0 ? props.param.typ[0] : null;

    const {param, section} = props;

    const current_template = cacheStore(state => state.currentTemplate);


    let key = "__meta__";
    if (!props.is_main){
        key = current_template? current_template.id : "__garbage__"
    }

    const dependencyValue = createProjectStore(state =>
        current_template ? state.results[key]?.[section]?.[param.while_??""] : undefined
    );

    const dependency = param.while_
        ? props.allParams.find(el => el.out == param.while_)
        : null;

    const is_active=(() => {
        if (!param.while_) return null;
        if (!dependency) return null;
        if (dependencyValue === undefined) {
            return false;
        }
        if (typeof dependencyValue !== typeof dependency.def) {
            return false;
        }

        return dependencyValue !== dependency.def;
    })()

    const show = typeof is_active == "boolean" ? is_active : true;



    const value = createProjectStore(
            state =>
                state.results[key]?.[section]?.[param.out]
            );
    const new_def = value !== undefined ? value : props.param.def;

    useEffect(() => {
        props.set(props.param.def as string|boolean)
    }, [props.param.def]);


    const res = createProjectStore(state=>state.results)



    return (
        <div className={"project-parameter"}>
            {typ == "input" &&
                <InputParameter val={new_def} def={show} parameter={props.param} set_value={props.set}/>
            }
            {typ == "check" &&
                <CheckParameter val={new_def} def={show} parameter={props.param} set_value={props.set}/>
            }
            {typ == "list" &&
                <ListParameter val={new_def} def={show} parameter={props.param} set_value={props.set}/>
            }
            {typ == "file" &&
                <FileParameter val={new_def} def={show} parameter={props.param} set_value={props.set}/>
            }
            {
                typ == null &&
                <p>{JSON.stringify(props.param.def)}</p>
            }
        </div>
    )
}


interface ParameterValue {
    parameter: IParameter,
    def: boolean,
    set_value: (val: string | boolean) => void
    val: IVal
}


function InputParameter(props: ParameterValue) {
    const typ = props.parameter.typ[1];
    const param = props.parameter;
    const label = param.label;
    const def_ = props.val;

    function write(event: InputEvent<HTMLInputElement> | InputEvent<HTMLTextAreaElement>) {
        const tg = event.currentTarget.value;
        props.set_value(tg)
    }


    let classes = props.def ? "project-parameter-value project-parameter-input" :
        "project-parameter-value project-parameter-input project-parameter-value-disabled"


    return (
        <div className={classes}>
            <p className={"project-parameter-input-p"}>{label[0]}</p>
            {typ == "base" &&
                <input placeholder={label[1]} value={typeof def_ == "string" ? def_ : ""} onInput={write}/>
            }
            {
                typ == "resize" &&
                <textarea placeholder={label[1]} value={typeof def_ == "string" ? def_ : ""} onInput={write}/>
            }
        </div>
    )
}

function CheckParameter(props: ParameterValue) {
    const param = props.parameter;
    const label = param.label;
    const def_ = props.val;

    function write(event: ChangeEvent<HTMLInputElement>) {
        const tg = event.currentTarget.checked;
        props.set_value(tg)
    }


    let classes = props.def ? "project-parameter-value project-parameter-check" :
        "project-parameter-value project-parameter-check project-parameter-value-disabled"

    return (
        <div className={classes}>
            <input type={"checkbox"} checked={typeof def_ == "boolean" ? def_ : false} onChange={write}/>
            <p>{label}</p>
        </div>
    )
}

function ListParameter(props: ParameterValue) {
    const param = props.parameter;
    const label = param.label;
    const def_ = props.val;
    const typ = props.parameter.typ.slice(1);

    function write(event: ChangeEvent<HTMLSelectElement>) {
        const tg = event.currentTarget.value;
        props.set_value(tg)
    }


    let classes = props.def ? "project-parameter-value project-parameter-list" :
        "project-parameter-value project-parameter-list project-parameter-value-disabled"


    return (
        <div className={classes}>
            <select value={typeof def_ == "string" ? def_ : ""} onChange={write}>
                {typ.map((el, i) =>
                    <option key={i}>{el}</option>
                )}
            </select>
            <p>{label}</p>
        </div>
    )
}

function FileParameter(props: ParameterValue) {
    const typ = props.parameter.typ[1];
    const param = props.parameter;
    const label = param.label;
    const def_ = props.val;

    async function openDialog() {
        if (typ == "file") {
            const res = await open({
                directory: false,
                title: "Choose the file"
            })
            if (res !== null) {
                props.set_value(res)
            }
        } else {
            const res = await open({
                directory: true,
                title: "Choose the directory"
            })
            if (res !== null) {
                props.set_value(res)
            }
        }
    }

    function write(event: InputEvent<HTMLInputElement>) {
        const tg = event.currentTarget.value;

        props.set_value(tg)
    }

    let classes = props.def ? "project-parameter-value project-parameter-file" :
        "project-parameter-value project-parameter-file project-parameter-value-disabled"


    return (
        <div className={classes}>
            <p className={"project-parameter-input-p"}>{label[0]}</p>
            <div className={"project-parameter-file-in"}>
                <input placeholder={label[1]} value={typeof def_ == "string" ? def_ : ""} onInput={write}/>
                <button className={"project-parameter-file-in-bt"} onClick={openDialog}>
                    <img src={dir}/>
                </button>
            </div>

        </div>
    )
}