import "./styles/package.css"
import {ChangeEvent} from "react";


type Props={
    package_: IPackage
    set: (val: boolean)=>void
    def_: boolean;
}


export default function Package(props: Props){

    function toggle(e: ChangeEvent<HTMLInputElement>) {
        const val = e.currentTarget.checked;
        props.set(val);
    }

    return (
        <div className={"project-package"}>
            <input type={"checkbox"}
                checked={
                props.def_
            }
                   onChange={toggle}
            />
            <p>{props.package_.name} ({props.package_.id})</p>
        </div>
    )
}