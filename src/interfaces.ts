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
    template: ITemplate,
    packages: string[],
    workspace: {
        widgets: {}[],
        buttons: IAsideButtonExtended[],
        opened_files: OpenedFile[],
        launch_references: LaunchTemplateReference[],
        launch_objects: LaunchObject[],
        launch_templates: LaunchTemplate[],
        current_launch: number | null
    },
    vars: IVar[],
    tasks: any[],
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
    packages_id: string[],
    dependencies: Dependency[],
    launches: LaunchTemplate[]

}

interface Dependency {
    program: string,
    platform?: string,
    level: "CRITICAL" | "CONFLICTS" | "OPTIONAL"
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
    widget: string
    component_type: "Light" | "Heavy"
    component: (props?: { active?: boolean }) => React.ReactElement

}

interface IAsideButtonExtended {
    pos: string
    widget: string
    order: number
    alt: string
    keys: string
    icon: string
    component_type?: "Light" | "Heavy"
    component?: () => React.ReactElement

}

interface OpenedFile {
    path: string,
    cursor: [number, number],
    name: string
}

interface Opened extends OpenedFile {
    id: number;
    cache_id: number
}

interface ICodeSpace {
    id: number,
    current_file: number | null // opened file id
    opened_files: Opened[]
    opened_files_stack: number[]
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


interface FsExtIcon {
    typ: string,
    ext: string[] | null,
    name: string[] | null,
    icon: string
}

interface FsConfigIcons {
    theme: string,
    scheme: number,
    icons: FsExtIcon[]
}


interface FileCacheLight {
    path: string,
    content: string;
    is_dirty: boolean
}

interface FileCache extends FileCacheLight {
    id: number

}


interface configFsTemplate {
    id: string,
    title: string,
    typ: "file" | "dir",
    icon?: string,
    ext?: string,
    default_content?: string,
    inner?: configFsTemplate[],
    base_name?: string
}


interface Cache {
    settings: Settings,
    templates: ITemplate[],
    packages: IPackage[],
    groups: string[],
    data_dir_path: string,
    projects_dir: string,
    os: string,
    file_icons: FsConfigIcons[],
    file_templates: configFsTemplate[]
    shells: string[]
    themes: Theme[]

}


interface LaunchTemplate {
    id: number,
    title: string,
    scheme: number,
    icon?: string,
    sections: LaunchSection[],
    actions: LaunchAction[],
    functions: LaunchFunction[]
}

interface LaunchTemplateReference {
    id: number,
    template: [string, number],
    scheme: number,
    icon?: string,
    name: string,
    results: LaunchTemplateResult
}

type LaunchTemplateResult = Record<number, Record<string, string>>

interface LaunchFunction {
    id: number,
    actions: LaunchFunctionAction[]
}

interface LaunchFunctionAction {
    function: string,
    args?: [string, [string, number][]][]
}

interface LaunchSection {
    id: number,
    title?: string,
    options: LaunchOption[]
}

interface LaunchOption {
    id: string,
    title: string,
    typ: {
        typ: "input" | "check" | "path" | "list",
        restriction?: number,
        list_types?: string[],
        path_type?: "file" | "dir" | "all"
    },
    def: string | number
}

interface LaunchAction {
    id: number,
    next?: number,
    if_: IfStatementPart[][],
    command: {
        command: string,
        args?: string,
        cwd?: string,
        env?: [string, string][]
    }[]
}

interface LaunchObject {
    id: number,
    launch_reference: number,
    scheme: number,
    tasks: LaunchTask[]
}


interface LaunchTask {
    SINGLE?: {
        command: string,
        env?: [string, string][],
        cwd?: string
    },
    GRAPH?: {
        command: string,
        next: LaunchTask,
        env?: [string, string][],
        cwd?: string
    }
}

/**
 *
 */
interface Theme {
    id: string;
    name: string;
    meta?: {
        authors?: string[]
        version?: string
        description?: string;
        source?: string
    }
    vars?: {
        name: string;
        value: string
    }[]
    pages: {
        id: string;
        elements: {
            selector: string
            color?: string
            background_color?: string;
            background_image?: string;
            background_opacity: string;

            border?: string
            border_right?: string
            border_left?: string
            border_top?: string
            border_bottom?: string
            border_x?: string
            border_y?: string

            border_size?: string
            border_color?: string
            border_style?: string

            border_radius?: string

            margin?: string
            margin_left?: string
            margin_right?: string
            margin_top?: string
            margin_bottom?: string
            margin_x?: string
            margin_y?: string

            padding?: string
            padding_left?: string
            padding_right?: string
            padding_top?: string
            padding_bottom?: string
            padding_x?: string
            padding_y?: string

            width?: string
            height?: string

            opacity?: string
            font?: string
            font_family?: string
            font_weight?: string


        }[]
    }[]
}


interface SettingsElement {
    id: number;
    title: string;
    sections?: ISettingsSection[]
    list?: SettingsList
}


interface SettingsList {
    window: string;
    list: SettingsListElement[]
}


type SettingsListElement = Record<string, string>


interface ISettingsSection {
    id: number;
    title?: string;
    parameters: ISettingsParameter[]
}

interface ISettingsParameter {
    id: number;
    title: string;
    type: "input" | "area" | "check" | "file" | "dir" | "fs" | "list" | "gen"
    list?: string[]
    def?: string | boolean | string[]
    readonly?: boolean;
    required?: boolean;
}


interface Settings {
    doctype: string;
    version: string;
    general: {
        path_to_projects: string,
        project_groups: string[]
    },
    appearance: {
        theme: string,
        lang: string,
        font: string,
        font_size: number
    },
    run: {
        shells: string[]
    }
}