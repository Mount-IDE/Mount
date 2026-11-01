import {ModalApi, ModalApiExtended, modalPageStore, RemoveEvent} from "../stores/modal_page_store.ts";


class ModalPageService implements ModalApi {
    get_saved(page: string, field: string) {
        let pages = modalPageStore.getState().pages
        let p = pages[page]
        if (!p) return null
        let data = p.default_data[field] as IVal | undefined
        return data ?? null
    }

    get_unsaved(page: string, field: string) {
        let pages = modalPageStore.getState().pages
        let p = pages[page]
        if (!p) return null
        let data = p.unsaved_data?.[field] as IVal | undefined
        return data ?? null
    }

    async save(page: string) {
        return modalPageStore.getState().save_data(page)
    }

    write_unsaved(page: string, field: string, value: IVal) {
        if (modalPageStore.getState().pages[page] === undefined) return false
        modalPageStore.getState().write_unsaved_data(page, field, value)
        return true
    }

    write_unsaved_obj(page: string, obj: Record<string, IVal>) {
        if (modalPageStore.getState().pages[page] === undefined) return false
        modalPageStore.getState().set_unsaved_data(page, obj)
        return true
    }

}


export const modalPageService = new ModalPageService();


class ModalPageServiceExtended extends ModalPageService implements ModalApiExtended {
    hide(): void {
        modalPageStore.getState().hide()
    }

    remove_current_save() {
        let cur = modalPageStore.getState().current
        if (cur != null) {
            return modalPageStore.getState().remove_page(cur!)
        }
        return RemoveEvent.NotFound
    }

    remove_current_unsaved() {
        let cur = modalPageStore.getState().current
        if (cur != null) {
            modalPageStore.getState().remove_page_unchecked(cur!)
        }

    }

}

export const modalPageExtendedService = new ModalPageServiceExtended()