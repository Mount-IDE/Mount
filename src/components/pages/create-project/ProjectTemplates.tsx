import "./styles/project-templates.css"
import {cacheStore} from "../../../stores/cache_store.ts";
import Template from "./Template.tsx";
import settings from "../../../assets/settings.svg"
import Icon from "../../common/Icon.tsx";


export default function ProjectTemplates() {
    const templates:ITemplate[] = cacheStore(state=>state.templates);
    const current = cacheStore(state=>state.currentTemplate);
    const setCurrent = cacheStore(state=>state.set_current_template);
    return (
        <div id={"create-project-templates"}>
            <div className={"flex w-full justify-between items-center"}>
                <p id={"create-project-templates-p"}>Templates</p>
                <Icon src={settings} size={"65%"}/>
            </div>
            <div id={"create-project-templates-list"}>
                {templates.map((el, i)=>
                <Template current={el.id==current?.id} template={el} cb={()=>{
                    setCurrent(el)
                }} key={`${el.id}-${i}`}/>
                )}
            </div>
            {/*<button id={"create-project-templates-manage"}>Manage Templates</button>*/}
        </div>
    )
}