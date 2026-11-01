import {Props as PageProps} from "../components/common/Page"
import {create} from "zustand";


/**
 * extended data of page that uses for Page.tsx component
 */
export type ModalPageState = {
    props: PageProps,
    unsaved_data?: Record<string, IValExtended>, // first: '<page_id>:<field_id>'
    default_data: Record<string, IValExtended> // first: '<page_id>:<field_id>'
    save_fn?: SaveCallback
}

/**
 * uses for page child component
 */
export interface ModalApi {
    write_unsaved: (page: string, field: string, value: IValExtended) => boolean
    save: (page: string) => Promise<WriteEvent>
    get_unsaved: (page: string, field: string) => IValExtended | null
    get_saved: (page: string, field: string) => IValExtended | null

    write_unsaved_obj: (page: string, obj: Record<string, IValExtended>) => boolean
}

export interface ModalApiShorted {
    write_unsaved: (field: string, value: IValExtended) => boolean
    save: () => Promise<WriteEvent>
    get_unsaved: (field: string) => IValExtended | null
    get_saved: (field: string) => IVal | null
    write_unsaved_obj: (obj: Record<string, IValExtended>) => boolean

    get_obj: () => any | null
}



/**
 * Used fo page buttons
 */
export interface ModalApiExtended extends ModalApi {
    remove_current_save: () => RemoveEvent
    remove_current_unsaved: () => void
    hide: () => void
}

/**
 * Unsaved - page has unsaved data and can not be closed
 * Done - all data was saved and page was closed
 */
export const enum RemoveEvent {
    Unsaved,
    Done,
    NotFound
}


/**
 * Done - save function has in store and called successfull
 * Error - function has in store, but function return nonzero number or throw error
 * NotFound - function not found in store
 */
export const enum WriteEvent {
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


export type SaveCallback = (() => Promise<void>)

interface Type {
    /**
     * page queue
     */
    pages: Record<string, ModalPageState>


    write_default_data: (page: string, field: string, value: IValExtended) => WriteEvent;

    set_default_data: (page: string, data: Record<string, IValExtended>) => WriteEvent;


    /**
     * add page if queue don't contain it
     * @param id
     * @param props
     */
    add_page: (id: string, props: ModalPageState) => PageType

    /**
     * remove page and don't check data
     * @param id
     */
    remove_page_unchecked: (id: string) => void
    /**
     * remove page if store don't have unsaved data
     * @param id
     */
    remove_page: (id: string) => RemoveEvent


    /**
     * write data of page
     * @param page
     * @param field
     * @param value
     */
    write_unsaved_data: (page: string, field: string, value: IValExtended) => void
    set_unsaved_data: (page: string, obj: Record<string, IValExtended>) => void

    /**
     * save unsaved data of needed page
     * @param page
     */
    save_data: (page: string) => Promise<WriteEvent>

    /**
     * append save function to buffer
     * @param page
     * @param fn
     */
    subscribe_on_save: (page: string, fn: SaveCallback) => [boolean, (() => void)]

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

    /**
     * show needed page
     * @param page - page id
     */
    show: (page: string) => void

    /**
     * hides current page and don't delete it
     */
    hide: () => void

    /**
     * id of page that need to be shown
     */
    current: string | null
}

export const modalPageStore = create<Type>((set, get) => ({
    pages: {},
    current: null,

    set_default_data: (id, value) => {
        let pages = get().pages
        let page = pages[id];
        if (!page) {
            return WriteEvent.NotFound
        }
        page.default_data = {
            ...page.default_data,
            ...value
        }
        pages[id] = {...page}
        set({
            pages: {...pages}
        })
        return WriteEvent.Done
    },
    write_default_data: (id, field, value) => {
        let pages = get().pages
        let page = pages[id];
        if (!page) {
            return WriteEvent.NotFound
        }

        page.default_data[field] = value
        pages[id] = {...page}
        set({
            pages: {...pages}
        })
        return WriteEvent.Done
    },

    show: (page) => {
        set({
            current: page
        })
    },
    hide: () => {
        set({
            current: null
        })
    },
    add_page(id: string, props: ModalPageState): PageType {
        let pages = get().pages;
        if (Object.keys(pages).includes(id)) {
            return PageType.Has
        }
        pages[id] = props
        set({
            current: id,
            pages: {...pages}
        })
        return PageType.Done
    },
    has_data(page: string, field: string): boolean {
        let pages = get().pages;
        let page_ = pages[page]
        if (!page_) {
            return false
        }

        return page_.default_data[field] != undefined
    },
    remove_page(id: string): RemoveEvent {
        let pages = get().pages
        let page = pages[id]
        if (!page) {
            return RemoveEvent.NotFound
        }
        if (get().current != null) {
            set({
                current: null
            })
        }
        delete pages[id]
        set({
            pages: {...pages}
        })
        return RemoveEvent.Done;
    },

    remove_page_unchecked(id: string): void {
        let pages = get().pages;
        if (get().current == id) {
            set({
                current: null
            })
        }
        delete pages[id]
        set({
            pages: {...pages},
        })
    },
    async save_data(page: string) {
        let page_ = get().pages[page]
        if (!page_) return WriteEvent.NotFound
        let fn = page_.save_fn;
        try {
            await fn?.();
            return WriteEvent.Done
        } catch {
            return WriteEvent.Error
        }
    },
    subscribe_on_save(page: string, fn): [boolean, () => void] {
        let pages = get().pages
        let p = pages[page]
        if (!p) return [false, () => {
        }]
        p.save_fn = fn
        pages[page] = {...p}
        set({
            pages: {...pages}
        })
        return [true, () => {
            modalPageStore.getState().unsubscribe_on_save(page)
        }]
    },
    write_unsaved_data(page: string, field: string, value: IVal): void {
        let pages = get().pages;
        let p = pages[page]
        if (!p) return
        let data = p.unsaved_data;
        data = {
            ...data,
            [field]: value
        }
        p.unsaved_data = {...data}
        pages[page] = {...p}
        set({
            pages: {...pages}

        })
    },
    set_unsaved_data(page: string, obj: Record<string, IVal>): void {
        let pages = get().pages;
        let p = pages[page]
        if (!p) return
        p.unsaved_data = {...obj}
        pages[page] = {...p}
        set({
            pages: {...pages}
        })
    },


    unsubscribe_on_save(page: string): void {
        let pages = get().pages;
        let p = pages[page]
        if (!p) return
        p.save_fn = undefined
        pages[page] = {...p}
        set({
            pages: {...pages}
        })
    }

}))







