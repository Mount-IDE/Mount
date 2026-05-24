interface IRecentProject {
    name: string,
    path: string
    meta: {
        authors: string[],
        description: string,
        license: string,
        group: string,
        tags: string[]
    },
    packages: string[],
    last_opened: number
}


interface IProject {
    name: string,
    path: string,
    meta: {
        authors: string[],
        description: string,
        license: string,
        tags: string[],
        group: string
    },
    packages: string[],
    workspace:{
        widgets: {}[],
        buttons: IAsideButtonExtended[],
        opened_files: OpenedFile[]
    }
}


interface ITemplate {
    id: string,
    name: string,
    meta?: {
        authors: string[],
        description: string,
        icon: string
    }
    startup: {
        var: IVar[],
        actions: IAction[],
        sections: ISection[]
    },
    packages_id: string[]
}


interface IVar {
    name: String,
    val: IVal
}

type IVal = string | boolean | number | string[]


interface ISection {
    id: number,
    label: string,
    list: [boolean, boolean],
    params: IParameter[]
}

interface IParameter {
    out: string,
    label: string | [string, string],
    // val: IVal,
    def: IVal,
    typ: string[]
    while_?: string
}


interface IAction {
    id: number,
    for_?: string,
    callable: boolean,
    if_: IfStatement[],
    on_error: String,
    next?: number
    command:
        {
            platform: string
            shell: string,
            env?: [string, string][],
            command: {
                SINGLE: string
            } |
                {
                    WithArgs: [string, string[]]
                }
        }[]
}

interface IfStatement {
    or?: IfStatementPart[],
    all?: IfStatementPart[]
}

interface IfStatementPart {
    from: string,
    oper: string,
    value: IVal
}


interface IPackage {
    id: string,
    name: string,
    meta: {
        authors: string[],
        description: string
    }
    startup: {
        var: IVar[],
        actions: IAction[],
        parameters: IParameter[]
    }
}


interface IAsideButton {
    id: number
    alt: string
    keys: string
    icon: string
    component: ()=> React.ReactElement

}

interface IAsideButtonExtended {
    pos: string
    order: number
    alt: string
    keys: string
    icon: string
    component: ()=> React.ReactElement

}

interface OpenedFile {
    path: string,
    cursor: [number, number],
    name: string
}

interface Opened extends OpenedFile{
    id: number;
    cache_id:number
}

interface ICodeSpace{
    id: number,
    current_file: number|null // opened file id
    opened_files: Opened[]
}




interface FsDirectory {
    name: string;
    typ_: "dir";
    directories: FsDirectory[]
    files: FsFile[]
    path: string;
}

interface FsFile {
    name: string;
    path: string;
    typ_: "file"
    modified?: boolean;
}



interface FsExtIcon{
    typ: string,
    ext: string[],
    icon: string
}
interface FsConfigIcons{
    theme: string,
    scheme: number,
    icons:FsExtIcon[]
}



interface FileCacheLight{
    path: string,
    content: string;
    is_dirty: boolean
}
interface FileCache extends  FileCacheLight{
    id: number

}


interface configFsTemplate{
    id: string,
    title: string,
    typ: "file"|"dir",
    icon?:string,
    ext?:string,
    default_content?:string,
    inner?:configFsTemplate[],
    base_name?:string
}
