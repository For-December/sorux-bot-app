import { defineStore } from 'pinia';

export const useUserStore = defineStore('userStore', {
    state: () => ({
        isLoggedIn: false
    }),
    actions: {
        login() {
            this.isLoggedIn = true;
        },
        logout() {
            this.isLoggedIn = false;
        }
    }
});
