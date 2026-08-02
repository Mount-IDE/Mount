import "./styles/project-meta.css"
import Section from "./Section.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
import Tags from "./Tags.tsx";
import {settingsStore} from "../../../stores/settings_store.ts";


export default function ProjectMeta() {

    const project_path = cacheStore(state => state.projects_path);
    const settings = settingsStore(state => state.settings)
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
                    def: "general",
                    label: ["Group", ""],
                    out: "project-group",
                    typ: ["list", ...[...settings?.general.project_groups ?? "general"]],
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

    return (
        <div id={"create-project-meta"}>
            {base_meta.map(
                (el, i) =>
                    <Section is_main section={el} key={`base-${i}`}/>
            )}
            {other_sections.map((el, i) =>
                <Section section={el} key={i}/>
            )}
            <Tags/>
        </div>
    )
}