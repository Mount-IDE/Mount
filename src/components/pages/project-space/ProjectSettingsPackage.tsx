import "./styles/project-settings-package.css"
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import Inner, {Props as InnerProps} from "../../common/Inner.tsx";
import {useEffect, useMemo} from "react";


type Props = {
    pack: IPackage
}


export default function ProjectSettingsPackage(props: Props) {

    const results = projectSettingsStore(state => state.package_results[props.pack.id]) as IPackage | undefined

    const {pack} = props;

    const write = projectSettingsStore(state => state.write_pack)

    const pack_ = useMemo<InnerProps>(() => {
        return {
            title: `${pack.name} (${pack.id})`,
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
                                // value: pack?.meta?.description ?? "",
                                value: results?.meta?.description ?? pack.meta?.description ?? "",
                                setValue: (val) => write(pack.id, p => {
                                    console.log("CALl")
                                    p.meta = {
                                        ...p.meta,
                                        description: val.toString() ?? ""
                                    };
                                    return p
                                }, pack),
                                other_meta: {
                                    title: "Description"
                                }
                            },
                            {
                                typ: "list",
                                value: results?.meta?.license ?? pack.meta?.license ?? "",
                                other_meta: {
                                    title: "License",
                                    variants: ["MIT", "LGPL-3.0", "NonLicensed"],
                                    readonly: true
                                }
                            },
                            {
                                typ: "gen",
                                value: (Array.isArray(results?.meta?.authors) ? results?.meta?.authors : Array.isArray(pack.meta?.authors) ? pack.meta?.authors : []),
                                other_meta: {
                                    title: "Authors",
                                    readonly: true
                                }
                            },
                            {
                                typ: "gen",
                                value: (Array.isArray(results?.meta?.tags) ? results?.meta?.tags : Array.isArray(pack.meta?.tags) ? pack.meta?.tags : []),
                                other_meta: {
                                    title: "Tags",
                                    readonly: true
                                }
                            },
                            {
                                typ: "input",
                                value: results?.meta?.typ ?? pack.meta?.typ ?? "",
                                other_meta: {
                                    readonly: true,
                                    title: "Type",
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
                                value: results?.files.extentions ?? pack.files.extentions ?? [],
                                setValue: val => write(pack.id, p => {
                                    p.files.extentions = val as string[];
                                    return p
                                }, pack),
                                other_meta: {
                                    title: "File Extentions",
                                }
                            },
                            {
                                typ: "gen",
                                value: results?.files.files ?? pack.files.files ?? [],
                                setValue: val => write(pack.id, p => {
                                    p.files.files = val as string[]
                                    return p
                                }, pack),
                                other_meta: {
                                    title: "Files",
                                }
                            },
                            {
                                typ: "gen",
                                value: results?.files.ignore_files ?? pack.files.ignore_files ?? [],
                                setValue: val => write(pack.id, p => {
                                    p.files.ignore_files = val as string[]
                                    return p
                                }, pack),
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
                            (results ?? pack).highlight.map((el, i) => ({
                                typ: "inner",
                                meta: {
                                    title: `${el.id}`,
                                    can_show: true,
                                    show_default: false,
                                    api_children: [
                                        {
                                            typ: "input",
                                            value: el.lang,
                                            setValue: val => write(pack.id, p => {
                                                p.highlight[i].lang = val as string
                                                return p
                                            }, pack),
                                            other_meta: {
                                                readonly: true,
                                                title: "Language"
                                            }
                                        },
                                        {
                                            typ: "gen",
                                            value: el.extentions,
                                            setValue: val => write(pack.id, p => {
                                                p.highlight[i].extentions = val as string[]
                                                return p
                                            }, pack),
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

    }, [results, props.pack])


    useEffect(() => {
        console.log("RES", results)
    }, [results]);

    return (
        <>
            <Inner {...pack_}/>
        </>
    )
}



