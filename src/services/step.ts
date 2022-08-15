import { readable } from "svelte/store"

export default readable(1, (set) => {
    var pushState = history.pushState;
    var replaceState = history.replaceState;

    history.pushState = function () {
        pushState.apply(history, arguments);
        window.dispatchEvent(new Event('pushstate'));
        window.dispatchEvent(new Event('locationchange'));
    };

    history.replaceState = function () {
        replaceState.apply(history, arguments);
        window.dispatchEvent(new Event('replacestate'));
        window.dispatchEvent(new Event('locationchange'));
    };

    window.addEventListener('popstate', function () {
        window.dispatchEvent(new Event('locationchange'))
    });


    window.addEventListener('locationchange', function () {
        switch (window.location.pathname) {
            case "/source":
                set(2); break
            case "/datasets":
                set(3); break
            default:
                set(1); break
        }
    })

})
