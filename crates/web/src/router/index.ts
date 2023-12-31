import { createRouter, createWebHashHistory } from "vue-router"


const user = new Map<string, any>()

export default createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "/",
            redirect: { name: "home" },
        },
        {
            path: "/home",
            name: "home",
            component: () => import("src/view/Home/Home.vue")
        }
    ]
})
