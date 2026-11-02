import "./styles/launch-option.css"
import dir from "@assets/dir.svg"
import {open} from "@tauri-apps/plugin-dialog";
import Check from "@common/Check.tsx";
import Input from "@common/Input.tsx";
import FSContext from "@common/FSContext.tsx";
import launchService from "@/src/services/launch_service.ts";

type Props = {
    obj: LaunchOption,
    section: number,
    functions: LaunchFunction[]
    cur_ref: number
    project: IProject | null

}


export default function LaunchOption(props: Props) {
    const {obj} = props;
    const typ = obj.typ.typ;

    let find = () => launchService.find_result(
        props.cur_ref,
        props.section,
        obj.id)
    let val = find() ?? props.obj.def

    // const [val, setVal] = useState(() => find() ?? props.obj.def.toString());

    /* useEffect(() => {
         let found = find()
         console.log("FOUND", found, launchStore.getState().copy_references)
         if (found) {
             setVal(found)
         }
     }, []);*/

    function write_(val_: IVal) {
        launchService.write_ref_result(props.cur_ref, props.section, obj.id, val_)
    }

    return (
        <div className={"launch-option"}>
            {
                typ == "check" &&
                <Check title={obj.title} write={write_} value={!!val}/>
            }
            {
                typ == "input" &&
                <Input title={obj.title} placeholder={""} typ={"input"} write={write_} value={val.toString()}/>
            }
            {/* {
                typ == "list" &&
                <List title={obj.title} variants={} write={write_} value={val}/>
            }*/}
            {
                typ == "path" &&
                <FSContext title={obj.title} typ={"file"} write={write_} placeholder={""} value={val.toString()}/>
            }
        </div>
    )
}

type ChildProps = {
    write: (value: string) => void,
    val: string,
    obj: LaunchOption,
}

function LaunchOptionCheck(props: ChildProps) {
    return (
        <div className={"launch-option-child check"}>
            <input type={"checkbox"} checked={props.val == "1"}
                   onChange={(e) =>
                       props.write(e.currentTarget.checked ? "1" : "0")
                   }
            />
            <p>{props.obj.title}</p>
        </div>
    )
}

function LaunchOptionInput(props: ChildProps) {
    return (
        <div className={"launch-option-child input"}>
            <p>{props.obj.title}</p>
            <input value={props.val}
                   onInput={(e) =>
                       props.write(e.currentTarget.value)
                   }
            />
        </div>
    )
}

function LaunchOptionList(props: ChildProps) {
    return (
        <div className={"launch-option-child list"}>
            {
                props.obj.typ.list_types &&
                <select
                    onChange={(e) => props.write(e.currentTarget.value)}
                >
                    {
                        props.obj.typ.list_types!.map((el, i) =>
                            <option
                                key={i}
                                value={el}
                                selected={el == props.val}>
                                {el}
                            </option>
                        )
                    }
                </select>
            }
            <p>{props.obj.title}</p>
        </div>
    )
}

function LaunchOptionPath(props: ChildProps) {

    const typ = props.obj.typ.path_type ?? "file";

    async function read_file() {
        if (typ == "file") {
            const res = await open({
                directory: false,
                title: "Choose the file"
            })
            if (res !== null) {
                props.write(res)
            }
        } else {
            const res = await open({
                directory: true,
                title: "Choose the directory"
            })
            if (res !== null) {
                props.write(res)
            }
        }
    }

    return (
        <div className={"launch-option-child path"}>
            <p>{props.obj.title}</p>
            <div>
                <input value={props.val}
                       onInput={(e) => {
                           props.write(e.currentTarget.value)
                       }
                       }
                />
                <button
                    onClick={read_file}
                ><img src={dir}/></button>
            </div>
        </div>
    )
}