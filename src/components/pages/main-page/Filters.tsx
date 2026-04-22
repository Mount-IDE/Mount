import "./styles/filters.css"
import search from "../../../assets/search.svg"
import filters from "../../../assets/filters.svg"



export default function Filters(){

    return (
        <div id={"filters"}>
            <div id={"search-filter"}>
                <div>
                    <img src={search}/>
                </div>
                <input placeholder={"Search projects"}/>
            </div>
            <div id={"filters-filters"}>
                <img src={filters}/>
            </div>
        </div>
    )
}
