import "./styles/parameter.css"


type Props = {
    param: IParameter
    get_default: () => boolean | null
    set: (val: string | boolean )=>void
}

export default function Parameter(props: Props) {

    const is_active = props.get_default();
    const typ = props.param.typ.length > 0 ? props.param.typ[0] : null;
    return (
        <div className={"project-parameter"}>
            {typ == "input" &&
                <InputParameter def={is_active} parameter={props.param} set_value={props.set}/>
            }
            {typ == "check" &&
                <CheckParameter def={is_active} parameter={props.param} set_value={props.set}/>
            }
            {typ == "list" &&
                <ListParameter def={is_active} parameter={props.param} set_value={props.set}/>
            }
            {typ == "file" &&
                <FileParameter def={is_active} parameter={props.param} set_value={props.set}/>
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
    def: boolean | null,
    set_value: (val: string | boolean) => void
}


function InputParameter(props: ParameterValue) {
    const typ = props.parameter.typ[1];
    const param = props.parameter;
    const label = param.label;
    const def_ = props.parameter.def;


    return (
        <div className={"project-parameter-value project-parameter-input"}>
            <p className={"project-parameter-input-p"}>{label[0]}</p>
            {typ == "base" &&
                <input placeholder={label[1]} defaultValue={typeof def_ == "string" ? def_ : ""}/>
            }
            {
                typ == "resize" &&
                <textarea placeholder={label[1]} defaultValue={typeof def_ == "string" ? def_ : ""}/>
            }
        </div>
    )
}

function CheckParameter(props: ParameterValue) {
    const param = props.parameter;
    const label = param.label;
    const def_ = props.parameter.def;
    return (
        <div className={"project-parameter-value project-parameter-check"}>
            <input type={"checkbox"} defaultChecked={typeof def_ == "boolean" ? def_ : false}/>
            <p>{label}</p>
        </div>
    )
}

function ListParameter(props: ParameterValue) {
    const param = props.parameter;
    const label = param.label;
    const def_ = props.parameter.def;
    const typ = props.parameter.typ.slice(1);
    return (
        <div className={"project-parameter-value project-parameter-list"}>
            <select>
                {typ.map((el, i) =>
                    <option key={i}>{el}</option>
                )}
            </select>
            <p>{label}</p>
        </div>
    )
}

function FileParameter(props: ParameterValue) {
    return (
        <div className={"project-parameter-value project-parameter-file"}>

        </div>
    )
}