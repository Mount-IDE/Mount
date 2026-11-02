import "./styles/launch-option.css"
import Check from "@common/Check.tsx";
import Input from "@common/Input.tsx";
import FSContext from "@common/FSContext.tsx";
import launchService from "@/src/services/launch_service.ts";
import List from "@common/List.tsx";

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
            {
                typ == "list" &&
                <List title={obj.title} variants={obj.typ.list_types ?? []} write={write_} value={val.toString()}/>
            }
            {
                typ == "path" &&
                <FSContext title={obj.title} typ={"file"} write={write_} placeholder={""} value={val.toString()}/>
            }
        </div>
    )
}
