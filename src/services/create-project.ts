import {createProjectStore} from "../stores/create_project.ts";
import pageStore from "../stores/page_store.ts";

export function open_project(){
    pageStore.getState().setFilter(true);
    createProjectStore.getState().open();
}

export function close_project(){
    pageStore.getState().setFilter(false);
    createProjectStore.getState().close();
}