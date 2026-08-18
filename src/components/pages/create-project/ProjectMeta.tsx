import "./styles/project-meta.css"
import Section from "./Section.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
import Tags from "./Tags.tsx";
import {settingsStore} from "../../../stores/settings_store.ts";

import img_ from "../../../assets/img.svg"
import close_ from "../../../assets/title-close.svg"
import {useEffect, useState} from "react";

import {SketchPicker} from 'react-color';
import {open} from "@tauri-apps/plugin-dialog";
import {ToggleGroup} from "radix-ui"
import {motion} from "motion/react";
import {invoke} from "@tauri-apps/api/core";
import {createProjectStore} from "../../../stores/create_project.ts";
import PackageSection from "./PackageSection.tsx";

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
                    title: "Project Name",
                    id: "project-name",
                    typ: {
                        typ: "input",
                        placeholder: "Enter a project name",
                        required: true
                    },
                }, {
                    def: project_path,
                    title: "Project Path",
                    id: "project-path",
                    typ: {
                        typ: "file",
                        fs_type: "dir",
                        placeholder: "Enter a project path",
                        required: true
                    },
                }
            ]
        }, {
            id: -3,
            label: "Addition Information",
            list: [true, true],
            params: [
                {
                    def: "",
                    title: "Authors",
                    id: "project-authors",
                    typ: {
                        typ: "input",
                        placeholder: "author1 author2"
                    },
                }, {
                    def: "",
                    title: "Description",
                    id: "project-description",
                    typ: {
                        typ: "area"
                    },
                },
                {
                    def: "",
                    title: "License",
                    id: "project-license",
                    typ: {
                        typ: "list",
                        list_type: ["NonLicense", "LGPL", "APACHE"]
                    },
                },
                {
                    def: "general",
                    title: "Group",
                    id: "project-group",
                    typ: {
                        typ: "list",
                        list_type: [...settings?.general.project_groups ?? "general"]
                    },
                },
            ]
        }, {
            id: -2,
            label: "Git Options",
            list: [true, false],
            params: [
                {
                    def: false,
                    title: "Create git repository",
                    id: "project-git",
                    typ: {
                        typ: "check"
                    },
                }, {
                    def: false,
                    title: "Add .gitignore",
                    id: "project-git-gitignore",
                    typ: {
                        typ: "check"
                    },
                    while_: "project-git"
                },
                {
                    def: "",
                    title: "Git remote origin",
                    id: "project-git-remote",
                    typ: {
                        typ: "input"
                    },
                    while_: "project-git"
                }
            ]
        }
    ]
    const template = cacheStore(state => state.currentTemplate);
    const other_sections = template ? template.startup.sections : []

    const [imageShow, setImageShow] = useState(false)
    const [color, setColor] = useState("")

    const [typ, setTyp] = useState("color")
    const [image, setImage] = useState<string | null>(null)

    useEffect(() => {
        createProjectStore.getState().add_result(
            "__meta__", -3, "image", JSON.stringify({
                typ, image, color
            })
        )
    }, [typ, image, color]);


    return (
        <div id={"create-project-meta"}>
            <div id={"create-project-img"}>

                {
                    !imageShow &&
                    <div onClick={() => setImageShow(true)}
                         id={"img"}
                         style={{
                             background: typ === "color" ? `${color}` : "#fff"
                         }}
                    >
                        {(typ == "color" || image == null) && <img src={img_}/>}
                        {typ == "image" && image != null && <img src={image}/>}
                    </div>
                }
                {
                    imageShow &&
                    <div id={"img-click"}>
                        <div id={"img-click-close"}
                             onClick={() => setImageShow(false)}
                        >
                            <img src={close_}/>
                        </div>
                        <ToggleGroup.Root
                            id={"image-click-toggle"}
                            type={"single"}
                            value={typ}
                            onValueChange={(e) => {
                                if (e == "color" || e == "image")
                                    setTyp(e)
                            }}
                        >
                            <ToggleGroup.Item value={"color"}>
                                <div
                                    className={"toggle-elem"}
                                    style={{
                                        color: typ == "color" ? "#000" : "var(--title)",
                                        background: typ == "color" ? "var(--grad)" : "transparent"
                                    }}
                                >
                                    color
                                </div>
                            </ToggleGroup.Item>
                            <ToggleGroup.Item value={"image"}>
                                <div className={"toggle-elem"}
                                     style={{
                                         color: typ == "image" ? "#000" : "var(--title)",
                                         background: typ == "image" ? "var(--grad)" : "transparent"
                                     }}
                                >image
                                </div>
                            </ToggleGroup.Item>

                        </ToggleGroup.Root>
                        <div id={"img-click-variants"}
                        >
                            {typ == "color" &&
                                <motion.div
                                    initial={{
                                        opacity: 0
                                    }}
                                    animate={{
                                        opacity: 1
                                    }}
                                    exit={{
                                        opacity: 0
                                    }}
                                    transition={{
                                        duration: 0.5
                                    }}
                                >
                                    <SketchPicker
                                        color={color}
                                        onChange={(e) => {
                                            console.log("changed")
                                            setColor(e.hex)
                                        }}
                                    />
                                </motion.div>

                            }
                            {
                                typ == "image" &&
                                <motion.div
                                    initial={{
                                        opacity: 0
                                    }}
                                    animate={{
                                        opacity: 1
                                    }}
                                    exit={{
                                        opacity: 0
                                    }}
                                    transition={{
                                        duration: 0.5
                                    }}
                                    id={"img-click-photo"}
                                    onClick={async () => {
                                        let res = await open({
                                            directory: false,
                                            title: "",
                                            filters: [{
                                                name: "image",
                                                extensions: ["png", "svg", "jpeg", "jpg", "gif"]
                                            }]
                                        })
                                        if (res != null) {
                                            let res2 = await invoke<string>("make_base64", {src: res})
                                            setImage(res2)
                                        } else {
                                            setImage(null)
                                        }
                                    }}
                                >
                                    {!image && <img src={img_} id={"image-logo"}/>}
                                    {image &&
                                        <img src={image} id={"image-back"}/>
                                    }
                                </motion.div>
                            }
                        </div>
                    </div>
                }
            </div>
            {base_meta.map(
                (el, i) =>
                    <Section is_main section={el} key={`base-${i}`}/>
            )}
            <PackageSection/>
            {other_sections.map((el, i) =>
                <Section section={el} key={i}/>
            )}
            <Tags/>
        </div>
    )
}