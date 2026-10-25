import "./styles/footer.css"
import {computeBP, themeStore} from "../../../stores/theme_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import {Fragment, useEffect, useState} from "react";
import {cacheStore} from "../../../stores/cache_store.ts";
import {fsAsideTreeStore} from "../../../stores/fs_aside_tree_store.ts";
import {fsExtStore} from "../../../stores/fs_ext_store.ts";
import Icon from "../../common/Icon.tsx";
import arrow from "../../../assets/arrow.svg"

export default function Footer() {

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.footer)

    const codespace_num = codeSpaceStore(state => state.current);
    const codespace = codeSpaceStore(state => state.get_space(codespace_num))
    let file = codespace?.opened_files?.[codespace?.current_file ?? 0]
    let pack = projectStore(state => state.get_pack_by_file(file?.name ?? null))


    let currentFs = fsAsideTreeStore(state => state.current)

    const [filePath, setFilePath] = useState<string[] | null>(null)

    const [icon, setIcon] = useState<string | null>(null)

    const project = projectStore(state => state.current_project);


    useEffect(() => {
        console.log(currentFs)
        if (!currentFs) {
            setFilePath(null)
            return
        }

        let path = currentFs.path;
        if (path.startsWith(project?.path ?? "")) {
            path = path.replace(project?.path ?? "", "")
        }

        let os = cacheStore.getState().os;
        let parts = [] as string[]
        if (os == "windows") {
            parts = path.split("\\")
        } else {
            parts = path.split("/")
        }
        parts = parts.filter(el => el.length > 0)
        if (parts.length == 0) {
            setFilePath(null)
            return
        }

        if (currentFs.type == "file") {
            let found = fsExtStore.getState().get_file_by_name(parts[parts.length - 1])
            if (found[0]) {
                setIcon(found[1])
            } else {
                setIcon(null)
            }
        }
        console.log(parts)
        setFilePath(parts)
    }, [currentFs])

    return (
        <div id={"project-footer"}
             style={{
                 background: theme?.background,
                 borderRadius: theme?.rounded,
                 ...computeBP(theme?.border, "border"),
                 ...computeBP(theme?.padding, "padding"),
             }}
        >
            <div id={"project-footer-path"}>
                {
                    filePath?.map((el, i) => {
                        if (currentFs?.type == "file" && i + 1 == filePath?.length) {
                            return (
                                <Fragment key={i}>
                                    <Icon src={arrow} rotate={"-90deg"} size={"50%"}/>
                                    <div className={"footer-path-part"}>
                                        {
                                            (icon?.length ?? 0) > 0 &&
                                            <Icon src={`/builtin/fs-icons/${icon}`} size={"100%"} modifier={"height"}/>
                                        }
                                        {el}
                                    </div>
                                </Fragment>

                            )
                        }
                        return (
                            <Fragment key={i}>
                                {
                                    i > 0 &&
                                    <Icon src={arrow} rotate={"-90deg"} size={"50%"}/>
                                }
                                <div className={"footer-path-part"}>
                                    {el}
                                </div>
                            </Fragment>
                        )
                    })
                }
            </div>
            <div id={"project-footer-pack"}>
                {
                    pack &&
                    <p>
                        package: {pack.main.id}
                    </p>
                }
            </div>
        </div>
    )
}