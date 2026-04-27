import "./styles/project-meta.css"
import Section from "./Section.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
import {createProjectStore} from "../../../stores/create_project.ts";
import cross from "../../../assets/title-close.svg"
import plus from "../../../assets/plus.svg"


export default function ProjectMeta() {

    const project_path = cacheStore(state => state.projects_path);
    const base_meta: ISection[] = [
        {
            id: -4,
            label: "",
            list: [false, false],
            params: [
                {
                    def: "Untitled",
                    label: ["Project Name", "Enter a project name"],
                    out: "project-name",
                    typ: ["input", "base"],
                }, {
                    def: project_path,
                    label: ["Project Path", "Enter a project path"],
                    out: "project-path",
                    typ: ["file", "dir"],
                }
            ]
        }, {
            id: -3,
            label: "Addition Information",
            list: [true, true],
            params: [
                {
                    def: "",
                    label: ["Authors", "author1 author2"],
                    out: "project-authors",
                    typ: ["input", "base"],
                }, {
                    def: "",
                    label: ["Description", ""],
                    out: "project-description",
                    typ: ["input", "resize"],
                },
                {
                    def: "",
                    label: ["License", ""],
                    out: "project-license",
                    typ: ["list", "NonLicense", "LGPL", "APACHE"],
                },
                {
                    def: "",
                    label: ["Group", ""],
                    out: "project-group",
                    typ: ["list", "general"],
                },
            ]
        }, {
            id: -2,
            label: "Git Options",
            list: [true, false],
            params: [
                {
                    def: false,
                    label: "Create git repository",
                    out: "project-git",
                    typ: ["check"],
                }, {
                    def: false,
                    label: "Add .gitignore",
                    out: "project-git-gitignore",
                    typ: ["check"],
                    while_: "project-git"
                },
                {
                    def: "",
                    label: ["Git remote origin", ""],
                    out: "project-git-remote",
                    typ: ["input", "base"],
                    while_: "project-git"
                }
            ]
        }
    ]
    const template = cacheStore(state => state.currentTemplate);
    const other_sections = template ? template.startup.sections : []
    const tags = createProjectStore(state=>state.tags)
    const add_tag = createProjectStore(state=>state.add_tag)
    const remove_tag = createProjectStore(state=>state.remove_tag)
    const change_tag = createProjectStore(state=>state.change_tag)
    return (
        <div id={"create-project-meta"}>
            {base_meta.map(
                (el, i) =>
                    <Section is_main section={el} key={`base-${i}`}/>
            )}
            {other_sections.map((el, i) =>
                <Section section={el} key={i}/>
            )}

            <div id={"create-project-tags"}>
                <div id={"create-project-tags-list"}>
                    {tags.map((el, i) =>
                        <div className={"create-project-tag"} key={i}>
                            <input value={el.name}
                                   onInput={(e)=>change_tag(el.id, e.currentTarget.value)} />
                            <button
                                onClick={()=>remove_tag(el.id)}
                                className={"create-project-tag-close"}>
                                <img src={cross}/>
                            </button>
                        </div>
                    )}
                </div>
                <button
                    onClick={()=>add_tag("general")}
                    id={"create-project-tags-add"}>
                    <img src={plus}/>
                </button>
            </div>
        </div>
    )
}