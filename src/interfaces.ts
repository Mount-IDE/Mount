
interface IRecentProject {
    name: string,
    path: string
    meta: {
        authors: string[],
        description: string,
        license: string,
        group: string,
        tags: string[],
        icon?: string
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
        group: string,
        icon?: string
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
        actions: IPackageAction[],
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
    name: string,
    value: IVal
}

type IVal = string | boolean | number | string[]


interface ISection {
    id: number,
    label: string,
    list: [boolean, boolean],
    params: IPackageParameter[]
}

interface IParameter {
    out: string,
    label: string | [string, string],
    def: IVal,
    typ: string[]
    while_?: string
    req?: boolean
}


interface IAction {
    id: number,
    for_?: string,
    callable: boolean,
    if_: IfStatementPart[][],
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


interface IfStatementPart {
    from: string,
    oper: string,
    value: IVal
}


interface IAsideButton {
    id: number
    alt: string
    keys: string
    icon: string
    widget: string
    component_type: "Light" | "Heavy"
    component: React.ComponentType<{ active?: boolean }>

}

interface IAsideButtonExtended {
    pos: string
    widget: string
    order: number
    alt: string
    keys: string
    icon: string
    component_type?: "Light" | "Heavy"
    component?: React.ComponentType<{ active?: boolean }>

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

interface PackageInner {
    main: IPackage,
    config: string,

}

interface Cache {
    recent_projects: IRecentProject[]
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
    themes: string[]

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
    gen_def?: string
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

type IThemeInner = string | {
    self?: string
    right?: string
    left?: string
    top?: string
    bottom?: string
}

interface IThemeProjectInner {
    color?: string
    underscore?: string
    hover?: {
        color?: string
        underscore?: string
    }
}

interface IThemeTitleBar {
    this?: {
        background?: string
        border?: IThemeInner
    },
    button?: {
        background?: string
        border?: IThemeInner
        hover?: {
            id: "wrap" | "resize" | "close" | "other",
            background?: string,
            border?: IThemeInner
        }[] | {
            background?: string
            border: IThemeInner
        }
    }

}

interface IThemeSyntax {
    base_color?: string,
    tokens?: Record<string, string>
    colors?: Record<string, string>
}

interface ITheme {
    id: string
    name: string
    schema: number
    meta?: {
        authors?: string[]
        description?: string
        tags?: string[]
    }
    colors?: { name: string, value: string }[]
    elements?: {
        common?: {
            input?: {
                this?: {
                    background?: string
                    rounded?: string
                    border?: IThemeInner
                }
                field?: {
                    background?: string
                    border?: IThemeInner
                    rounded?: string
                    color?: string
                    placeholder_color?: string
                    hover?: {
                        background?: string
                        border?: IThemeInner
                        rounded?: string
                        color?: string
                        placeholder_color?: string
                    }
                    focus?: {
                        background?: string
                        border?: IThemeInner
                        color?: string
                        placeholder_color?: string
                    }
                }
                label?: {
                    color?: string
                }
            }
            check?: {
                this?: {
                    background: string
                    rounded: string
                    border?: IThemeInner
                }
                field?: {
                    background?: string
                    border?: IThemeInner
                    focus?: {
                        background?: string
                        border?: IThemeInner
                    }
                }
                label?: {
                    color?: string
                }
            }
            list?: {
                this?: {
                    background?: string
                    rounded?: string
                    border?: IThemeInner
                }
                field?: {
                    background?: string
                    border?: IThemeInner
                    color?: string
                    focus?: {
                        background?: string
                        border?: IThemeInner
                        color?: string
                    }
                    hover?: {
                        background?: string
                        border?: IThemeInner
                        color?: string
                    }
                }
                label?: {
                    color?: string
                }
            }
            gen?: {
                this?: {
                    background?: string
                    border?: IThemeInner

                }
                element?: {
                    this?: {
                        background?: string
                        rounded?: string
                        hover?: {
                            background?: string
                            rounded?: string
                        }
                        focus?: {
                            background?: string
                            rounded?: string
                        }
                    }
                    label?: {
                        color?: string
                        placeholder_color?: string
                        hover?: {
                            color?: string
                            placeholder_color?: string
                        }
                        focus?: {
                            color?: string
                            placeholder_color?: string
                        }
                    }
                    button?: {
                        background?: string
                        rounded?: string
                        border?: IThemeInner
                        hover?: {
                            background?: string
                            border?: IThemeInner
                        }
                        focus?: {
                            background?: string
                            border?: IThemeInner
                        }
                    }
                }

                button?: {
                    background?: string
                    rounded?: string
                    border?: IThemeInner
                    hover?: {
                        background?: string
                        border?: IThemeInner
                    }
                    focus?: {
                        background?: string
                        border?: IThemeInner
                    }
                }
            }

            button?: {
                background?: string
                rounded?: string
                border?: IThemeInner
                hover?: {
                    background?: string
                    border?: IThemeInner
                }
                focus?: {
                    background?: string
                    border?: IThemeInner
                }
            }

            main_button?: {
                background?: string
                rounded?: string
                border?: IThemeInner
                hover?: {
                    background?: string
                    border?: IThemeInner
                }
                focus?: {
                    background?: string
                    border?: IThemeInner
                }
            },
            title_bar?: IThemeTitleBar,
            icons?: {
                color?: string
            }

            syntax?: IThemeSyntax

        }
        mainpage?: {
            left?: {
                background?: string
                rounded?: string
                padding?: IThemeInner
                border?: IThemeInner
            },
            right?: {
                rounded?: string
                padding?: IThemeInner
                border?: IThemeInner
                background?: string
            },
            filters?: {
                border: IThemeInner
            },
            project?: {
                this?: {
                    background?: string
                    padding?: IThemeInner
                    rounded?: string,
                    hover?: {
                        background?: string
                        rounded?: string
                    }
                }

                icon?: {
                    rounded?: string
                    border?: IThemeInner
                }

                name?: IThemeProjectInner
                path?: IThemeProjectInner
                packages?: IThemeProjectInner
                tags?: IThemeProjectInner
                more?: {
                    img?: string
                }
            }
            title_bar?: IThemeTitleBar
        }

        project_space?: {
            this?: {
                background?: string
            }
            mini_aside?: {
                right?: {
                    border?: IThemeInner
                    background?: string,
                    hr?: {
                        border?: string
                    }
                },
                left?: {
                    border?: IThemeInner
                    background?: string,
                    hr?: {
                        border?: string
                    }
                }
            }
            aside?: {
                left?: {
                    border?: IThemeInner
                    rounded?: string
                    background?: string
                    padding?: IThemeInner
                    hr?: {
                        border?: string
                    }
                }
                right?: {
                    border?: IThemeInner
                    rounded?: string
                    background?: string
                    padding?: IThemeInner
                    hr?: {
                        border?: string
                    }
                }
            }
            center?: {
                this?: {
                    border?: IThemeInner
                    background?: string
                    rounded?: string
                    padding?: IThemeInner
                }
                file_list?: {
                    this?: {
                        background?: string
                        border?: IThemeInner
                    }

                    element?: {
                        background?: string
                        border?: IThemeInner
                        rounded?: string
                        hover?: {
                            background?: string
                            border?: IThemeInner
                        }
                        focus?: {
                            background?: string
                            border?: IThemeInner
                        }
                    }
                }
            }

            bottom?: {
                background?: string
                border?: IThemeInner
                rounded?: string
                padding?: IThemeInner
                hr?: {
                    border?: string
                    active_border?: string
                }
            }

            footer?: {
                border?: IThemeInner
                background?: string
                rounded?: string
                padding?: IThemeInner
            }


            title_bar?: IThemeTitleBar
        }

        launch?: {
            this?: {
                background?: string
                border?: IThemeInner
                rounded?: string
                padding?: IThemeInner
            }
            left?: {
                this?: {
                    background?: string
                    border?: IThemeInner
                    rounded?: string
                    padding?: IThemeInner
                }
                list?: {
                    background?: string
                    border?: IThemeInner
                    padding?: IThemeInner
                    rounded?: string
                    hover?: {
                        border?: IThemeInner
                        background?: string
                    }
                    focus?: {
                        border?: IThemeInner
                        background?: string
                    }
                }
            }
            right?: {
                background?: string
                border?: IThemeInner
                padding?: IThemeInner
                rounded?: string
            }
        }

        settings?: {
            this?: {
                background?: string
                border?: IThemeInner
                rounded?: string
                padding?: IThemeInner
            }
            left?: {
                this?: {
                    background?: string
                    rounded?: string
                    border?: IThemeInner
                    padding?: IThemeInner
                }
                list?: {
                    background?: string
                    border?: IThemeInner
                    rounded?: string
                    hover?: {
                        background?: string
                        border?: IThemeInner
                    }
                    focus?: {
                        background?: string
                        border?: IThemeInner
                    }
                }

            }

            right?: {
                background?: string
                border?: IThemeInner
                rounded?: string
                padding?: IThemeInner
            }
        },

        create_project?: {
            this?: {
                padding?: IThemeInner,
                border?: IThemeInner,
                background?: string
                rounded?: string
            },
            left?: {
                padding?: IThemeInner,
                border?: IThemeInner,
                background?: string
                rounded?: string
            },
            right?: {
                padding?: IThemeInner,
                border?: IThemeInner,
                background?: string
                rounded?: string
            },

        }


        create_entities?: {
            this?: {
                padding?: IThemeInner,
                border?: IThemeInner,
                background?: string
                rounded?: string
            },
            left?: {
                this?: {
                    padding?: IThemeInner,
                    border?: IThemeInner,
                    background?: string
                    rounded?: string
                },
                field?: {
                    padding?: IThemeInner,
                    border?: IThemeInner,
                    background?: string
                    rounded?: string
                    color?: string,
                    hover?: {
                        padding?: IThemeInner,
                        border?: IThemeInner,
                        background?: string
                        rounded?: string
                        color?: string
                    },
                    focus?: {
                        padding?: IThemeInner,
                        border?: IThemeInner,
                        background?: string
                        rounded?: string
                        color?: string
                    }
                }
            }

            right?: {
                padding?: IThemeInner,
                border?: IThemeInner,
                background?: string
                rounded?: string
            }
        }
    }

}


interface IPackage {
    id: string
    name: string
    meta?: {
        version?: string
        authors?: string[]
        description?: string
        tags?: string[]
        source?: string
        typ?: "language" | "framework" | "tool" | "build_system"
        icon?: string
        license?: string
    }
    scheme: number
    dependencies: {
        typ: "package" | "program"
        name: string
        version?: string
        platform?: ("windows" | "macos" | "linux")[] | "all"
        version_check_command?: string
        level: "required" | "conflicts"
    }[]
    startup: {
        options?:
            IPackageParameter[]

        actions?: IPackageAction[]
    }
    highlight: IPackageHighlight[]
    files: {
        extentions: string[], //string
        files?: string[] // regex str
        ignore_files?: string[] // regex str
    }
    tasks?: []
    var?: {
        name: string,
        value: string | boolean | string[]
        readonly?: boolean
    }[]
    components?:
        {
            id: string
            typ: "compiler" | "transpiler" | "interpreter" | "lsp" | "formatter" | "debugger" | "build_system" | "pack_manager"
            program: string // имя программы
            platform: ("windows" | "macos" | "linux")[] | "all"
            languages: string[]
            priority?: number
            arguments?: string[]
            builtin_params: {
                is_builtin: boolean
                url?: string
                path?: string
                in_path?: boolean
                min_version?: string
                version_check_command?: string | {
                    platform: string
                    command: string
                }[]
            }
        }[]

}

interface IPackageHighlight {
    // for finding parsers in <package>/highlight/<id>/{ grammar.js, parser.wasm, highlight.csm, ... }
    id: string, // id of entity
    lang: string, // language name
    extentions: string[] // we can restrict logic only for files with this extensions
    ignore_files?: string[] // list of files that must be ignored
    files?: string[] // we can restrict logic only for theese files
    nodes: Record<string, string>
    syntax?: IThemeSyntax

}

interface IPackageParameter {
    id: string
    title: string
    typ: {
        typ: "input" | "area" | "list" | "gen" | "file" | "check",
        fs_type?: "file" | "dir" | "fs",
        list_type?: string[]
        placeholder?: string
        required?: boolean
        readonly?: boolean
        validate?: string
        fs_filter?: string[]
        gen_min?: number
        gen_max?: number
    }
    def?: string | boolean | string[]
    while_?: string

}

interface IPackageAction {
    id: number
    next?: number[]
    if_?: {
        from?: string,
        op: "==" | "!=" | ">" | "<" | ">=" | "<=" | "empty" | "!empty" | "in" | "!in" | "regex" | "!regex" | "!already" | "stopped" | "!stopped" | "installed" | "!installed",
        value?: string | boolean | string[] | number
    }[][]
    on_error: "continue" | "stop_graph" | "stop_all"

    platform?: ("windows" | "macos" | "linux")[] | "all"
    commands?: {
        platform?: ("windows" | "macos" | "linux")[] | "all"
        cwd?: string
        env?: [string, string][]
        needed_exit_code?: number[]
        command: string
    }[]

}

interface PackageConfigMeta {
    project: IProject
}

type MethodBinding = string | ((args: any, meta: PackageConfigMeta) => { method: string; params: any } | undefined)


interface PackageConfig {
    id: string
    comments: {
        line?: string
        block?: string
    }
    languages: {

        elements: {
            id: string,
            extensions: string[],
            //        grammar: string
        }[]

    }


    methods?: {
        fileOpened?: MethodBinding | null
        fileClosed?: MethodBinding | null
        fileSaved?: MethodBinding | null
        fileChanged?: MethodBinding | null
        goToDefinition?: MethodBinding | null
        goToDeclaration?: MethodBinding | null
        goToImplementation?: MethodBinding | null
        hover?: MethodBinding | null
        completion?: MethodBinding | null
        signatureHelp?: MethodBinding | null
        references?: MethodBinding | null
        rename?: MethodBinding | null
        codeAction?: MethodBinding | null
        formatting?: MethodBinding | null
        inlayHints?: MethodBinding | null
        documentSymbol?: MethodBinding | null
        semanticTokens?: MethodBinding | null
    }
    lsp?: {
        buildInitializationOptions?: (meta: PackageConfigMeta) => Record<string, any> | undefined
        buildCapabilities?: (defaults: Record<string, any>, meta: PackageConfigMeta) => Record<string, any>
        onInitialized?: (serverCapabilities: Record<string, any>, meta: PackageConfigMeta) => void

        onServerRequest?: (method: string, params: any, meta: PackageConfigMeta) => any | undefined
        onServerNotification?: (method: string, params: any, meta: PackageConfigMeta) => void

        beforeShutdown?: (meta: PackageConfigMeta) => void
        onCrash?: (exitCode: number | null, meta: PackageConfigMeta) => "restart" | "ignore"
    } | null
}


interface IGrammar {
    scopeName: string;              // уникальное имя scope языка, например "source.rust"
    patterns: IRawRule[];           // список правил, применяемых на верхнем уровне файла
    repository?: IRawRepository;    // "библиотека" именованных правил для переиспользования
    injections?: { [expression: string]: IRawRule }; // внедрение правил в другие грамматики
    injectionSelector?: string;
    fileTypes?: string[];           // расширения файлов (мета-инфо, не обязательно)
    name?: string;                  // человекочитаемое имя
    firstLineMatch?: string;
}

interface IRawRule {
    id?: number;
    include?: string;               // ссылка на другое правило: "#keywords", "$self", "$base"

    name?: string;                  // scope-имя токена, например "keyword.control.rust"
    contentName?: string;           // scope для содержимого между begin/end

    match?: string;                 // regex для однострочного совпадения
    begin?: string;                 // regex начала блока (многострочного)
    end?: string;                   // regex конца блока
    while?: string;                 // альтернатива end — блок продолжается, пока матчится while

    captures?: IRawCaptures;        // именование групп захвата для `match`
    beginCaptures?: IRawCaptures;   // именование групп в `begin`
    endCaptures?: IRawCaptures;     // именование групп в `end`
    whileCaptures?: IRawCaptures;

    patterns?: IRawRule[];          // вложенные правила (внутри begin/end блока)

    applyEndPatternLast?: boolean;
}

interface IRawCaptures {
    [group: string]: {
        name?: string;
        patterns?: IRawRule[];
    };
}

interface IRawRepository {
    [name: string]: IRawRule;
}

