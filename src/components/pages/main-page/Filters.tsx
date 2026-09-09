import "./styles/filters.css"
import search from "../../../assets/search.svg"
import filters from "../../../assets/filters.svg"
import {mainPageStore} from "../../../stores/main_page_store.ts";
import {filterStore} from "../../../stores/filter_store.ts";


export default function Filters() {

    let str = mainPageStore(state => state.filter_string)

    return (
        <div id={"filters"}>
            <div id={"search-filter"}>
                <div>
                    <img src={search}/>
                </div>
                <input value={str}
                       onInput={(e) =>
                           mainPageStore
                               .getState()
                               .set_filter_string(e.currentTarget.value)}
                       placeholder={"Search projects"}
                />
            </div>
            <div id={"filters-filters"} onClick={() => filterStore.getState().set_opened()}>
                <img src={filters}/>
            </div>
        </div>
    )
}
