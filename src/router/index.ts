import {createRouter, createWebHashHistory, RouteRecordRaw} from "vue-router";
import {useUserStore} from "@/store/userStore.ts";
import {
    Menu as IconMenu,
    Setting,
} from '@element-plus/icons-vue'

export const Layout = () => import('@/layout/BasicLayout.vue');

export const routes: Readonly<RouteRecordRaw[]> = [
    {
        path: '/login',
        component: () => import('@/pages/Login.vue'),
        meta: {hidden: true}
    },
    {
        path: '/',
        component: Layout,
        redirect: '/home',
        children: [
            {
                path: '/home',
                component: () => import('@/pages/Home.vue'),
                name: "主页",
                meta: {title: "主页", icon: IconMenu}
            },
            {
                path: '/plugin-list',
                component: () => import('@/pages/PluginList.vue'),
                name: "插件列表",
                meta: {title: "插件列表", icon: Setting}
            }
        ],
        meta: {isAuth: true, title: "管理页面"}
    }
]

// 3. 创建路由实例并传递 `routes` 配置
// 你可以在这里输入更多的配置，但我们在这里
// 暂时保持简单
const router = createRouter({
    // 4. 内部提供了 history 模式的实现。为了简单起见，我们在这里使用 hash 模式。
    history: createWebHashHistory(),
    routes, // `routes: routes` 的缩写
})

//全局前置路由守卫————初始化的时候被调用、每次路由切换之前被调用
router.beforeEach((to, _, next) => {
    //如果路由需要跳转
    if (to.meta.isAuth) {
        //判断 如果school本地存储是qinghuadaxue的时候，可以进去
        const userStore = useUserStore();
        if (userStore.isLoggedIn) {
            next()  //放行
        } else {
            next({
                path: '/login',
                // 将跳转的路由path作为参数，登录成功后跳转到该路由
                query: {redirect: to.fullPath}
            })

        }
    } else {
        // 否则，放行
        next()
    }
})

export default router;