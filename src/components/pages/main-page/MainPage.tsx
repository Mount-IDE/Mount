import "./styles/main-page.css"
import Button from "../../common/Button.tsx";


export default function MainPage() {

    const buttons = [
        {
            title: "New Project",
            cb: () => {}
        }, {
            title: "Open Project",
            cb: () => {}
        }, {
            title: "Import from VCS",
            cb: () => {}
        }, {
            title: "Connect to",
            cb: () => {}
        }
    ]

    return (
        <div className={"page"} id={"main-page"}>
            <div id={"main-page-left"}>
                <div id={"main-page-logo"}>
                    <div id={"main-page-logo-logo"}>
                        <img/>
                    </div>
                    <p>Welcome to Mount!</p>
                </div>
                <div id={"main-page-left-buttons"}>
                    {buttons.map((el, i)=>
                    <Button {...el} key={i}/>
                    )}
                </div>
            </div>
            <div id={"main-page-right"}>

            </div>
        </div>
    )
}