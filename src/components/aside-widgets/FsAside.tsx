import "./styles/fs-aside.css"
import {fsAsideTreeStore} from "../../stores/fs_aside_tree_store.ts";
import {useEffect} from "react";
import {projectStore} from "../../stores/project_store.ts";
import DirectoryX from "./DirectoryX.tsx";


export default function FsAside() {

    const tree = fsAsideTreeStore(state => state.tree);
    const load_tree = fsAsideTreeStore(state => state.load_tree);
    const watch = fsAsideTreeStore(state => state.watch);
    const unwatch = fsAsideTreeStore(state => state.unwatch);
    const cwd = projectStore(state => state.path_to_current_project);

    useEffect(() => {
        if (cwd.length === 0) {
            return;
        }

        load_tree(cwd);
        watch(cwd);

        return () => {
            void unwatch();
        }
    }, [cwd, load_tree, watch, unwatch]);

    return (
        <div className={"aside-in"}>
            {tree != null && <DirectoryX obj={tree}/>}
        </div>
    )
}
