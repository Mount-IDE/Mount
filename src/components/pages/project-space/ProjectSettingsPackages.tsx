import "./styles/project-settings-packages.css"
import {Fragment, useMemo} from "react";
import Inner, {Props as InnerProps} from "../../common/Inner.tsx"
import {projectStore} from "../../../stores/project_store.ts";

function usePackages(packages: Map<string, PackageInner>) {
    let packs = [...packages.entries()]
    let res = useMemo<InnerProps[]>(() => {
        return packs.map<InnerProps>(el => {
            let pack = el[1].main
            return {
                title: `${el[1].main.name} (${el[0]})`,
                can_show: true,
                show_default: false,
                api_children: [ // id
                    {
                        typ: "input",
                        value: pack?.id ?? "",
                        other_meta: {
                            title: "id",
                            readonly: true
                        }
                    },
                    {
                        typ: "inner",
                        meta: {
                            title: "Meta",
                            can_show: true,
                            show_default: false,
                            api_children: [
                                {

                                    typ: "area",
                                    value: pack?.meta?.description ?? "",
                                    other_meta: {
                                        title: "Description"
                                    }
                                },
                                {
                                    typ: "list",
                                    value: pack?.meta?.license ?? "",
                                    other_meta: {
                                        title: "License",
                                        variants: ["MIT", "LGPL-3.0", "NonLicensed"]
                                    }
                                },
                                {
                                    typ: "gen",
                                    value: Array.isArray(pack.meta?.authors) ? pack?.meta?.authors : [],
                                    other_meta: {
                                        title: "Authors"
                                    }
                                },
                                {
                                    typ: "gen",
                                    value: Array.isArray(pack?.meta?.tags) ? pack?.meta?.tags : [],
                                    other_meta: {
                                        title: "Tags"
                                    }
                                },
                                {
                                    typ: "input",
                                    value: pack?.meta?.typ ?? "",
                                    other_meta: {
                                        readonly: true,
                                        title: "Type"
                                    }
                                }
                            ]
                        }
                    }, // meta
                    {
                        typ: "inner",
                        meta: {
                            title: "Files",
                            can_show: true,
                            show_default: false,
                            api_children: [
                                {
                                    typ: "gen",
                                    value: pack.files.extentions,
                                    other_meta: {
                                        title: "File Extentions",
                                        readonly: true
                                    }
                                },
                                {
                                    typ: "gen",
                                    value: pack.files.files,
                                    other_meta: {
                                        title: "Files",
                                        readonly: true
                                    }
                                },
                                {
                                    typ: "gen",
                                    value: pack.files.ignore_files,
                                    other_meta: {
                                        title: "Ignored Files",
                                        readonly: true
                                    }
                                }
                            ]
                        }
                    }, // files
                    {
                        typ: "inner",
                        meta: {
                            title: "Highlights",
                            can_show: true,
                            show_default: false,
                            api_children: (
                                pack.highlight.map((el) => ({
                                    typ: "inner",
                                    meta: {
                                        title: `${el.id}`,
                                        can_show: true,
                                        show_default: false,
                                        api_children: [
                                            {
                                                typ: "input",
                                                value: el.lang,
                                                other_meta: {
                                                    readonly: true,
                                                    title: "Language"
                                                }
                                            },
                                            {
                                                typ: "gen",
                                                value: el.extentions,
                                                other_meta: {
                                                    title: "File Extentions",
                                                    readonly: true
                                                }
                                            }
                                        ]
                                    }


                                }))
                            )

                        }
                    }

                ]

            }
        })
    }, [packages])
    return res
}


export default function ProjectSettingsPackages() {

    const packages = projectStore(state => state.selected_packages)

    const widgets = usePackages(packages)

    return (
        <div id={"project-settings-packages"}>
            {
                widgets.map((el, i) =>
                    <Fragment key={i}>
                        <Inner {...el} />
                        <hr style={{
                            border: "1px solid var(--border2)",
                            marginBottom: "20px",
                            width: "90%",
                            marginLeft: "5%"
                        }}/>
                    </Fragment>
                )
            }
        </div>
    )
}