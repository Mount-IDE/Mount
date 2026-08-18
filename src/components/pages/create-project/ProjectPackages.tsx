import "./styles/project-packages.css"
import {cacheStore} from "../../../stores/cache_store.ts";
import {createProjectStore} from "../../../stores/create_project.ts";
import {useEffect} from "react";
import Package from "./Package.tsx";
import {packageStore} from "../../../stores/package_store.ts";




export default function ProjectPackages(){

    const current_template = cacheStore(state=>state.currentTemplate);
    const packages = packageStore(state => state.packages);
    const res_packages = createProjectStore(state=>state.packages);
    const add_package= createProjectStore(state=>state.add_package)
    const add_packages= createProjectStore(state=>state.add_packages)
    const rem_package= createProjectStore(state=>state.remove_package)


    useEffect(() => {
        const default_packages = current_template?.packages_id??[];
        const packs = packages.filter(el=>default_packages.includes(el.id))
        add_packages(packs.map(el => el.id));
    }, [current_template]);

    const togglePackage = (el: IPackage, val: boolean) => {
        val ? add_package(el.id) : rem_package(el.id);
    };


    return (
    <div id={"create-project-packages"}>
        <p id={"create-project-packages-p"}>Packages</p>
        <div id={"create-project-packages-list"}>
            {packages.map((el, i)=>
                <Package key={`${el.id}-${i}`}
                         package_={el}
                         set={(val)=>togglePackage(el, val)}
                         def_={res_packages.has(el.id)}
                />
            )}
        </div>
        <button id={"create-project-packages-manage"}>Manage Packages</button>
    </div>)
}
