import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import router from "@/router";

import * as ElementPlusIconsVue from '@element-plus/icons-vue'

import { createPinia } from 'pinia';
import 'uno.css'

const app = createApp(App)

// element plus 图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}

app.use(router)
    .use(createPinia())
    .mount('#app')
