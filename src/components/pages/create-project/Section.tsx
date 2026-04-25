import "./styles/section.css"
import {useCallback, useEffect, useState} from "react";
import arrow from "../../../assets/arrow.svg"
import Parameter from "./Parameter.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";

type Props = {
    section: ISection
}


export default function Section(props: Props) {

    const [isOpened, setIsOpened] = useState<boolean>(false);
    const [isList, _] = useState(props.section.list[0]);

    const current_template = cacheStore.getState().currentTemplate!;
    const get_result = createProjectStore.getState().get_result;
    const has_result = createProjectStore.getState().has_result;
    const set_value = createProjectStore.getState().add_result;
    useEffect(() => {
        const is_opened = props.section.list[1];
        if (!isList) {
            setIsOpened(true)
        } else {
            setIsOpened(is_opened);
        }
    }, [props.section]);


    const fn = useCallback<(el: IParameter)=>boolean|null>((el: IParameter)=>{
        if (el.while_ ===undefined){
            return null
        }
        const result=`${current_template.id}:${props.section.id}:${el.out}`
        const res= has_result(result);
        if (!res){
            return true;
        }
        const val = get_result(result)!;
        const def = props.section.params.find(el=>el.out==el.while_);
        if (def!==undefined){
            const val2 =def.def;
            if (typeof val != typeof val2) return false
            return val==val2
        }

        return null
    },[props.section.params, current_template, props.section.id, has_result, get_result])


    return (
        <div className={"project-section"}>
            <div className={"project-section-head"}
                onClick={()=>{
                    if (isList) {
                        setIsOpened(prev=>!prev)
                    }
                }}
            >
                {isList&& <div
                    style={{
                        transform: isOpened? "rotate(0deg)": "rotate(-90deg)"
                    }}
                    className={"project-section-head-arrow"}>
                    <svg width="19" height="10" viewBox="0 0 19 10" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M0.421718 0.412777C-0.140573 0.956418 -0.140573 1.83792 0.421718 2.38154L7.46557 9.18545C8.59035 10.2719 10.4129 10.2714 11.5371 9.18461L18.5782 2.37653C19.1406 1.8329 19.1406 0.951407 18.5782 0.407752C18.016 -0.135917 17.1043 -0.135917 16.542 0.407752L10.5155 6.23469C9.95328 6.77845 9.04159 6.77831 8.47934 6.23469L2.45792 0.412777C1.89564 -0.130892 0.983995 -0.130892 0.421718 0.412777Z" fill="#A4A4A4"/>
                    </svg>

                </div>}
                <p
                    style={{
                        color: isOpened? "var(--title)":"var(--subtitle)"
                    }}
                    className={"project-section-head-p"}>{props.section.label}</p>
            </div>
            <div className={isOpened ? "project-section-in-open project-section-in" : "project-section-in"}>                {props.section.params.map(el=>
                    <Parameter
                        set={(val)=>{
                            const from = `${current_template.id}:${props.section.id}:${el.out}`
                            set_value(from, val);
                        }}
                        param={el}
                        key={`${props.section.id}${el.out}`}
                        get_default={()=>fn(el)}
                    />)}
            </div>
        </div>
    )
}