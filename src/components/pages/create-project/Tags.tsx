import "./styles/tags.css"
import {createProjectStore} from "../../../stores/create_project.ts";
import cross from "../../../assets/title-close.svg";
import plus from "../../../assets/plus.svg";


export default function Tags() {
    const tags = createProjectStore(state => state.tags)
    const add_tag = createProjectStore(state => state.add_tag)
    const remove_tag = createProjectStore(state => state.remove_tag)
    const change_tag = createProjectStore(state => state.change_tag)

    return (<div id={"create-project-tags"}>
            <div id={"create-project-tags-list"}>
                {tags.map((el, i) =>
                    <div className={"create-project-tag"} key={i}>
                        <input value={el.name}
                               onInput={(e) => change_tag(el.id, e.currentTarget.value)}/>
                        <button
                            onClick={() => remove_tag(el.id)}
                            className={"create-project-tag-close"}>
                            <img src={cross}/>
                        </button>
                    </div>
                )}
            </div>
            <button
                onClick={() => add_tag("general")}
                id={"create-project-tags-add"}>
                <img src={plus}/>
            </button>
        </div>

    )
}