import {defineStore} from 'pinia';
import router from "@/router";

export const useUserStore = defineStore('userStore', {
    state: () => ({
        isLoggedIn: false
    }),
    actions: {
        login() {

            this.isLoggedIn = true;
            router.push("/").then(r => {
                    console.log("登陆成功，导航完毕", r)
                }
            )
        },
        logout() {
            this.isLoggedIn = false;
        }
    }
});
