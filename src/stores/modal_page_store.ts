import {Props as PageProps} from "../components/common/Page"
import {ReactElement} from "react";
import {create} from "zustand";


/**
 * extended data of page that uses for Page.tsx component
 */
type ModalPage = {
    props: PageProps,
    children: ReactElement
}


/**
 * unsaved data of page
 */
interface ModalData {
    [id: string]: {
        [id: string]: IVal
    }
}

/**
 * Unsaved - page has unsaved data and can not be closed
 * Done - all data was saved and page was closed
 */
export const enum SaveType {
    Unsaved,
    Done
}


/**
 * Done - save function has in store and called successfull
 * Error - function has in store, but function return nonzero number or throw error
 * NotFound - function not found in store
 */
export const enum SaveEvent {
    Done,
    Error,
    NotFound
}

/**
 * Done - page was added in queue
 * Has - page already in queue
 */
export const enum PageType {
    Done,
    Has
}


export type SaveCallback = (() => number | undefined)

interface Type {
    /**
     * page queue
     */
    pages: {
        [id: string]: ModalPage
    }
    /**
     * unsaved data of pages
     */
    data: ModalData

    /**
     * save functions buffer
     */
    save_fn: {
        [page: string]: SaveCallback
    }
    /**
     * add page if queue don't contains it
     * @param id
     * @param props
     */
    add_page: (id: string, props: ModalPage) => PageType

    /**
     * remove page and don't check data
     * @param id
     */
    remove_page_unchecked: (id: string) => void
    /**
     * remove page if store don't have unsaved data
     * @param id
     */
    remove_page: (id: string) => SaveType


    /**
     * write data of page
     * @param page
     * @param field
     * @param value
     */
    write_data: (page: string, field: string, value: IVal) => void

    /**
     * save unsaved data of needed page
     * @param page
     */
    save_data: (page: string) => SaveEvent

    /**
     * append save function to buffer
     * @param page
     * @param fn
     */
    subscribe_on_save: (page: string, fn: SaveCallback) => (() => void)

    /**
     *
     * @param page
     * @param field
     */
    has_data: (page: string, field: string) => boolean


    /**
     * removes save function from buffer
     * @param page
     */
    unsubscribe_on_save: (page: string) => void
}

export const modalPageStore = create<Type>((set, get) => ({
    data: {},
    pages: {},
    save_fn: {},
    add_page(id: string, props: ModalPage): PageType {
        let pages = get().pages;
        if (Object.keys(pages).includes(id)) {
            return PageType.Has
        }
        pages[id] = props
        set({
            pages: {...pages}
        })
        return PageType.Done
    },
    has_data(page: string, field: string): boolean {
        return get().data[page]?.[field] != undefined
    },
    remove_page(id: string): SaveType {
        if (get().data[id] != undefined) {
            return SaveType.Unsaved
        }
        return SaveType.Done;
    },
    remove_page_unchecked(id: string): void {
        let fn = get().save_fn;
        delete fn[id]
        set({
            save_fn: {...fn}
        })
    },
    save_data(page: string): SaveEvent {
        let fn = get().save_fn;
        if (Object.keys(fn).includes(page)) {
            try {
                let f = fn[page]
                let res = f()
                if (typeof res == "number" && res != 0) {
                    return SaveEvent.Error
                }
                return SaveEvent.Done
            } catch {
                return SaveEvent.Error
            }
        }
        return SaveEvent.NotFound


    },
    subscribe_on_save(page: string, fn): () => void {
        set({
            save_fn: {...get().save_fn, [page]: fn}
        })
        return function () {
            modalPageStore.getState().unsubscribe_on_save(page)
        };
    },
    write_data(page: string, field: string, value: IVal): void {
        let data = get().data;
        set({
            data: {
                ...data,
                [page]: {
                    ...data[page],
                    [field]: value
                }
            }
        })
    },
    unsubscribe_on_save(page: string): void {
        let fn = get().save_fn
        delete fn[page]
        set({
            save_fn: {...fn}
        })
    }

}))







