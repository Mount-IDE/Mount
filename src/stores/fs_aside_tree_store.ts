import {create} from "zustand";
import {invoke} from "@tauri-apps/api/core";
import {listen, type UnlistenFn} from "@tauri-apps/api/event";

type FsNode = FsFile | FsDirectory;

type FsWatchEvent =
    | { kind: "created"; path: string; node?: FsNode }
    | { kind: "removed"; path: string }
    | { kind: "modified"; path: string }
    | { kind: "renamed"; old_path: string; new_path: string; node?: FsNode };

interface Type {
    tree: FsDirectory | null;
    load_tree: (cwd: string) => Promise<void>;
    watch: (cwd: string) => Promise<void>;
    change_node: (path: string, node: FsNode) => void
    move_node: (old_path: string, node: FsNode) => void
    mark_modified: (path: string) => void;
    delete_node: (path: string) => void;
    add_node: (node: FsNode) => void;
    unwatch: () => Promise<void>;
}

let fs_watch_unlisten: UnlistenFn | null = null;
let watched_project_path = "";

const normalize_path = (path: string): string =>
    path.replace(/\\/g, "/").replace(/\/+$/, "").toLowerCase();

const same_path = (left: string, right: string): boolean =>
    normalize_path(left) === normalize_path(right);

const get_parent_path = (path: string): string => {
    const without_trailing_separator = path.replace(/[\\/]+$/, "");
    const separator_index = Math.max(
        without_trailing_separator.lastIndexOf("\\"),
        without_trailing_separator.lastIndexOf("/"),
    );

    if (separator_index === -1) {
        return "";
    }

    return without_trailing_separator.slice(0, separator_index);
}

const is_directory = (node: FsNode): node is FsDirectory =>
    Array.isArray((node as FsDirectory).directories);

const sort_by_name = <T extends { name: string }>(items: T[]): T[] =>
    [...items].sort((left, right) => left.name.localeCompare(right.name));

const upsert_node = <T extends FsNode>(items: T[], node: T): T[] =>
    sort_by_name([...items.filter(item => !same_path(item.path, node.path)), node]);

const add_node_to_directory = (directory: FsDirectory, node: FsNode): [FsDirectory, boolean] => {
    const parent_path = get_parent_path(node.path);

    if (same_path(directory.path, parent_path)) {
        if (is_directory(node)) {
            return [
                {
                    ...directory,
                    directories: upsert_node(directory.directories, node),
                    files: directory.files.filter(file => !same_path(file.path, node.path)),
                },
                true,
            ];
        }

        return [
            {
                ...directory,
                directories: directory.directories.filter(dir => !same_path(dir.path, node.path)),
                files: upsert_node(directory.files, node),
            },
            true,
        ];
    }

    let changed = false;
    const directories = directory.directories.map(child => {
        const [next_child, child_changed] = add_node_to_directory(child, node);
        changed ||= child_changed;
        return next_child;
    });

    return changed ? [{...directory, directories}, true] : [directory, false];
}

const delete_node_from_directory = (directory: FsDirectory, path: string): [FsDirectory, boolean] => {
    const files = directory.files.filter(file => !same_path(file.path, path));
    const directories_without_target = directory.directories.filter(dir => !same_path(dir.path, path));

    if (
        files.length !== directory.files.length ||
        directories_without_target.length !== directory.directories.length
    ) {
        return [{...directory, files, directories: directories_without_target}, true];
    }

    let changed = false;
    const directories = directory.directories.map(child => {
        const [next_child, child_changed] = delete_node_from_directory(child, path);
        changed ||= child_changed;
        return next_child;
    });

    return changed ? [{...directory, directories}, true] : [directory, false];
}

const change_node_in_directory = (
    directory: FsDirectory,
    path: string,
    node: FsNode,
): [FsDirectory, boolean] => {
    if (same_path(directory.path, path) && is_directory(node)) {
        return [node, true];
    }

    const file_index = directory.files.findIndex(file => same_path(file.path, path));
    if (file_index !== -1 && !is_directory(node)) {
        const files = [...directory.files];
        files[file_index] = node;
        return [{...directory, files: sort_by_name(files)}, true];
    }

    const directory_index = directory.directories.findIndex(dir => same_path(dir.path, path));
    if (directory_index !== -1 && is_directory(node)) {
        const directories = [...directory.directories];
        directories[directory_index] = node;
        return [{...directory, directories: sort_by_name(directories)}, true];
    }

    let changed = false;
    const directories = directory.directories.map(child => {
        const [next_child, child_changed] = change_node_in_directory(child, path, node);
        changed ||= child_changed;
        return next_child;
    });

    return changed ? [{...directory, directories}, true] : [directory, false];
}

const mark_modified_in_directory = (directory: FsDirectory, path: string): [FsDirectory, boolean] => {
    const file_index = directory.files.findIndex(file => same_path(file.path, path));

    if (file_index !== -1) {
        const files = [...directory.files];
        files[file_index] = {...files[file_index], modified: true};
        return [{...directory, files}, true];
    }

    let changed = false;
    const directories = directory.directories.map(child => {
        const [next_child, child_changed] = mark_modified_in_directory(child, path);
        changed ||= child_changed;
        return next_child;
    });

    return changed ? [{...directory, directories}, true] : [directory, false];
}

const add_node_to_tree = (tree: FsDirectory, node: FsNode): FsDirectory => {
    if (same_path(tree.path, node.path) && is_directory(node)) {
        return node;
    }

    const [next_tree, changed] = add_node_to_directory(tree, node);
    return changed ? next_tree : tree;
}

const delete_node_from_tree = (tree: FsDirectory, path: string): FsDirectory | null => {
    if (same_path(tree.path, path)) {
        return null;
    }

    const [next_tree, changed] = delete_node_from_directory(tree, path);
    return changed ? next_tree : tree;
}

const change_node_in_tree = (tree: FsDirectory, path: string, node: FsNode): FsDirectory => {
    const [next_tree, changed] = change_node_in_directory(tree, path, node);
    return changed ? next_tree : add_node_to_tree(tree, node);
}

const move_node_in_tree = (tree: FsDirectory, old_path: string, node: FsNode): FsDirectory | null => {
    const without_old_node = delete_node_from_tree(tree, old_path);

    if (without_old_node === null) {
        return is_directory(node) ? node : null;
    }

    return add_node_to_tree(without_old_node, node);
}

const mark_modified_in_tree = (tree: FsDirectory, path: string): FsDirectory => {
    const [next_tree, changed] = mark_modified_in_directory(tree, path);
    return changed ? next_tree : tree;
}

export const fsAsideTreeStore =
    create<Type>((set, get) => (
        {
            tree: null,
            add_node(node: FsNode): void {
                set(state => ({
                    tree: state.tree === null ? (is_directory(node) ? node : null) : add_node_to_tree(state.tree, node),
                }));
            },
            change_node(path: string, node: FsNode): void {
                set(state => ({
                    tree: state.tree === null ? null : change_node_in_tree(state.tree, path, node),
                }));
            },
            move_node(old_path: string, node: FsNode): void {
                set(state => ({
                    tree: state.tree === null ? null : move_node_in_tree(state.tree, old_path, node),
                }));
            },
            mark_modified(path: string): void {
                set(state => ({
                    tree: state.tree === null ? null : mark_modified_in_tree(state.tree, path),
                }));
            },
            delete_node(path: string): void {
                set(state => ({
                    tree: state.tree === null ? null : delete_node_from_tree(state.tree, path),
                }));
            },
            async load_tree(cwd: string): Promise<void> {
                if (cwd.length === 0) {
                    set({tree: null});
                    return;
                }

                try{
                    const res = await invoke<FsDirectory>("read_dir_rec", {
                        cwd: cwd
                    })
                    set({tree: res})
                }catch (e){
                    console.error(e)
                }
            },
            async watch(cwd: string): Promise<void> {
                if (cwd.length === 0 || watched_project_path === cwd) {
                    return;
                }

                await get().unwatch();

                fs_watch_unlisten = await listen<FsWatchEvent>("fs-event", event => {
                    const payload = event.payload;

                    if (payload.kind === "created" && payload.node) {
                        get().add_node(payload.node);
                        return;
                    }

                    if (payload.kind === "removed") {
                        get().delete_node(payload.path);
                        return;
                    }

                    if (payload.kind === "renamed" && payload.node) {
                        get().move_node(payload.old_path, payload.node);
                        return;
                    }

                    if (payload.kind === "modified") {
                        get().mark_modified(payload.path);
                    }
                });

                try {
                    await invoke("watch_project", {projectPath: cwd});
                    watched_project_path = cwd;
                } catch (e) {
                    fs_watch_unlisten();
                    fs_watch_unlisten = null;
                    console.error(e);
                }
            },
            async unwatch(): Promise<void> {
                if (fs_watch_unlisten !== null) {
                    fs_watch_unlisten();
                    fs_watch_unlisten = null;
                }

                if (watched_project_path.length === 0) {
                    return;
                }

                try {
                    await invoke("unwatch_project");
                } catch (e) {
                    console.error(e);
                } finally {
                    watched_project_path = "";
                }
            }

        }
    ))
